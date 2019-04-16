use crate::application::query::dmr_query::DMRQuery;
use crate::infrastructure::api::helpers::ApiResponse;
use crate::infrastructure::repository::dmr_projection_repository::DMRProjectionRepository;
use crate::infrastructure::repository::repository::Repository;
use chrono::NaiveDateTime;

// TODO: pagination
#[get("/q/dmr?<repo_id>&<from>&<to>", format = "application/json")]
pub fn index(repo_id: Option<i64>, from: Option<i64>, to: Option<i64>) -> ApiResponse {
    let dmr_projection_repository: Repository = DMRProjectionRepository::new();
    match (repo_id, from, to) {
        (None, _, _) => ApiResponse::bad_request("Query string parameter missing: 'repo_id'"),
        (_, None, _) => ApiResponse::bad_request("Query string parameter missing: 'from'"),
        (_, _, None) => ApiResponse::bad_request("Query string parameter missing: 'to'"),
        (Some(repo_id), Some(from), Some(to)) => {
            let dmr_query = DMRQuery::new(
                &dmr_projection_repository,
                repo_id,
                NaiveDateTime::from_timestamp(from, 0),
                NaiveDateTime::from_timestamp(to, 0),
            );

            ApiResponse::ok(json!(dmr_query.execute()))
        }
    }
}
