use crate::application::command::{
    pull_request_assign_command::PullRequestAssignCommand,
    pull_request_assign_handler::PullRequestAssignCommandHandler,
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

#[post("/pull-request/assign", format = "application/json", data = "<data>")]
pub fn assign(data: Json<PullRequestAssignCommand>) -> Status {
    let command = PullRequestAssignCommand::new(data);
    PullRequestAssignCommandHandler::handle(command);
    Status::Ok
}
