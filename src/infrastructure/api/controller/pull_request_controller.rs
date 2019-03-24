use crate::application::command::{
    pull_request_open_command::PullRequestOpenCommand,
    pull_request_open_handler::PullRequestOpenCommandHandler,
};
use rocket::http::Status;
use rocket_contrib::json::Json;

#[post("/pull-request/open", format = "application/json", data = "<data>")]
pub fn open(data: Json<PullRequestOpenCommand>) -> Status {
    let command = PullRequestOpenCommand::new(data);
    PullRequestOpenCommandHandler::handle(command);
    Status::Ok
}
