use chrono::NaiveDateTime;
use serde_json;

#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct Event {
    pub seq_num: i64,
    pub aggregate_id: i64,
    pub data: serde_json::Value,
    pub type_: String,
    pub meta: serde_json::Value,
    pub log_date: NaiveDateTime,
}
