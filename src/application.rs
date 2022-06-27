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
use crate::config::{Config, NodeType};
use crate::encoder::{self, Encoder};

use crossterm::event::{Event as TermEvent, KeyCode, KeyEvent, KeyModifiers};

use message_io::{events::{EventReceiver}, node::NodeEvent};
use message_io::network::{NetEvent, Endpoint, Transport};
use message_io::node::{
    self, NodeListener, NodeTask, NodeHandler,
};

use std::{thread::{self, JoinHandle}, io::Stdout, collections::HashMap};
use std::sync::{mpsc, MutexGuard};


use std::{io::{ErrorKind}, sync::{Mutex, Arc}};
use std::net::SocketAddrV4;
use crate::cardascii::core_cards::{Game24, UnusedCardsError};

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
    //node: NodeHandler<Signal>,
    //_task: NodeTask,
    //renderer: Renderer<Stdout>,
    //read_file_ev: ReadFile,
    //_terminal_events: TerminalEventCollector,
    //receiver: EventReceiver<NodeEvent<Signal>>,
}

impl<'a> Application {
    pub fn new(config: Config) -> Application {

        //let mut renderer = Renderer::new(std::io::stdout()).unwrap();


        //let terminal_handler = handler.clone(); // Collect terminal events

        /*let _terminal_events = 
        TerminalEventCollector::new(
            move |term_event|
                match term_event {
                    Ok(event) =>
                        terminal_handler.signals().send(Signal::Terminal(event)),
                    Err(e) =>
                        terminal_handler.signals().send(Signal::Close(Some(e))),
        }).unwrap();*/


        //let (_task, receiver) = listener.enqueue();

        let commands = CommandManager::default().with(SendFileCommand);

        #[cfg(feature = "stream-video")]
        let commands = commands.with(SendStreamCommand).with(StopStreamCommand);

        let commands = commands.with(CardasciiAnswerCommand);
        let mut state = State::default();

        state.game24 = match config.boot {
            true => Some(Game24::new()),
            false => None,
        };

        Application {
            config,
            commands,
            state,
            //node: handler,
            //_task,
            //renderer,
            // Stored because we need its internal thread running until the Application was dropped
            //_terminal_events,
            //receiver,
        }


        /*let _terminal_events = 
        TerminalEventCollector::new(
            move |term_event|{
                match term_event {
                    Ok(event) =>
                    thread::spawn(move || {
                        read_input(app_arc, event)
                    }),
                    Err(e) =>
                        (),//terminal_handler.signals().send(Signal::Close(Some(e))),
                    };
            }
                
                ).unwrap();*/



        
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

        /*thread::spawn(move || {
            let self_clone = Arc::clone(&self_arc);
            loop{

                thread::sleep(time::Duration::from_millis(300));
            }
        });*/
        //let (_, listener) = node::split::<()>();
        //listener.for_each(move |event| match event.network() {

        /*loop {
            match self.receiver.receive() {
                NodeEvent::Network(net_event) => match net_event {
                    NetEvent::Connected(endpoint, _) => {
                        self.log_in_chat(format!("connected! <{endpoint:?}"))
                    }
                    NetEvent::Message(endpoint, message) => match encoder::decode(&message) {
                        Some(net_message) => self.process_network_message(endpoint, net_message),
                        None => return Err("Unknown message received".into()),
                    },
                    NetEvent::Accepted(endpoint, _resource_id) => {
                        self.log_in_chat(format!("accepted! <{endpoint:?}"))
                    }
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
                        };
                    }
                },
            }
            renderer.render(&self.state, &self.config.theme)?;
        }*/
        //Renderer is destroyed here and the terminal is recovered
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

    fn process_network_message(&mut self, endpoint: Endpoint, message: NetMessage, node: & NodeHandler<Signal>, encoder: & mut Encoder) {
        //self.log_in_chat(format!("processing {:?}", message));
        match message {
            NetMessage::HelloServer(user, server_port) => {
               // let server_addr = (endpoint.addr().ip(), server_port);
                if user != self.config.user_name {
                    let message =
                    NetMessage::HelloUser(self.config.user_name.clone());

                    node.network().send(endpoint, encoder.encode(message));
                    self.state.connected_user(endpoint, &user);

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
            NetMessage::CardasciiNewTurn(_) => {}
            NetMessage::CardasciiAnswer(content) => {
                if let Some(user) = self.state.user_name(&endpoint) {
                    let message = ChatMessage::new(
                        user.into(),
                        MessageType::Text(format!("24Game_answer! > {content}")),
                    );
                    self.state.add_message(message);
                    self.righ_the_bell();
                }

                if self.state.game24.is_some() {
                    //let Some(game) = &mut self.state.game24 {
                    let message2 = ChatMessage::new(
                        "Me".to_string(),
                        MessageType::Text(format!("analizing > {content}")),
                    );

                    self.state.add_message(message2);

                    let game = self.state.game24.as_mut().unwrap();
                    let result_message = match game.make_answer(0, content.clone()) {
                        Ok(()) => format!("correct answer!! =_= > {}", content.clone()),

                        Err(UnusedCardsError(msg)) => format!(
                            "isn't correct answer!! =_= > {} > problem: {}",
                            content.clone(),
                            msg
                        ),
                    };

                    for endpoint in self.state.all_user_endpoints() {
                        node.network().send(
                            *endpoint,
                            encoder.encode(NetMessage::UserMessage(result_message.clone())),
                        );
                    }
                }
            }
        }
    }

    fn process_terminal_event(&mut self, term_event: TermEvent, node: & NodeHandler<Signal>, encoder: & mut Encoder) {
        match term_event {
            TermEvent::Mouse(_) => (),
            TermEvent::Resize(_, _) => (),
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
            game.give_cards();
        }
    }

    fn process_action(&mut self, mut action: Box<dyn Action>, node: & NodeHandler<Signal>) {
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


    fn test_server(& mut self, event: NodeEvent<Signal>) {
        match event.network() {
            NetEvent::Connected(_, _) => (), // Only generated at connect() calls.
            NetEvent::Accepted(endpoint, _listener_id) => {
                // Only connection oriented protocols will generate this event
                self.log_in_chat(format!("Client ({}) connected", endpoint.addr()));
            }
            NetEvent::Message(endpoint, input_data) => {
                self.log_in_chat(format!("Message({})", endpoint.addr()));
            }
            NetEvent::Disconnected(endpoint) => {
                // Only connection oriented protocols will generate this event
                self.log_in_chat(format!("Client ({}) disconnected", endpoint.addr()));
            }
        }
    }

    fn test_client(& mut self, event: NodeEvent<Signal>) {
        match event.network() {
            NetEvent::Connected(_, established) => {
                if established {
                    self.log_in_chat(format!("Connected to server at "));
                    self.log_in_chat(format!("Client identified by local port"));
                }
                else {
                    self.log_in_chat(format!("Can not connect to server at"));
                }
            }
            NetEvent::Accepted(_, _) => unreachable!(), // Only generated when a listener accepts
            NetEvent::Message(_, input_data) => {
                self.log_in_chat(format!("Message()"));
            }
            NetEvent::Disconnected(_) => {
                self.log_in_chat(format!("Server is disconnected"));
            }
        }
    }



}

use std::time::Duration;

use crossterm::{event::{read, poll}};

pub fn read_input<'a>(
    _1_app_arc : Arc<Mutex<Application>>,
    _2_encoder_arc : Arc<Mutex<Encoder>>,
    _3_node_arc : Arc<Mutex<NodeHandler<Signal>>>,
    _4_renderer_arc :  Arc<Mutex<Renderer<Stdout>>>) 
    -> Result<bool> {    
        loop {
            if poll(Duration::from_millis(100))? {
                // It's guaranteed that `read` wont block, because `poll` returned
                // `Ok(true)`.
                //let arc = Arc::clone(&arc);
                //let guard = & mut arc.lock().unwrap();

                let app_guard = & mut _1_app_arc.lock().unwrap();
                let encoder_guard = & mut _2_encoder_arc.lock().unwrap();
                let node_guard = & _3_node_arc.lock().unwrap();
                let renderer_guard = & mut _4_renderer_arc.lock().unwrap();

                app_guard.process_terminal_event(read()?, node_guard, encoder_guard);
                renderer_guard.render(&app_guard.state, &app_guard.config.theme);
            } else {
                // Timeout expired, no `Event` is available
            }
        }
}

pub fn read_ws<'a>(
    _1_app_arc : Arc<Mutex<Application>>,
    _2_encoder_arc : Arc<Mutex<Encoder>>,
    _3_node_arc : Arc<Mutex<NodeHandler<Signal>>>,
    _4_renderer_arc :  Arc<Mutex<Renderer<Stdout>>>,
    listener: NodeListener<Signal>) {

        
        listener.for_each(move |event| {

            let app_guard = & mut _1_app_arc.lock().unwrap();
            let encoder_guard = & mut _2_encoder_arc.lock().unwrap();
            let node_guard = & _3_node_arc.lock().unwrap();
            let renderer_guard = & mut _4_renderer_arc.lock().unwrap();


            match event.network() {
                NetEvent::Connected(endpoint, _) => {
                    //app_guard.log_in_chat(format!("connected! <{endpoint:?}"));
                        
                    let message =
                    NetMessage::HelloServer(app_guard.config.user_name.clone(), endpoint.addr().port());

                    node_guard.network().send(endpoint, encoder_guard.encode(message));
                    
                }
                NetEvent::Message(endpoint, message) => match encoder::decode(&message) {
                    Some(net_message) => {
                        app_guard.process_network_message(endpoint, net_message, node_guard, encoder_guard);
                    },
                    None => /*return Err("Unknown message received".into())*/(),
                },
                NetEvent::Accepted(endpoint, _resource_id) => {
                    //app_guard.log_in_chat(format!("accepted! <{endpoint:?}"));
                },
                NetEvent::Disconnected(endpoint) => {
                    app_guard.state.disconnected_user(endpoint);
                    //If the endpoint was sending a stream make sure to close its window
                    app_guard.state.windows.remove(&endpoint);
                    app_guard.righ_the_bell();
                }
            }
            renderer_guard.render(&app_guard.state, &app_guard.config.theme);
        } );
    }

pub fn run_app(config: Config){




    let (node, listener) = node::split();

    
    let _1_app_arc = Arc::new(Mutex::new(Application::new(config)));
    let _2_encoder_arc = Arc::new(Mutex::new(Encoder::new()));
    let _3_node_arc = Arc::new(Mutex::new(node));
    let _4_render_arc =  Arc::new(Mutex::new(Renderer::new(std::io::stdout()).unwrap()));

    /*let (sender, receiver) = 
        mpsc::channel::<Arc<Mutex<Application>>>();
        let sender = sender.clone();*/
       
        
    let app_arc = Arc::clone(& _1_app_arc);
    let encoder_arc = Arc::clone( & _2_encoder_arc);
    let node_arc = Arc::clone( & _3_node_arc);
    {

        let app_guard = & mut app_arc.lock().unwrap();
        let encoder_guard = & mut encoder_arc.lock().unwrap();
        let node_guard = & node_arc.lock().unwrap();

        app_guard.try_new_turn_game24();

        match &app_guard.config.node_type {
            NodeType::Client { server_addr } => {
                let (server_endpoint, server_addr) =
                node_guard.network().connect(Transport::Ws, server_addr.clone()).unwrap();
    
            }
            NodeType::Server { port } => {
                let my_addr = format!("0.0.0.0:{}", port).parse::<SocketAddrV4>().unwrap();
                let (_, _) = node_guard.network().listen(Transport::Ws, my_addr).unwrap();
            }
        }    
    }
    let app_arc = Arc::clone(& _1_app_arc);
    let renderer_arc = Arc::clone(&_4_render_arc);
    {
        let app_guard = & app_arc.lock().unwrap();
        let renderer_guard = & mut renderer_arc.lock().unwrap();
        renderer_guard.render(&app_guard.state, &app_guard.config.theme);
    
        
    }

    let app_arc = Arc::clone(& _1_app_arc);
    let encoder_arc = Arc::clone( & _2_encoder_arc);
    let node_arc = Arc::clone( & _3_node_arc);
    let renderer_arc = Arc::clone(&_4_render_arc);

    let t1 = thread::spawn(move|| {

        read_ws(
            app_arc,
            encoder_arc,
            node_arc, 
            renderer_arc,
            listener);

        
    });



    let app_arc = Arc::clone(& _1_app_arc);
    let encoder_arc = Arc::clone( & _2_encoder_arc);
    let node_arc = Arc::clone( & _3_node_arc);
    let renderer_arc = Arc::clone(&_4_render_arc);

    let t2 = thread::spawn(move|| {
        
        read_input(
            app_arc,
            encoder_arc,
            node_arc,
            renderer_arc,
            );
    });

    /*loop {
        // receive each value and wait between each one
        let reciver_arc  = receiver.recv().unwrap();
        let arc = Arc::clone(&reciver_arc);
        let guard = & mut arc.lock().unwrap();
        renderer.render(&guard.state, &guard.config.theme);
    }*/
    t1.join().unwrap();
    t2.join().unwrap();

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