use crate::infrastructure::{
    models::read::event::Event as ReadEvent, models::write::event::Event as WriteEvent,
    repository::connection_manager,
};
use crate::write_schema::events::dsl::*;
use chrono::NaiveDateTime;
use chrono::{Duration, Utc};
use diesel::result::Error;
#[allow(unused_imports)]
use diesel::Connection;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub struct EventRepository {
    conn: connection_manager::PoolConnection,
}

impl EventRepository {
    pub fn new() -> Self {
        Self {
            conn: connection_manager::get(connection_manager::PoolType::Write),
        }
    }

    pub fn add(&self, event: WriteEvent) -> QueryResult<usize> {
        diesel::insert_into(events)
            .values(&event)
            .execute(&*self.conn)
    }

    pub fn find_in_range(
        &self,
        agg_id: i64,
        event_type: &'static str,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> QueryResult<Vec<ReadEvent>> {
        events
            .filter(aggregate_id.eq(agg_id))
            .filter(type_.eq(event_type))
            .filter(log_date.gt(from))
            .filter(log_date.lt(to))
            .load::<ReadEvent>(&*self.conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;

    #[test]
    fn add_event() {
        dotenv::dotenv().ok();
        let event_repo = EventRepository::new();
        event_repo.conn.test_transaction::<_, Error, _>(|| {
            let json: serde_json::Value = serde_json::from_str("{}").unwrap();
            let event_to_add =
                WriteEvent::new(10_i64, json.clone(), String::from("test"), json.clone());
            event_repo.add(event_to_add)?;
            let result = events.load::<ReadEvent>(&event_repo.conn)?;
            assert_eq!(result.first().unwrap().aggregate_id, 10_i64);
            Ok(())
        });
    }

    #[test]
    fn find_in_range() {
        dotenv::dotenv().ok();
        let event_repo = EventRepository::new();
        event_repo.conn.test_transaction::<_, Error, _>(|| {
            let json: serde_json::Value = serde_json::from_str("{}").unwrap();
            let example_from = (Utc::now() - Duration::seconds(60_i64)).naive_utc();
            let example_to = (Utc::now() + Duration::seconds(60_i64)).naive_utc();
            let event1 = WriteEvent::new(10_i64, json.clone(), String::from("repo"), json.clone());
            let event2 = WriteEvent::new(11_i64, json.clone(), String::from("repo"), json.clone());
            event_repo.add(event1.clone())?;
            event_repo.add(event1.clone())?;
            event_repo.add(event2.clone())?;
            let result = event_repo.find_in_range(10_i64, "repo", example_from, example_to)?;
            assert_eq!(result.first().unwrap().aggregate_id, 10_i64);
            assert_eq!(result.len(), 2);
            Ok(())
        });
    }
}
