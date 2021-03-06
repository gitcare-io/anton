use crate::domain::{
    installation::Installation, pull_request::PullRequest, user::User, repo::Repo,
};
use rocket_contrib::json::Json;

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct PullRequestAssignCommand {
    pub action: String,
    pub number: u64,
    pub pull_request: PullRequest,
    pub repository: Repo,
    pub assignee: User,
    pub sender: User,
    pub installation: Installation,
}

impl PullRequestAssignCommand {
    pub fn new(data: Json<Self>) -> Self {
        Self {
            action: data.action.clone(),
            number: data.number.clone(),
            pull_request: data.pull_request.clone(),
            repository: data.repository.clone(),
            assignee: data.assignee.clone(),
            sender: data.sender.clone(),
            installation: data.installation.clone(),
        }
    }
}
