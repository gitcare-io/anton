use eventbus::Event;
use crate::application::command::pull_request_assign_command::PullRequestAssignCommand;
use crate::domain::{
    installation::Installation, pull_request::PullRequest, user::User, repo::Repo,
};

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct PullRequestAssignedEvent {
    pub action: String,
    pub number: u64,
    pub pull_request: PullRequest,
    pub repository: Repo,
    pub assignee: User,
    pub sender: User,
    pub installation: Installation,
}

impl Event for PullRequestAssignedEvent {}

impl PullRequestAssignedEvent {
    pub fn new(command: PullRequestAssignCommand) -> Self {
        Self {
            action: command.action,
            number: command.number,
            pull_request: command.pull_request,
            assignee: command.assignee,
            repository: command.repository,
            sender: command.sender,
            installation: command.installation,
        }
    }
}
