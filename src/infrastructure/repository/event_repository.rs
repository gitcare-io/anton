use crate::infrastructure::schema::event_store_schema::events::dsl::*;
use crate::infrastructure::{
    models::event_store::event::EventInsertable,
    models::event_store::event::EventQueryable,
    repository::repository::{CommonRepository, Repository, __construct},
};
use chrono::NaiveDateTime;
use diesel::dsl::sql;
#[allow(unused_imports)]
use diesel::Connection;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub trait EventRepository {
    fn new() -> Self;
    fn persist_event(&self, event: EventInsertable) -> QueryResult<(i64)>;
    fn find_by_repo_and_type(
        &self,
        repo_id: u64,
        eventtype: &'static str,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> QueryResult<Vec<EventQueryable>>;
    fn find_by_seq_num(&self, seq_num: i64) -> QueryResult<EventQueryable>;
}

impl EventRepository for Repository {
    fn new() -> Self { __construct("event_store") }

    fn persist_event(&self, event: EventInsertable) -> QueryResult<(i64)> {
        diesel::insert_into(events)
            .values(&event)
            .returning(seq_num)
            .get_result::<(i64)>(self.conn())
    }

    fn find_by_seq_num(&self, seq_n: i64) -> QueryResult<EventQueryable> {
        events.find(seq_n).first(self.conn())
    }

    fn find_by_repo_and_type(
        &self,
        repo_id: u64,
        eventtype: &'static str,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> QueryResult<Vec<EventQueryable>> {
        let query = events
            .filter(event_type.eq(eventtype))
            .filter(log_date.gt(from))
            .filter(log_date.lt(to))
            .filter(sql(&format!("meta->>'repo_id' = '{}'", repo_id)[..]));

        query.load::<EventQueryable>(self.conn())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_config;
    use chrono::{Duration, Utc};
    use diesel::result::Error;

    #[test]
    fn add_event() {
        load_config();
        let event_repository: Repository = EventRepository::new();
        event_repository.conn().test_transaction::<_, Error, _>(|| {
            let json: serde_json::Value = serde_json::from_str("{}").unwrap();
            let event_to_add =
                EventInsertable::new(1_i64, json.clone(), String::from("test"), json.clone());
            event_repository.persist_event(event_to_add)?;
            let result = events.load::<EventQueryable>(event_repository.conn())?;
            assert_eq!(result.first().unwrap().aggregate_id, 1_i64);
            Ok(())
        });
    }

    #[test]
    fn find_by_repo_and_type() {
        load_config();
        let event_repository: Repository = EventRepository::new();
        event_repository.conn().test_transaction::<_, Error, _>(|| {
            let json: serde_json::Value = serde_json::from_str("{}").unwrap();
            let json_meta: serde_json::Value = serde_json::from_str("{ \"repo_id\": 10 }").unwrap();
            let example_from = (Utc::now() - Duration::seconds(60_i64)).naive_utc();
            let example_to = (Utc::now() + Duration::seconds(60_i64)).naive_utc();
            let event1 =
                EventInsertable::new(1_i64, json.clone(), String::from("repo"), json_meta.clone());
            let event2 =
                EventInsertable::new(2_i64, json.clone(), String::from("repo"), json.clone());
            event_repository.persist_event(event1.clone())?;
            event_repository.persist_event(event1.clone())?;
            event_repository.persist_event(event2.clone())?;
            let result =
                event_repository.find_by_repo_and_type(10_u64, "repo", example_from, example_to)?;
            assert_eq!(result.first().unwrap().meta["repo_id"], 10_u64);
            assert_eq!(result.len(), 2);
            Ok(())
        });
    }

    #[test]
    fn find_by_seq_num_test() {
        load_config();
        let event_repository: Repository = EventRepository::new();
        event_repository.conn().test_transaction::<_, Error, _>(|| {
            let event = EventInsertable::new(
                2_i64,
                serde_json::from_str("{}").unwrap(),
                String::from("repo"),
                serde_json::from_str("{}").unwrap(),
            );
            let seq_n = event_repository.persist_event(event.clone())?;
            let found_event = event_repository.find_by_seq_num(seq_n)?;
            assert_eq!(found_event.seq_num, seq_n);
            assert_eq!(found_event.aggregate_id, 2_i64);
            Ok(())
        });
    }
}
