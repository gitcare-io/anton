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
    fn value(&self) -> &'static str {
        match *self {
            Event::PullRequestOpened => "pull_requst_opened",
            Event::PullRequestAssigned => "pull_requst_assigned",
            Event::PullRequestClosed => "pull_requst_closed",
        }
    }
}
