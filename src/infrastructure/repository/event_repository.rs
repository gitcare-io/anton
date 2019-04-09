use crate::infrastructure::schema::event_store_schema::events::dsl::*;
use crate::infrastructure::{
    models::event_store::event::Event,
    models::event_store::event::EventQueryable,
    repository::repository::{CommonRepository, Repository, __construct},
};
use chrono::NaiveDateTime;
use chrono::{Duration, Utc};
use diesel::dsl::sql;
use diesel::result::Error;
#[allow(unused_imports)]
use diesel::Connection;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub trait EventRepository {
    fn new() -> Self;
    fn add(&self, event: Event) -> QueryResult<usize>;
    fn find_by_repo_and_type(
        &self,
        repo_id: u64,
        event_type: &'static str,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> QueryResult<Vec<EventQueryable>>;
}

impl EventRepository for Repository {
    fn new() -> Self { __construct() }

    fn add(&self, event: Event) -> QueryResult<usize> {
        diesel::insert_into(events)
            .values(&event)
            .execute(self.conn())
    }

    fn find_by_repo_and_type(
        &self,
        repo_id: u64,
        event_type: &'static str,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> QueryResult<Vec<EventQueryable>> {
        let query = events
            .filter(type_.eq(event_type))
            .filter(log_date.gt(from))
            .filter(log_date.lt(to))
            .filter(sql(&format!("meta->>'repo_id' = '{}'", repo_id)[..]));

        query.load::<EventQueryable>(self.conn())
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
            let event_to_add = Event::new(1_i64, json.clone(), String::from("test"), json.clone());
            event_repository.add(event_to_add)?;
            let result = events.load::<EventQueryable>(event_repository.conn())?;
            assert_eq!(result.first().unwrap().aggregate_id, 1_i64);
            Ok(())
        });
    }

    #[test]
    fn find_by_repo_and_type() {
        dotenv::dotenv().ok();
        let event_repository: Repository = EventRepository::new();
        event_repository.conn().test_transaction::<_, Error, _>(|| {
            let json: serde_json::Value = serde_json::from_str("{}").unwrap();
            let json_meta: serde_json::Value = serde_json::from_str("{ \"repo_id\": 10 }").unwrap();
            let example_from = (Utc::now() - Duration::seconds(60_i64)).naive_utc();
            let example_to = (Utc::now() + Duration::seconds(60_i64)).naive_utc();
            let event1 = Event::new(1_i64, json.clone(), String::from("repo"), json_meta.clone());
            let event2 = Event::new(2_i64, json.clone(), String::from("repo"), json.clone());
            event_repository.add(event1.clone())?;
            event_repository.add(event1.clone())?;
            event_repository.add(event2.clone())?;
            let result =
                event_repository.find_by_repo_and_type(10_u64, "repo", example_from, example_to)?;
            assert_eq!(result.first().unwrap().meta["repo_id"], 10_u64);
            assert_eq!(result.len(), 2);
            Ok(())
        });
    }
}
