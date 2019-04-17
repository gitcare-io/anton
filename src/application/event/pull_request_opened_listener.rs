use crate::application::event;
use crate::infrastructure::models::event_store::event::{EventInsertable, EventPullRequestMeta};
use crate::infrastructure::repository::event_repository::EventRepository;
use crate::infrastructure::repository::repository::Repository;
use serde_json::json;

pub struct PullRequestOpenedListener;

impl PullRequestOpenedListener {
    pub fn execute(event: &mut event::pull_request_opened_event::PullRequestOpenedEvent) -> () {
        let event_name = event::PULL_REQUEST_OPENED;
        let event_meta = EventPullRequestMeta {
            pull_request_id: event.pull_request.id,
            user_id: event.sender.id,
            repo_id: event.repository.id,
        };

        let event = EventInsertable::new(
            event.pull_request.id as i64,
            json!(&event),
            String::from(event_name),
            json!(&event_meta),
        );

        let event_repo: Repository = EventRepository::new();
        let seq_num = event_repo
            .persist_event(event)
            .expect(&format!("{}: failed - cannot add to event_store", event_name)[..]);

        Self::generate_projections(seq_num);
    }

    pub fn generate_projections(_seq_num: i64) {}
}
