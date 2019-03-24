use crate::application::event::pull_request_opened_event::PullRequestOpenedEvent;
use crate::infrastructure::models::write::event::{Event, EventMeta};
use crate::infrastructure::repository::event_repository::EventRepository;
use serde_json::json;

pub fn execute(event: &mut PullRequestOpenedEvent) -> () {
    let event_meta = EventMeta {
        user_id: event.sender.id,
        repo_id: event.pull_request.base.repo.id,
    };

    let event = Event::new(
        event.pull_request.id as i64,
        json!(&event_meta),
        String::from("pull_request_opened"),
        json!(&event),
    );

    EventRepository::add(event).expect("pull_request_opend failed");
}
