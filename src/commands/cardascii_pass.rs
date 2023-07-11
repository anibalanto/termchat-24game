use message_io::network::NetworkController;
use crate::action::{Action, Processing};
use crate::commands::Command;
use crate::encoder::Encoder;
use crate::message::NetMessage;
use crate::state::State;
use crate::util::Result;

pub struct CardasciiPassCommand;

impl Command for CardasciiPassCommand {
    fn name(&self) -> &'static str {
        "pass"
    }

    fn parse_params(&self, params: Vec<String>) -> Result<Box<dyn Action>> {

        match CardasciiPass::new() {
            Ok(action) => Ok(Box::new(action)),
            Err(e) => Err(e),
        }
    }
}

pub struct CardasciiPass {
    encoder: Encoder,
}

impl CardasciiPass {

    pub fn new() -> Result<CardasciiPass> {
        Ok(CardasciiPass { encoder: Encoder::new() })
    }
}

impl Action for CardasciiPass {
    fn process(&mut self, state: &mut State, network: &NetworkController) -> Processing {

        let net_message = NetMessage::CardasciiPass();

        let message = self.encoder.encode(net_message);

        for endpoint in state.all_user_endpoints() {
            network.send(*endpoint, message);
        }
        
        Processing::Completed
    }
}
