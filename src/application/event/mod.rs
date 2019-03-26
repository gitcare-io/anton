pub mod pull_request_opened_event;
pub mod pull_request_opened_listener;
pub mod pull_request_assigned_event;
pub mod pull_request_assigned_listener;

#[derive(PartialEq)]
pub enum Event {
    PullRequestOpened,
    PullRequestAssigned,
}