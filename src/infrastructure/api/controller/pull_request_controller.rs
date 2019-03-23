use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::application::command::pull_request_open_command::PullRequestOpenCommand;
use crate::application::command::pull_request_open_handler::PullRequestOpenCommandHandler;

#[post("/pull-request/open", format = "application/json", data = "<data>")]
pub fn open(data: Json<PullRequestOpenCommand>) -> Status {
    let command = PullRequestOpenCommand::new(data);
    PullRequestOpenCommandHandler::new().handle(command);
    Status::Ok
}