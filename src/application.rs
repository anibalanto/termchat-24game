use super::state::{State, CursorMovement, ChatMessage, MessageType, ScrollMovement};
use crate::cardascii::common::{Card, HandCardData};
use crate::cardascii::terminal::{draw_hand_from_array, draw_hand_from_stack};
use crate::state::Window;
use crate::renderer::{Renderer};
use crate::action::{Action, Processing};
use crate::commands::{CommandManager};
use crate::message::{NetMessage, Chunk};
use crate::util::{Error, Result, Reportable};
use crate::commands::send_file::{SendFileCommand};
use crate::commands::cardascii_answer::{CardasciiAnswerCommand};
#[cfg(feature = "stream-video")]
use crate::commands::send_stream::{SendStreamCommand, StopStreamCommand};
use crate::config::{Config, NodeType};
use crate::encoder::{self, Encoder};

use crossterm::event::{Event as TermEvent, KeyCode, KeyEvent, KeyModifiers};

use std::convert::TryInto;

use message_io::network::{NetEvent, Endpoint, Transport};
use message_io::node::{self, NodeEvent, NodeListener, NodeHandler};

use std::{
    thread::{self},
    io::Stdout
};

use std::{
    io::{ErrorKind},
    sync::{Mutex, Arc},
};
use std::net::SocketAddrV4;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::cardascii::game::{Game24, Game24Err, TurnResult};

pub enum Signal {
    Terminal(TermEvent),
    Action(Box<dyn Action>),
    // Close event with an optional error in case of failure
    // Close(None) means no error happened
    Close(Option<Error>),
}

pub struct Application {
    config: Config,
    commands: CommandManager,
    state: State,
}

