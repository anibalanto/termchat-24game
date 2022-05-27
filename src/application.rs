use super::state::{State, CursorMovement, ChatMessage, MessageType, ScrollMovement};
use crate::{
    state::Window,
    terminal_events::{TerminalEventCollector},
};
use crate::renderer::{Renderer};
use crate::action::{Action, Processing};
use crate::commands::{CommandManager};
use crate::message::{NetMessage, Chunk};
use crate::util::{Error, Result, Reportable};
use crate::commands::send_file::{SendFileCommand};
use crate::commands::cardascii_answer::{CardasciiAnswerCommand};
#[cfg(feature = "stream-video")]
use crate::commands::send_stream::{SendStreamCommand, StopStreamCommand};
use crate::config::Config;
use crate::encoder::{self, Encoder};

use crossterm::event::{Event as TermEvent, KeyCode, KeyEvent, KeyModifiers};

use message_io::events::{EventReceiver};
use message_io::network::{Endpoint, Transport};
use message_io::node::{
    self, StoredNodeEvent as NodeEvent, StoredNetEvent as NetEvent, NodeTask, NodeHandler,
};

use std::io::{ErrorKind};
use crate::cardascii::core_cards::{Game24, UnusedCardsError};

pub enum Signal {
    Terminal(TermEvent),
    Action(Box<dyn Action>),
    // Close event with an optional error in case of failure
    // Close(None) means no error happened
    Close(Option<Error>),
}

pub struct Application<'a> {
    config: &'a Config,
    commands: CommandManager,
    state: State,
    node: NodeHandler<Signal>,
    _task: NodeTask,
    //read_file_ev: ReadFile,
    _terminal_events: TerminalEventCollector,
    receiver: EventReceiver<NodeEvent<Signal>>,
    encoder: Encoder,
}

