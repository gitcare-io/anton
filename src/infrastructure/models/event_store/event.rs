use crate::infrastructure::schema::event_store_schema::events;
use chrono::NaiveDateTime;
use serde_json;

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct EventQueryable {
    pub seq_num: i64,
    pub aggregate_id: i64,
    pub data: serde_json::Value,
    pub type_: String,
    pub meta: serde_json::Value,
    pub log_date: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Clone)]
#[table_name = "events"]
pub struct EventInsertable {
    pub aggregate_id: i64,
    pub data: serde_json::Value,
    pub type_: String,
    pub meta: serde_json::Value,
}

impl EventInsertable {
    pub fn new(
        aggregate_id: i64,
        data: serde_json::Value,
        event_type: String,
        meta: serde_json::Value,
    ) -> Self {
        Self {
            aggregate_id,
            data,
            type_: event_type,
            meta,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EventPullRequestMeta {
    pub pull_request_id: u64,
    pub user_id: u64,
    pub repo_id: u64,
}
