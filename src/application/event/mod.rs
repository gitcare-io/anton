pub mod pull_request_opened_event;
pub mod pull_request_opened_listener;

#[derive(PartialEq)]
pub enum Event {
    PullRequestOpened
}