impl<'a> Application<'a> {
    pub fn new(config: &'a Config) -> Result<Application<'a>> {
        let (handler, listener) = node::split();

        let terminal_handler = 
            handler.clone(); // Collect terminal events

        let _terminal_events = 
            TerminalEventCollector::new(
                move |term_event| 
                    match term_event {
                        Ok(event) => 
                            terminal_handler
                                .signals()
                                .send(Signal::Terminal(event)),

                        Err(e) => 
                            terminal_handler
                                .signals()
                                .send(Signal::Close(Some(e))),
        })?;

        let (_task, receiver) = 
            listener.enqueue();

        let commands = 
            CommandManager::default().with(SendFileCommand);
        #[cfg(feature = "stream-video")]
        let commands = commands.with(SendStreamCommand).with(StopStreamCommand);

        let commands = commands.with(CardasciiAnswerCommand);
        let mut state = State::default();


        state.game24 = match config.boot {
            true => Some(Game24::new()),
            false => None
        };

        Ok(
            Application {
                config,
                commands,
                state,
                node: handler,
                _task,
                // Stored because we need its internal thread running until the Application was dropped
                _terminal_events,
                receiver,
                encoder: Encoder::new(),
            }
        )
    }

    pub fn run(&mut self, out: impl std::io::Write) -> Result<()> {

        self.try_new_turn_game24();
        let mut renderer = Renderer::new(out)?;
        renderer.render(&self.state, &self.config.theme)?;

        let server_addr = ("0.0.0.0", self.config.tcp_server_port);
        let (_, server_addr) = self.node.network().listen(Transport::FramedTcp, server_addr)?;
        self.node.network().listen(Transport::Udp, self.config.discovery_addr)?;

        let (discovery_endpoint, _) =
            self.node.network().connect_sync(Transport::Udp, self.config.discovery_addr)?;
        let message = NetMessage::HelloLan(self.config.user_name.clone(), server_addr.port());
        self.node.network().send(discovery_endpoint, self.encoder.encode(message));

        loop {
            match self.receiver.receive() {
                NodeEvent::Network(net_event) => match net_event {
                    NetEvent::Connected(_, _) => { /* handler in the connect call*/ }
                    NetEvent::Message(endpoint, message) => match encoder::decode(&message) {
                        Some(net_message) => self.process_network_message(endpoint, net_message),
                        None => return Err("Unknown message received".into()),
                    },
                    NetEvent::Accepted(_endpoint, _resource_id) => (),
                    NetEvent::Disconnected(endpoint) => {
                        self.state.disconnected_user(endpoint);
                        //If the endpoint was sending a stream make sure to close its window
                        self.state.windows.remove(&endpoint);
                        self.righ_the_bell();
                    }
                },
                NodeEvent::Signal(signal) => match signal {
                    Signal::Terminal(term_event) => {
                        self.process_terminal_event(term_event);
                    }
                    Signal::Action(action) => {
                        self.process_action(action);
                    }
                    Signal::Close(error) => {
                        self.node.stop();
                        return match error {
                            Some(error) => Err(error),
                            None => Ok(()),
                        }
                    }
                },
            }
            renderer.render(&self.state, &self.config.theme)?;
        }
        //Renderer is destroyed here and the terminal is recovered
    }

    fn process_network_message(&mut self, endpoint: Endpoint, message: NetMessage) {
        match message {
            // by udp (multicast):
            NetMessage::HelloLan(user, server_port) => {
                let server_addr = (endpoint.addr().ip(), server_port);
                if user != self.config.user_name {
                    let mut try_connect = || -> Result<()> {
                        let (user_endpoint, _) =
                            self.node.network().connect_sync(Transport::FramedTcp, server_addr)?;
                        let message = NetMessage::HelloUser(self.config.user_name.clone());
                        self.node.network().send(user_endpoint, self.encoder.encode(message));
                        self.state.connected_user(user_endpoint, &user);
                        Ok(())
                    };
                    try_connect().report_if_err(&mut self.state);
                }
            }
            // by tcp:
            NetMessage::HelloUser(user) => {
                self.state.connected_user(endpoint, &user);
                self.righ_the_bell();
            }
            NetMessage::UserMessage(content) => {
                if let Some(user) = self.state.user_name(&endpoint) {
                    let message = ChatMessage::new(
                        user.into(),
                        MessageType::Text(content));
                    self.state.add_message(message);
                    self.righ_the_bell();
                }
            }
            NetMessage::UserData(file_name, chunk) => {
                use std::io::Write;
                if self.state.user_name(&endpoint).is_some() {
                    // safe unwrap due to check
                    let user = self.state.user_name(&endpoint).unwrap().to_owned();

                    match chunk {
                        Chunk::Error => {
                            format!("'{}' had an error while sending '{}'", user, file_name)
                                .report_err(&mut self.state);
                        }
                        Chunk::End => {
                            format!(
                                "Successfully received file '{}' from user '{}'!",
                                file_name, user
                            )
                            .report_info(&mut self.state);
                            self.righ_the_bell();
                        }
                        Chunk::Data(data) => {
                            let try_write = || -> Result<()> {
                                let user_path = std::env::temp_dir().join("termchat").join(&user);
                                match std::fs::create_dir_all(&user_path) {
                                    Ok(_) => (),
                                    Err(ref err) if err.kind() == ErrorKind::AlreadyExists => (),
                                    Err(e) => return Err(e.into()),
                                }

                                let file_path = user_path.join(file_name);
                                std::fs::OpenOptions::new()
                                    .create(true)
                                    .append(true)
                                    .open(file_path)?
                                    .write_all(&data)?;

                                Ok(())
                            };

                            try_write().report_if_err(&mut self.state);
                        }
                    }
                }
            }
            NetMessage::Stream(data) => match data {
                Some((data, width, height)) if data.len() == width * height / 2 => {
                    self.state
                        .windows
                        .entry(endpoint)
                        .or_insert_with(|| Window::new(width, height));
                    self.state.update_window(&endpoint, data, width, height);
                }
                _ => {
                    self.state.windows.remove(&endpoint);
                }
            },
            NetMessage::CardasciiNewTurn(_) => {

            },
            NetMessage::CardasciiAnswer(content) => {
                if let Some(user) = self.state.user_name(&endpoint) {
                    let message = ChatMessage::new(
                        user.into(),
                        MessageType::Text(format!("24Game_answer! > {content}")));
                    self.state.add_message(message);
                    self.righ_the_bell();
                }

                if self.state.game24.is_some(){//let Some(game) = &mut self.state.game24 {
                    let message2 = ChatMessage::new(
                        "Me".to_string(),
                        MessageType::Text(format!("analizing > {content}")));

                    self.state.add_message(message2);

                    let game = self.state.game24.as_mut().unwrap();
                    let result_message = match game.make_answer(0, content.clone()) {

                        Ok(()) => format!(
                            "correct answer!! =_= > {}",
                            content.clone() ),

                        Err(UnusedCardsError(msg)) => format!(
                            "isn't correct answer!! =_= > {} > problem: {}",
                            content.clone(), msg ),
                    };

                    for endpoint in self.state.all_user_endpoints() {
                        self.node.network().send (
                            *endpoint,
                            self.encoder.encode (
                                NetMessage::UserMessage (
                                    result_message.clone()
                                )
                            )
                        );
                    }

                }

            }
        }
    }

    fn process_terminal_event(&mut self, term_event: TermEvent) {
        match term_event {
            TermEvent::Mouse(_) => (),
            TermEvent::Resize(_, _) => (),
            TermEvent::Key(KeyEvent { code, modifiers }) => match code {
                KeyCode::Esc => {
                    self.node.signals().send_with_priority(Signal::Close(None));
                }
                KeyCode::Char(character) => {
                    if character == 'c' && modifiers.contains(KeyModifiers::CONTROL) {
                        self.node.signals().send_with_priority(Signal::Close(None));
                    }
                    else {
                        self.state.input_write(character);
                    }
                }
                KeyCode::Enter => {
                    if let Some(input) = self.state.reset_input() {
                        match self.commands.find_command_action(&input).transpose() {
                            Ok(action) => {
                                let message = ChatMessage::new(
                                    format!("{} (me)", self.config.user_name),
                                    MessageType::Text(input.clone()),
                                );
                                self.state.add_message(message);

                                for endpoint in self.state.all_user_endpoints() {
                                    self.node.network().send(
                                        *endpoint,
                                        self.encoder.encode(NetMessage::UserMessage(input.clone())),
                                    );
                                }

                                match action {
                                    Some(action) => self.process_action(action),
                                    None => {
                                        if input.starts_with('?') {
                                            String::from("This command doesn't exists")
                                                .report_err(&mut self.state);
                                        }
                                    }
                                }
                            }
                            Err(error) => {
                                error.report_err(&mut self.state);
                            }
                        };
                    }
                }
                KeyCode::Delete => {
                    self.state.input_remove();
                }
                KeyCode::Backspace => {
                    self.state.input_remove_previous();
                }
                KeyCode::Left => {
                    self.state.input_move_cursor(CursorMovement::Left);
                }
                KeyCode::Right => {
                    self.state.input_move_cursor(CursorMovement::Right);
                }
                KeyCode::Home => {
                    self.state.input_move_cursor(CursorMovement::Start);
                }
                KeyCode::End => {
                    self.state.input_move_cursor(CursorMovement::End);
                }
                KeyCode::Up => {
                    self.state.messages_scroll(ScrollMovement::Up);
                }
                KeyCode::Down => {
                    self.state.messages_scroll(ScrollMovement::Down);
                }
                KeyCode::PageUp => {
                    self.state.messages_scroll(ScrollMovement::Start);
                }
                _ => (),
            },
        }
    }

    fn try_new_turn_game24(&mut self){
        if let Some(game) = &mut self.state.game24 {
            game.give_cards();
        }
    }

    fn process_action(&mut self, mut action: Box<dyn Action>) {
        match action.process(&mut self.state, self.node.network()) {
            Processing::Completed => (),
            Processing::Partial(delay) => {
                self.node.signals().send_with_timer(Signal::Action(action), delay);
            }
        }
    }

    pub fn node_handler(&self) -> NodeHandler<Signal> {
        self.node.clone()
    }

    pub fn righ_the_bell(&self) {
        if self.config.terminal_bell {
            print!("\x07");
        }
    }

}
