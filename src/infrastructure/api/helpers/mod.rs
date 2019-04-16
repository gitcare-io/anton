use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket_contrib::json::JsonValue;

#[derive(Debug)]
pub struct ApiResponse {
    pub json: JsonValue,
    pub status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

impl ApiResponse {
    pub fn ok(json: JsonValue) -> Self {
        Self {
            json,
            status: Status::Ok,
        }
    }

    pub fn bad_request(msg: &'static str) -> Self {
        Self {
            json: json!({
                "message": msg,
                "status": 400,
                "name": "Bad Request"
            }),
            status: Status::BadRequest,
        }
    }

    pub fn unprocessable_entity(msg: &'static str) -> Self {
        Self {
            json: json!({
                "message": msg,
                "status": 422,
                "name": "Unprocessable Entity"
            }),
            status: Status::UnprocessableEntity,
        }
    }

    pub fn forbidden(msg: &'static str) -> Self {
        Self {
            json: json!({
                "message": msg,
                "status": 403,
                "name": "Forbidden"
            }),
            status: Status::Forbidden,
        }
    }

    pub fn unauthorized(msg: &'static str) -> Self {
        Self {
            json: json!({
                "message": msg,
                "status": 401,
                "name": "Unauthorized"
            }),
            status: Status::Unauthorized,
        }
    }

    pub fn conflict(msg: &'static str) -> Self {
        Self {
            json: json!({
                "message": msg,
                "status": 409,
                "name": "Conflict"
            }),
            status: Status::Conflict,
        }
    }

    pub fn internal_server_error(msg: &'static str) -> Self {
        Self {
            json: json!({
                "message": msg,
                "status": 500,
                "name": "InternalServerError"
            }),
            status: Status::InternalServerError,
        }
    }
}