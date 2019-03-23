use rocket_contrib::json::Json;
use crate::domain::installation::Installation;
use crate::domain::label::Label;
use crate::domain::pull_request::PullRequest;
use crate::domain::user::User;

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct PullRequestOpenCommand {
    pub action: String,
    pub number: u64,
    pub pull_request: PullRequest,
    pub label: Option<Label>,
    pub sender: User,
    pub installation: Installation,
}

impl PullRequestOpenCommand {
    pub fn new(data: Json<Self>) -> (Self, Json<Self>) {
        (
            Self {
                action: data.action.clone(),
                number: data.number.clone(),
                pull_request: data.pull_request.clone(),
                label: data.label.clone(),
                sender: data.sender.clone(),
                installation: data.installation.clone(),
            },
            data
        )
    }
}
