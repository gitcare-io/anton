use crate::application::event::pull_request_closed_event::PullRequestClosedEvent;
use crate::application::projection::dmr_projection::{DMRProjection, DMRProjectionIdentity};
use crate::infrastructure::models::event_store::event::{Event, EventPullRequestMeta};
use crate::infrastructure::repository::event_repository::EventRepository;
use crate::infrastructure::repository::repository::Repository;
use serde_json::json;

pub fn execute(event: &mut PullRequestClosedEvent) -> () {
    let event_repo: Repository = EventRepository::new();
    let event_meta = EventPullRequestMeta {
        pull_request_id: event.pull_request.id,
        user_id: event.sender.id,
        repo_id: event.repository.id,
    };
    let event_to_persist = Event::new(
        event.pull_request.id as i64,
        json!(&event),
        String::from("pull_request_closed"),
        json!(&event_meta),
    );

    event_repo
        .add(event_to_persist)
        .expect("pull_request_closed event: failed - cannot add to event_store");

    DMRProjection::new(
        &event_repo,
        DMRProjectionIdentity {
            repo_id: event.repository.id,
        },
        String::from("Europe/Warsaw"), // FIXME: it should be configurable for each repo.
        10_f32,                        // FIXME: it should be configurable for each repo.
    )
    .generate()
    .persist();
}
