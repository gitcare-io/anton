use eventbus::Event;

use crate::domain::{
    installation::Installation, label::Label, pull_request::PullRequest, user::User,
};

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct PullRequestOpenedEvent {
    pub action: String,
    pub number: u64,
    pub pull_request: PullRequest,
    pub label: Option<Label>,
    pub sender: User,
    pub installation: Installation,
}

impl Event for PullRequestOpenedEvent {}
