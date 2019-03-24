use crate::application::event::{
    pull_request_opened_event::PullRequestOpenedEvent, pull_request_opened_listener,
};
use crate::infrastructure::event_bus::EVENT_BUS;

#[allow(deprecated)]
pub fn register_events() {
    register_hook!(
        &EVENT_BUS,
        0,
        PullRequestOpenedEvent,
        pull_request_opened_listener::execute
    );
}
