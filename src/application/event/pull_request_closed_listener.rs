use crate::application::event;
use crate::application::projection::dmr::dmr_projector::DMRProjector;
use crate::application::projection::projector::Projector;
use crate::infrastructure::models::event_store::event::{EventInsertable, EventPullRequestMeta};
use crate::infrastructure::repository::dmr_projection_repository::DMRProjectionRepository;
use crate::infrastructure::repository::event_repository::EventRepository;
use crate::infrastructure::repository::repository::Repository;
use serde_json::json;

pub struct PullRequestClosedListener;

impl PullRequestClosedListener {
    pub fn execute(event: &mut event::pull_request_closed_event::PullRequestClosedEvent) {
        let event_repository: Repository = EventRepository::new();
        let event_meta = EventPullRequestMeta {
            pull_request_id: event.pull_request.id,
            user_id: event.sender.id,
            repo_id: event.repository.id,
        };

        let seq_num: i64 = event_repository
            .persist_event(EventInsertable::new(
                event.pull_request.id as i64,
                json!(&event),
                String::from(event::PULL_REQUEST_CLOSED),
                json!(&event_meta),
            ))
            .expect(
                &format!(
                    "{}: failed - cannot add to event_store",
                    event::PULL_REQUEST_CLOSED
                )[..],
            );

        Self::generate_projections(seq_num);
    }

    pub fn generate_projections(seq_num: i64) {
        let event_repository: Repository = EventRepository::new();
        let dmr_projection_repository: Repository = DMRProjectionRepository::new();

        DMRProjector::new(&event_repository, &dmr_projection_repository)
            .project(seq_num)
            .expect(&format!("{}: DMR Projector failed", event::PULL_REQUEST_CLOSED)[..]);
    }
}
