use crate::infrastructure::schema::read_schema::dmrprojections;
use chrono::NaiveDateTime;
use serde_json;

#[derive(Serialize, Deserialize, Insertable, AsChangeset, Clone)]
#[table_name = "dmrprojections"]
pub struct DMRProjectionInsertable {
    pub id: String,
    pub repo_id: i64,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub data: serde_json::Value,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct DMRProjectionQueryable {
    pub id: String,
    pub repo_id: i64,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub data: serde_json::Value,
    pub projected_at: NaiveDateTime,
}

impl DMRProjectionInsertable {
    pub fn new(
        id: String,
        repo_id: i64,
        from: NaiveDateTime,
        to: NaiveDateTime,
        data: serde_json::Value,
    ) -> Self {
        Self {
            id,
            repo_id,
            from,
            to,
            data,
        }
    }
}
