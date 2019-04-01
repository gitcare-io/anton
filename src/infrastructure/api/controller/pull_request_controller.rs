use crate::application::command::*;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[post("/pull-request/open", format = "application/json", data = "<data>")]
pub fn open(data: Json<pull_request_open_command::PullRequestOpenCommand>) -> Status {
    let command = pull_request_open_command::PullRequestOpenCommand::new(data);
    pull_request_open_handler::PullRequestOpenCommandHandler::handle(command);
    Status::Ok
}

#[post("/pull-request/assign", format = "application/json", data = "<data>")]
pub fn assign(data: Json<pull_request_assign_command::PullRequestAssignCommand>) -> Status {
    let command = pull_request_assign_command::PullRequestAssignCommand::new(data);
    pull_request_assign_handler::PullRequestAssignCommandHandler::handle(command);
    Status::Ok
}

#[post("/pull-request/close", format = "application/json", data = "<data>")]
pub fn close(data: Json<pull_request_close_command::PullRequestCloseCommand>) -> Status {
    let command = pull_request_close_command::PullRequestCloseCommand::new(data);
    pull_request_close_handler::PullRequestCloseCommandHandler::handle(command);
    Status::Ok
}