impl<'a> Application {
    pub fn new(config: Config) -> Application {
        let commands = CommandManager::default().with(SendFileCommand);

        #[cfg(feature = "stream-video")]
        let commands = commands.with(SendStreamCommand).with(StopStreamCommand);

        let commands = commands.with(CardasciiAnswerCommand).with(CardasciiPassCommand);
        let mut state = State::default();

        state.game24 = match config.boot {
            true => Some(Game24::new()),
            false => None,
        };

        Application { config, commands, state }
    }

    pub fn run(
        &mut self,
        out: impl std::io::Write, /*, self_arc: Arc<& mut Application>*/
    ) -> Result<()> {
        self.try_new_turn_game24();

        let mut renderer = Renderer::new(out)?;

        renderer.render(&self.state, &self.config.theme)?;

        /*match &self.config.node_type {
            NodeType::Client { server_addr } => {
                let (server_endpoint, server_addr) =
                    self.node.network().connect(Transport::Ws, server_addr.clone()).unwrap();

                let message =
                    NetMessage::HelloServer(self.config.user_name.clone(), server_addr.port());

                self.node.network().send(server_endpoint, self.encoder.encode(message));
            }
            NodeType::Server { port } => {
                let my_addr = format!("0.0.0.0:{}", port).parse::<SocketAddrV4>().unwrap();
                let (_, _) = self.node.network().listen(Transport::Ws, my_addr)?;
            }
        }*/

        Ok(())
    }

    /*fn process_net_event(&mut self, net_event: NodeEvent<Signal>) -> Result<()>{
        match net_event {
            NodeEvent::Network(net_event) => match net_event {
                NetEvent::Connected(endpoint, _) => {
                    Ok(self.log_in_chat(format!("connected! <{endpoint:?}")))
                }
                NetEvent::Message(endpoint, message) => match encoder::decode(&message) {
                    Some(net_message) => {
                        self.process_network_message(endpoint, net_message);
                        Ok(())
                    },
                    None => return Err("Unknown message received".into()),
                },
                NetEvent::Accepted(endpoint, _resource_id) => {
                    Ok(self.log_in_chat(format!("accepted! <{endpoint:?}")))
                }
                NetEvent::Disconnected(endpoint) => {
                    self.state.disconnected_user(endpoint);
                    //If the endpoint was sending a stream make sure to close its window
                    self.state.windows.remove(&endpoint);
                    self.righ_the_bell();
                    Ok(())
                }
            },
            NodeEvent::Signal(signal) => match signal {
                Signal::Terminal(term_event) => {
                    self.process_terminal_event(term_event);
                    Ok(())
                },
                Signal::Action(action) => {
                    self.process_action(action);
                    Ok(())
                }
                Signal::Close(error) => {
                    self.node.stop();
                    return match error {
                        Some(error) => Err(error),
                        None => Ok(()),
                    };
                }
            },
        }
    }*/

    fn log_in_chat(&mut self, msg: String) {
        let message = ChatMessage::new("(me)".to_owned(), MessageType::Text(msg));
        self.state.add_message(message);
    }

    fn process_network_message(
        &mut self,
        endpoint: Endpoint,
        message : NetMessage,
        node    : & NodeHandler<Signal>,
        encoder : & mut Encoder,
    ) {
        //self.log_in_chat(format!("processing {:?}", message));
        match message {
            NetMessage::HelloServer(user, _) => {
                // let server_addr = (endpoint.addr().ip(), server_port);
                if user != self.config.user_name {
                    let message = NetMessage::HelloUser(self.config.user_name.clone());

                    node.network().send(endpoint, encoder.encode(message));
                    self.state.connected_user(endpoint, &user);

                    if let Some(game) = & self.state.game24 {
                        node.network().send(endpoint, encoder.encode(
                            NetMessage::CardasciiNewTurn(
                                get_vec_gived_cards(game)
                            )));
                    }
                }
            }
            // by tcp:
            NetMessage::HelloUser(user) => {
                self.state.connected_user(endpoint, &user);
                self.righ_the_bell();
            }
            NetMessage::UserMessage(content) => {
                if let Some(user) = self.state.user_name(&endpoint) {
                    let message = ChatMessage::new(user.into(), MessageType::Text(content));
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
            NetMessage::CardasciiNewTurn(hand) => {
                if self.state.game24.is_none() {
                    self.state.cards = draw_hand_from_array(& hand);
                }
            }
            NetMessage::CardasciiAnswer(content) => {
                let mut opt_user : Option<String> = None;
                if let Some(user) = self.state.user_name(&endpoint) {
                    opt_user = Some( user.clone() );
                    let message = ChatMessage::new(
                        user.into(),
                        MessageType::Text(format!("24Game_answer! > {content}")),
                    );
                    self.state.add_message(message);
                    self.righ_the_bell();
                }

                if let Some (t_user) = opt_user {
                    if self.state.game24.is_some() {
                        //let Some(game) = &mut self.state.game24 {
                        let message2 = ChatMessage::new(
                            "Me".to_string(),
                            MessageType::Text(format!("analizing > {content}")),
                        );
    
                        self.state.add_message(message2);
    
                        let game = self.state.game24.as_mut().unwrap();
                        let result_message = match game.make_answer( & t_user, content.clone()) {
                            Ok( _ ) => {
                                match game.do_give_cards() {
                                    Ok(turn) => {
                                        self.state.cards = draw_hand_from_stack(& turn.visible_cards);
                                        format!("correct answer!! =_= > {}", content.clone())
                                    }
                                    Err(Game24Err(msg)) =>
                                        format!("correct answer!! =_= > {}. {}", content.clone(), msg)
                                }
                            },
                            Err(Game24Err(msg)) =>
                                format!("isn't correct answer!! =_= > {} > problem: {}",content.clone(), msg),
                        };

                        let hand = get_vec_gived_cards(game);
                        for endpoint in self.state.all_user_endpoints() {
                            node.network().send(
                                *endpoint,
                                encoder.encode(
                                    NetMessage::UserMessage(result_message.clone())
                                ),
                            );

                            node.network().send(
                                *endpoint,
                                encoder.encode(
                                    NetMessage::CardasciiNewTurn(
                                        hand
                                    )
                                ),
                            );
                        }
                    }
                }
                
            },
            NetMessage::CardasciiPass() => {
                if let Some(user) = self.state.user_name(&endpoint).cloned() {
                    if let Some(game) = self.state.game24.as_mut() {
                        match game.do_pass(&user) {
                            Ok(turn) => {
                                match turn.result {
                                    TurnResult::Tie         =>
                                        self.log_in_chat(format!("all players passed this turn")),
                                    TurnResult::Gaming      =>
                                        self.log_in_chat(format!("some player passed this turn")),
                                    TurnResult::Winner(win)   =>
                                        self.log_in_chat(format!("have a winner!")),
                                    TurnResult::Abandoned   =>
                                        self.log_in_chat(format!("why!!!")),
                                }

                            }
                            Err(Game24Err(msg)) => self.log_in_chat(msg)
                        }
                    }
                }

            }
        }
    }

    fn process_terminal_event(
        &mut self,
        term_event: TermEvent,
        node: &NodeHandler<Signal>,
        encoder: &mut Encoder,
        renderer: &mut Renderer<Stdout>,
    ) {
        match term_event {
            TermEvent::Mouse(_) => (),
            TermEvent::Resize(_, _) => {
                renderer.render(&self.state, &self.config.theme);
            }
            TermEvent::Key(KeyEvent { code, modifiers }) => match code {
                KeyCode::Esc => {
                    node.signals().send_with_priority(Signal::Close(None));
                }
                KeyCode::Char(character) => {
                    if character == 'c' && modifiers.contains(KeyModifiers::CONTROL) {
                        node.signals().send_with_priority(Signal::Close(None));
                    } else {
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
                                    node.network().send(
                                        *endpoint,
                                        encoder.encode(NetMessage::UserMessage(input.clone())),
                                    );
                                }

                                match action {
                                    Some(action) => self.process_action(action, node),
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

    fn try_new_turn_game24(&mut self) {
        if let Some(game) = &mut self.state.game24 {
            game.do_give_cards();
        }
    }

    fn process_action(&mut self, mut action: Box<dyn Action>, node: &NodeHandler<Signal>) {
        match action.process(&mut self.state, node.network()) {
            Processing::Completed => (),
            Processing::Partial(delay) => {
                node.signals().send_with_timer(Signal::Action(action), delay);
            }
        }
    }

    pub fn node_handler(&self, node: NodeHandler<Signal>) -> NodeHandler<Signal> {
        node.clone()
    }

    pub fn righ_the_bell(&self) {
        if self.config.terminal_bell {
            print!("\x07");
        }
    }
}

use std::time::Duration;

use crossterm::{
    event::{read, poll},
};
use crate::commands::cardascii_pass::CardasciiPassCommand;

pub fn read_input<'a>(
    _1_app_arc: Arc<Mutex<Application>>,
    _2_encoder_arc: Arc<Mutex<Encoder>>,
    _3_node_arc: Arc<Mutex<NodeHandler<Signal>>>,
    _4_renderer_arc: Arc<Mutex<Renderer<Stdout>>>,
) -> Result<bool>{
    if poll(Duration::from_millis(100))? {

        let app_guard = &mut _1_app_arc.lock().unwrap();
        let encoder_guard = &mut _2_encoder_arc.lock().unwrap();
        let node_guard = &_3_node_arc.lock().unwrap();
        let renderer_guard = &mut _4_renderer_arc.lock().unwrap();

        app_guard.process_terminal_event(read()?, node_guard, encoder_guard, renderer_guard);
        renderer_guard.render(&app_guard.state, &app_guard.config.theme);
    }
    Ok(true)
    /* else {
        // Timeout expired, no `Event` is available
    }*/
}

pub fn read_ws<'a>(
    _1_app_arc: Arc<Mutex<Application>>,
    _2_encoder_arc: Arc<Mutex<Encoder>>,
    _3_node_arc: Arc<Mutex<NodeHandler<Signal>>>,
    _4_renderer_arc: Arc<Mutex<Renderer<Stdout>>>,
    listener: NodeListener<Signal>,
) {
    listener.for_each(move |event| {
        if let Ok(app_guard) = &mut _1_app_arc.lock() {
            if let Ok(encoder_guard) = &mut _2_encoder_arc.lock() {
                if let Ok(node_guard) = &_3_node_arc.lock() {
                    if let Ok(renderer_guard) = &mut _4_renderer_arc.lock() {
                        match event.network() {
                            NetEvent::Connected(endpoint, _) => {
                                //app_guard.log_in_chat(format!("connected! <{endpoint:?}"));
                
                                let message = NetMessage::HelloServer(
                                    app_guard.config.user_name.clone(),
                                    endpoint.addr().port(),
                                );
                
                                node_guard.network().send(endpoint, encoder_guard.encode(message));
                            }
                            NetEvent::Message(endpoint, message) => match encoder::decode(&message) {
                                Some(net_message) => {
                                    app_guard.process_network_message(
                                        endpoint,
                                        net_message,
                                        node_guard,
                                        encoder_guard,
                                    );
                                }
                                None =>
                                /*return Err("Unknown message received".into())*/
                                {
                                    ()
                                }
                            },
                            NetEvent::Accepted(_, _resource_id) => {
                                //app_guard.log_in_chat(format!("accepted! <{endpoint:?}"));
                            }
                            NetEvent::Disconnected(endpoint) => {
                                app_guard.state.disconnected_user(endpoint);
                                //If the endpoint was sending a stream make sure to close its window
                                app_guard.state.windows.remove(&endpoint);
                                app_guard.righ_the_bell();
                            }
                        }
                        renderer_guard.render(&app_guard.state, &app_guard.config.theme);
                    }
                }
            }    
        }
    });
}

pub fn run_app(config: Config) {
    let (node, listener) = node::split();

    let _1_app_arc = Arc::new(Mutex::new(Application::new(config)));
    let _2_encoder_arc = Arc::new(Mutex::new(Encoder::new()));
    let _3_node_arc = Arc::new(Mutex::new(node));
    let _4_renderer_arc = Arc::new(Mutex::new(Renderer::new(std::io::stdout()).unwrap()));

    /*let (sender, receiver) =
    mpsc::channel::<Arc<Mutex<Application>>>();
    let sender = sender.clone();*/

    let app_arc = Arc::clone(&_1_app_arc);
    //let encoder_arc = Arc::clone(&_2_encoder_arc);
    let node_arc = Arc::clone(&_3_node_arc);
    {
        let app_guard = &mut app_arc.lock().unwrap();
        //let encoder_guard = &mut encoder_arc.lock().unwrap();
        let node_guard = &node_arc.lock().unwrap();

        app_guard.try_new_turn_game24();

        match &app_guard.config.node_type {
            NodeType::Client { server_addr } => {
                /*let (server_endpoint, server_addr) =*/
                node_guard.network().connect(Transport::Ws, server_addr.clone()).unwrap();
            }
            NodeType::Server { port } => {
                let my_addr = format!("0.0.0.0:{}", port).parse::<SocketAddrV4>().unwrap();
                let (_, _) = node_guard.network().listen(Transport::Ws, my_addr).unwrap();
            }
        }
    }
    let app_arc = Arc::clone(&_1_app_arc);
    let renderer_arc = Arc::clone(&_4_renderer_arc);
    {
        let app_guard = &app_arc.lock().unwrap();
        let renderer_guard = &mut renderer_arc.lock().unwrap();
        renderer_guard.render(&app_guard.state, &app_guard.config.theme);
    }

    let app_arc = Arc::clone(&_1_app_arc);
    let encoder_arc = Arc::clone(&_2_encoder_arc);
    let node_arc = Arc::clone(&_3_node_arc);
    let renderer_arc = Arc::clone(&_4_renderer_arc);

    let t1 = thread::spawn(move || {
        read_ws(app_arc, encoder_arc, node_arc, renderer_arc, listener);
    });

    let app_arc = Arc::clone(&_1_app_arc);
    let encoder_arc = Arc::clone(&_2_encoder_arc);
    let node_arc = Arc::clone(&_3_node_arc);
    let renderer_arc = Arc::clone(&_4_renderer_arc);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let t2 = thread::spawn(move || {
        while running.load(Ordering::SeqCst) {
            if poll(Duration::from_millis(100))? {
                if let Ok(app_guard) = &mut _1_app_arc.lock() {
                    if let Ok(encoder_guard) = &mut _2_encoder_arc.lock() {
                        if let Ok(node_guard) = &_3_node_arc.lock() {
                            if let Ok(renderer_guard) = &mut _4_renderer_arc.lock() {
                                app_guard.process_terminal_event(read()?, node_guard, encoder_guard, renderer_guard);
                                renderer_guard.render(&app_guard.state, &app_guard.config.theme);
                            }
                        }
                    }
                }
            }
        }
        Result::<()>::Ok(())
    });


    t1.join().unwrap();
    t2.join().unwrap();
}

fn get_vec_gived_cards(game: & Game24) -> HandCardData {
    game.get_gived_cards()
        .into_iter()
        .map(|card|  *card)
        .collect::<Vec<Card>>()
        .try_into()
        .unwrap_or_else(|v: Vec<Card>| panic!("Expected a Vec of length {} but it was {}", 4, v.len()))

}
/*
struct AppOperation {
    _1_app_arc : Arc<Mutex<Application>>,
    _2_encoder_arc : Arc<Mutex<Encoder>>,
    _3_node_arc : Arc<Mutex<NodeHandler<Signal>>>,
    _4_renderer_arc :  Arc<Mutex<Renderer<Stdout>>>,
    operable: bool,

}

impl AppOperation {
    fn new() -> AppOperation {
        let (node, listener) = node::split();

        let _1_app_arc = Arc::new(Mutex::new(Application::new(config)));
        let _2_encoder_arc = Arc::new(Mutex::new(Encoder::new()));
        let _3_node_arc = Arc::new(Mutex::new(node));
        let _4_renderer_arc =  Arc::new(Mutex::new(Renderer::new(std::io::stdout()).unwrap()));

        AppOperation {
            _1_app_arc,
            _2_encoder_arc,
            _3_node_arc,
            _4_renderer_arc,
            operable: false,
        }
    }


}

/* */

impl AppOperation {
    fn cloneArcs(&self) -> Self{
        let _1_app_arc = Arc::clone(& self._1_app_arc);
        let _2_encoder_arc = Arc::clone( & self._2_encoder_arc);
        let _3_node_arc = Arc::clone( & self._3_node_arc);
        let _4_renderer_arc = Arc::clone(& self._4_renderer_arc);
        AppOperation {
            _1_app_arc,
            _2_encoder_arc,
            _3_node_arc,
            _4_renderer_arc,
            operable: true
        }
    }

    fn operate(
        &mut self,
        f: &dyn Fn(
            & mut Application,
            & NodeHandler<Signal>,
            & mut Encoder,
            & mut Renderer<Stdout>) -> Result<()>
        ) {
            let app_guard = & mut self._1_app_arc.lock().unwrap();
            let encoder_guard = & mut self._2_encoder_arc.lock().unwrap();
            let node_guard = & self._3_node_arc.lock().unwrap();
            let renderer_guard = & mut self._4_renderer_arc.lock().unwrap();
            f(app_guard, encoder_guard, node_guard)
        }
}
*/
/*fn read_input<'a>(arc: Arc<Mutex<& mut Application<'a>>>, term_event: TermEvent) {
    let arc = Arc::clone(&arc);
    let guard = & mut arc.lock().unwrap();
    guard.process_terminetwork_eventnal_event(term_event);
}*/
