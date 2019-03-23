use eventbus::Event;

use crate::domain::installation::Installation;
use crate::domain::label::Label;
use crate::domain::pull_request::PullRequest;
use crate::domain::user::User;

#[allow(dead_code)]
pub struct PullRequestOpenedEvent {
    pub action: String,
    pub number: u64,
    pub pull_request: PullRequest,
    pub label: Option<Label>,
    pub sender: User,
    pub installation: Installation,
}

impl Event for PullRequestOpenedEvent {}
