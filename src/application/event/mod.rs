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

impl Event {
    pub fn value(&self) -> &'static str {
        match *self {
            Event::PullRequestOpened => "pull_request_opened",
            Event::PullRequestAssigned => "pull_request_assigned",
            Event::PullRequestClosed => "pull_request_closed",
        }
    }
}
