use crate::application::event::pull_request_opened_event::PullRequestOpenedEvent;

pub fn execute(event: &mut PullRequestOpenedEvent) -> () {
    println!("event -> pull_request_opened");
}