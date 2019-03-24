use crate::application::command::pull_request_open_command::PullRequestOpenCommand;
use crate::domain::pull_request::PullRequest;

pub struct PullRequestOpenCommandHandler {}

impl PullRequestOpenCommandHandler {
    pub fn handle(command: PullRequestOpenCommand) -> () { PullRequest::open(command); }
}
