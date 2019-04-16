use crate::application::query::dmr_query::DMRQuery;
use crate::infrastructure::models::read::dmr_projection::DMRProjectionQueryable;
use crate::infrastructure::repository::dmr_projection_repository::DMRProjectionRepository;
use crate::infrastructure::repository::repository::Repository;
use chrono::NaiveDateTime;
use rocket_contrib::json::Json;

#[get("/q/dmr?<repo_id>&<from>&<to>", format = "application/json")]
pub fn index(repo_id: i64, from: i64, to: i64) -> Json<Vec<DMRProjectionQueryable>> {
    // TODO: error handling
    // TODO: pagination
    let dmr_projection_repository: Repository = DMRProjectionRepository::new();
    let dmr_query = DMRQuery::new(
        &dmr_projection_repository,
        repo_id,
        NaiveDateTime::from_timestamp(from, 0),
        NaiveDateTime::from_timestamp(to, 0),
    );

    let results = dmr_query.execute();
    Json(results)
}
