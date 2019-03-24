use crate::domain::{
    installation::Installation, label::Label, pull_request::PullRequest, user::User,
};
use rocket_contrib::json::Json;

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct PullRequestOpenCommand {
    pub action: String,
    pub number: u64,
    pub pull_request: PullRequest,
    pub label: Option<Label>,
    pub sender: User,
    pub installation: Installation,
}

impl PullRequestOpenCommand {
    pub fn new(data: Json<Self>) -> Self {
        Self {
            action: data.action.clone(),
            number: data.number.clone(),
            pull_request: data.pull_request.clone(),
            label: data.label.clone(),
            sender: data.sender.clone(),
            installation: data.installation.clone(),
        }
    }
}
