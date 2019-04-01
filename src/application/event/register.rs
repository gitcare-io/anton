use crate::application::event::*;
use crate::infrastructure::event_bus::EVENT_BUS;

#[allow(deprecated)]
pub fn register_events() {
    register_hook!(
        &EVENT_BUS,
        0,
        pull_request_opened_event::PullRequestOpenedEvent,
        pull_request_opened_listener::execute
    );

    register_hook!(
        &EVENT_BUS,
        0,
        pull_request_assigned_event::PullRequestAssignedEvent,
        pull_request_assigned_listener::execute
    );

    register_hook!(
        &EVENT_BUS,
        0,
        pull_request_closed_event::PullRequestClosedEvent,
        pull_request_closed_listener::execute
    );
}
