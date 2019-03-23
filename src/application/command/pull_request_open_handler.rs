use rocket_contrib::json::Json;
use crate::application::command::pull_request_open_command::PullRequestOpenCommand;

pub struct PullRequestOpenCommandHandler {
    // event_repository: ORMEventRepository, // TODO: eventRepo
}

impl PullRequestOpenCommandHandler {
    pub fn new() -> Self {
        Self {
            // event_repository: ORMEventRepository::new() // TODO: eventRepo
        }
    }

    pub fn handle(&self, command: (PullRequestOpenCommand, Json<PullRequestOpenCommand>)) -> () {
        println!("{:?}", command.1);
        // TODO: eventRepo
        // command.pull_request
        // added_user.register_user();
    }
}