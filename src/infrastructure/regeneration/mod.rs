use crate::application::event;
use crate::infrastructure::repository::event_repository::EventRepository;
use crate::infrastructure::repository::repository::Repository;
use rocket::response::status;
use std::thread;

#[post("/regeneration", format = "application/json")]
pub fn regenerate() -> status::Accepted<String> {
    let event_repository: Repository = EventRepository::new();
    thread::spawn(move || {
        let mut offset = 0_i64;
        let limit = 1000_i64;
        loop {
            let events = event_repository
                .find_all(limit.clone(), offset.clone())
                .unwrap();
            for event in events.iter() {
                let event_type = &event.event_type[..];
                println!(
                    "regenerating - event, seq_num: {:?}, event_type: {:?}",
                    event.seq_num, event_type
                );
                match event_type {
                    event::PULL_REQUEST_ASSIGNED => {
                        event::pull_request_assigned_listener::PullRequestAssignedListener::
                generate_projections(event.seq_num);
                    }
                    event::PULL_REQUEST_OPENED => {
                        event::pull_request_opened_listener::PullRequestOpenedListener::
                generate_projections(event.seq_num);
                    }
                    event::PULL_REQUEST_CLOSED => {
                        event::pull_request_closed_listener::PullRequestClosedListener::
                generate_projections(event.seq_num);
                    }
                    _ => {}
                }
            }
            offset += 1000;
            if events.len() == 0 {
                break;
            }
        }
    });
    status::Accepted(Some(format!("")))
}

pub fn regeneration_routes() -> Vec<rocket::Route> { routes![regenerate] }
