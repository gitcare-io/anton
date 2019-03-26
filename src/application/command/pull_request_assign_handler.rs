use crate::application::command::pull_request_assign_command::PullRequestAssignCommand;
use crate::domain::pull_request::PullRequest;

pub struct PullRequestAssignCommandHandler {}

impl PullRequestAssignCommandHandler {
    pub fn handle(command: PullRequestAssignCommand) -> () { PullRequest::assign(command); }
}
