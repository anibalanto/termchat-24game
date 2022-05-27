use message_io::network::NetworkController;
use crate::action::{Action, Processing};
use crate::commands::Command;
use crate::encoder::Encoder;
use crate::message::NetMessage;
use crate::state::State;
use crate::util::Result;

pub struct CardasciiAnswerCommand;

impl Command for CardasciiAnswerCommand {
    fn name(&self) -> &'static str {
        "answer"
    }

    fn parse_params(&self, params: Vec<String>) -> Result<Box<dyn Action>> {
        let operation = params
            .into_iter()
            .fold(
                "".to_string(),
                |mut cur, nxt| {
                    cur.push_str(format!(" {nxt}").as_str());
                    cur
                });

        match CardasciiAnswer::new(operation.clone()) {
            Ok(action) => Ok(Box::new(action)),
            Err(e) => Err(e),
        }
    }
}

pub struct CardasciiAnswer {
    operation: String,
    encoder: Encoder,
}

impl CardasciiAnswer {

    pub fn new(operation: String) -> Result<CardasciiAnswer> {
        Ok(CardasciiAnswer { operation, encoder: Encoder::new() })
    }
}

impl Action for CardasciiAnswer {
    fn process(&mut self, state: &mut State, network: &NetworkController) -> Processing {

        let net_message = NetMessage::CardasciiAnswer( self.operation.clone() );

        let message = self.encoder.encode(net_message);

        for endpoint in state.all_user_endpoints() {
            network.send(*endpoint, message);
        }


        Processing::Completed
    }
}
