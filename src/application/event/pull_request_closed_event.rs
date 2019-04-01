use crate::application::command::pull_request_close_command::PullRequestCloseCommand;
use crate::domain::{
    installation::Installation, pull_request::PullRequest, repo::Repo, user::User,
};
use eventbus::Event;

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct PullRequestClosedEvent {
    pub action: String,
    pub number: u64,
    pub pull_request: PullRequest,
    pub repository: Repo,
    pub sender: User,
    pub installation: Installation,
}

impl Event for PullRequestClosedEvent {}

impl PullRequestClosedEvent {
    pub fn new(command: PullRequestCloseCommand) -> Self {
        Self {
            action: command.action,
            number: command.number,
            pull_request: command.pull_request,
            repository: command.repository,
            sender: command.sender,
            installation: command.installation,
        }
    }
}
