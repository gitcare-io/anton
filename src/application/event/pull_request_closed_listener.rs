use crate::application::event::pull_request_closed_event::PullRequestClosedEvent;
use crate::application::projection::dmr_projection::{DMRProjection, DMRProjectionIdentity};
use crate::infrastructure::models::event_store::event::{EventInsertable, EventPullRequestMeta};
use crate::infrastructure::repository::event_repository::EventRepository;
use crate::infrastructure::repository::dmr_projection_repository::DMRProjectionRepository;
use crate::infrastructure::repository::repository::Repository;
use serde_json::json;

pub fn execute(event: &mut PullRequestClosedEvent) -> () {
    let event_repo: Repository = EventRepository::new();
    let dmr_projection_repo: Repository = DMRProjectionRepository::new();
    let event_meta = EventPullRequestMeta {
        pull_request_id: event.pull_request.id,
        user_id: event.sender.id,
        repo_id: event.repository.id,
    };

    event_repo
        .persist_event(EventInsertable::new(
        event.pull_request.id as i64,
        json!(&event),
        String::from("pull_request_closed"),
        json!(&event_meta),
    ))
        .expect("pull_request_closed event: failed - cannot add to event_store");

    // TODO: let's move this projection generation to the external projector service
    // It should works something similar to:
    // Projector::new("pull_request_closed").project() And in the Projector
    // ...
    // fn project() -> () {
    //     DMRProjection::new(...)
    //         .generate()
    //         .persist()
    // }
    // ...
    DMRProjection::new(
        &event_repo,
        &dmr_projection_repo,
        DMRProjectionIdentity {
            repo_id: event.repository.id,
        },
        String::from("Europe/Warsaw"), // TODO: it should be configurable for each repo.
        10_f32,                        // TODO: it should be configurable for each repo.
    )
    .generate()
    .persist();
}
