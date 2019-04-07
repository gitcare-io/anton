use crate::application::event::pull_request_closed_event::PullRequestClosedEvent;
use crate::infrastructure::models::write::event::{Event, EventPullRequestMeta};
use crate::infrastructure::repository::event_repository::EventRepository;
use crate::infrastructure::repository::repository::Repository;
use serde_json::json;

pub fn execute(event: &mut PullRequestClosedEvent) -> () {
    let event_meta = EventPullRequestMeta {
        pull_request_id: event.pull_request.id,
        user_id: event.sender.id,
        repo_id: event.repository.id,
    };

    let event = Event::new(
        event.pull_request.id as i64,
        json!(&event),
        String::from("pull_request_closed"),
        json!(&event_meta),
    );

    let event_repo: Repository = EventRepository::new();
    event_repo.add(event).expect("pull_request_closed failed");
}
