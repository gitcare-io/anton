use crate::write_schema::events;
use serde_json;

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "events"]
pub struct Event {
    aggregate_id: i64,
    data: serde_json::Value,
    type_: String,
    meta: serde_json::Value,
}

impl Event {
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

// TODO: different events will have different meta
#[derive(Serialize, Deserialize)]
pub struct EventMeta {
    pub user_id: u64,
    pub repo_id: u64,
}
