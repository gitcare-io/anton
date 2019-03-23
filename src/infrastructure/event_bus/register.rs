use super::EVENT_BUS;
use crate::application::event::pull_request_opened_event::PullRequestOpenedEvent;
use crate::application::event::pull_request_opened_listener;

pub fn register_events() {
    register_hook!(&EVENT_BUS, 0, PullRequestOpenedEvent, pull_request_opened_listener::execute);
}
