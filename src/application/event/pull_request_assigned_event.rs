use eventbus::Event;

use crate::domain::{
    installation::Installation, pull_request::PullRequest, user::User,
};

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct PullRequestAssignedEvent {
    pub action: String,
    pub number: u64,
    pub pull_request: PullRequest,
    pub assignee: User,
    pub sender: User,
    pub installation: Installation,
}

impl Event for PullRequestAssignedEvent {}
