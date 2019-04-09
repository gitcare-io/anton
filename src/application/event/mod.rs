pub mod helpers;
pub mod pull_request_assigned_event;
pub mod pull_request_assigned_listener;
pub mod pull_request_closed_event;
pub mod pull_request_closed_listener;
pub mod pull_request_opened_event;
pub mod pull_request_opened_listener;
pub mod register;

#[derive(PartialEq)]
pub enum Event {
    PullRequestOpened,
    PullRequestAssigned,
    PullRequestClosed,
}
