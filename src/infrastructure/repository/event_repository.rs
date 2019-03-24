use crate::infrastructure::{models::write::event::Event, repository::connection_manager};
use crate::write_schema::events::dsl::*;
use diesel::{QueryResult, RunQueryDsl};

#[derive(Debug)]
pub struct EventRepository;

impl EventRepository {
    pub fn add(event: Event) -> QueryResult<usize> {
        diesel::insert_into(events)
            .values(&event)
            .execute(&*connection_manager::get(
                connection_manager::PoolType::Write,
            ))
    }
}
