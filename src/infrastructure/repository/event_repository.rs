use crate::infrastructure::{
    models::read::event::Event as ReadEvent,
    models::write::event::Event as WriteEvent,
    repository::repository::{CommonRepository, Repository, __construct},
};
use crate::write_schema::events::dsl::*;
use chrono::NaiveDateTime;
use chrono::{Duration, Utc};
use diesel::result::Error;
#[allow(unused_imports)]
use diesel::Connection;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub trait EventRepository {
    fn new() -> Self;
    fn add(&self, event: WriteEvent) -> QueryResult<usize>;
    fn find_in_range(
        &self,
        agg_id: i64,
        event_type: &'static str,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> QueryResult<Vec<ReadEvent>>;
}

impl EventRepository for Repository {
    fn new() -> Self { __construct() }

    fn add(&self, event: WriteEvent) -> QueryResult<usize> {
        diesel::insert_into(events)
            .values(&event)
            .execute(self.conn())
    }

    fn find_in_range(
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
            .load::<ReadEvent>(self.conn())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;

    #[test]
    fn add_event() {
        dotenv::dotenv().ok();
        let event_repository: Repository = EventRepository::new();
        event_repository.conn().test_transaction::<_, Error, _>(|| {
            let json: serde_json::Value = serde_json::from_str("{}").unwrap();
            let event_to_add =
                WriteEvent::new(10_i64, json.clone(), String::from("test"), json.clone());
            event_repository.add(event_to_add)?;
            let result = events.load::<ReadEvent>(event_repository.conn())?;
            assert_eq!(result.first().unwrap().aggregate_id, 10_i64);
            Ok(())
        });
    }

    #[test]
    fn find_in_range() {
        dotenv::dotenv().ok();
        let event_repository: Repository = EventRepository::new();
        event_repository.conn().test_transaction::<_, Error, _>(|| {
            let json: serde_json::Value = serde_json::from_str("{}").unwrap();
            let example_from = (Utc::now() - Duration::seconds(60_i64)).naive_utc();
            let example_to = (Utc::now() + Duration::seconds(60_i64)).naive_utc();
            let event1 = WriteEvent::new(10_i64, json.clone(), String::from("repo"), json.clone());
            let event2 = WriteEvent::new(11_i64, json.clone(), String::from("repo"), json.clone());
            event_repository.add(event1.clone())?;
            event_repository.add(event1.clone())?;
            event_repository.add(event2.clone())?;
            let result =
                event_repository.find_in_range(10_i64, "repo", example_from, example_to)?;
            assert_eq!(result.first().unwrap().aggregate_id, 10_i64);
            assert_eq!(result.len(), 2);
            Ok(())
        });
    }
}
