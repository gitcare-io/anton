use crate::application::event::pull_request_opened_event::PullRequestOpenedEvent;
use crate::infrastructure::models::event_store::event::{EventInsertable, EventPullRequestMeta};
use crate::infrastructure::repository::event_repository::EventRepository;
use crate::infrastructure::repository::repository::Repository;
use serde_json::json;

pub fn execute(event: &mut PullRequestOpenedEvent) -> () {
    let event_meta = EventPullRequestMeta {
        pull_request_id: event.pull_request.id,
        user_id: event.sender.id,
        repo_id: event.repository.id,
    };

    let event = EventInsertable::new(
        event.pull_request.id as i64,
        json!(&event),
        String::from("pull_request_opened"),
        json!(&event_meta),
    );

    let event_repo: Repository = EventRepository::new();
    event_repo
        .persist_event(event)
        .expect("pull_request_opened event: failed - cannot add to event_store");
}
