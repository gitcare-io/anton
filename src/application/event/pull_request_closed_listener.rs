use crate::application::event::pull_request_closed_event::PullRequestClosedEvent;
use crate::application::event::Event::PullRequestClosed;
use crate::application::projection::dmr_projector::DMRProjector;
use crate::application::projection::projector::Projector;
use crate::infrastructure::models::event_store::event::{EventInsertable, EventPullRequestMeta};
use crate::infrastructure::repository::dmr_projection_repository::DMRProjectionRepository;
use crate::infrastructure::repository::event_repository::EventRepository;
use crate::infrastructure::repository::repository::Repository;
use serde_json::json;

pub fn execute(event: &mut PullRequestClosedEvent) -> () {
    let event_name = PullRequestClosed.value();
    let event_repository: Repository = EventRepository::new();
    let dmr_projection_repository: Repository = DMRProjectionRepository::new();
    let event_meta = EventPullRequestMeta {
        pull_request_id: event.pull_request.id,
        user_id: event.sender.id,
        repo_id: event.repository.id,
    };

    let seq_num: i64 = event_repository
        .persist_event(EventInsertable::new(
            event.pull_request.id as i64,
            json!(&event),
            String::from(event_name),
            json!(&event_meta),
        ))
        .expect(&format!("{}: failed - cannot add to event_store", event_name)[..]);
    
    println!("seq_num of event - {}", seq_num);

    DMRProjector::new(&event_repository, &dmr_projection_repository)
        .project(seq_num)
        .expect(&format!("{}: DMR Projector failed", event_name)[..]);
}
