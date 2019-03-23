use super::shared::installation::Installation;
use super::shared::label::Label;
use super::shared::pull_request::PullRequest;
use super::shared::user::User;

#[allow(dead_code)]
pub struct CreatePullRequest {
    _event: String,
    action: String,
    number: u64,
    pull_request: PullRequest,
    label: Label,
    sender: User,
    installation: Installation,
}
