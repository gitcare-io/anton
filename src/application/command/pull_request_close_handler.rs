use crate::application::command::pull_request_close_command::PullRequestCloseCommand;
use crate::domain::pull_request::PullRequest;

pub struct PullRequestCloseCommandHandler {}

impl PullRequestCloseCommandHandler {
    pub fn handle(command: PullRequestCloseCommand) -> () { PullRequest::close(command); }
}
