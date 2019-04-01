use crate::application::command::pull_request_assign_command::PullRequestAssignCommand;
use crate::application::command::pull_request_open_command::PullRequestOpenCommand;
use crate::application::command::pull_request_close_command::PullRequestCloseCommand;
use crate::application::event::pull_request_assigned_event::PullRequestAssignedEvent;
use crate::application::event::pull_request_opened_event::PullRequestOpenedEvent;
use crate::application::event::pull_request_closed_event::PullRequestClosedEvent;
use crate::domain::{base::Base, href::Href, label::Label, user::User};
use crate::infrastructure::event_bus::EVENT_BUS;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PullRequest {
    url: String,
    pub id: u64,
    node_id: String,
    html_url: String,
    diff_url: String,
    patch_url: String,
    issue_url: String,
    number: u64,
    state: String,
    locked: bool,
    title: String,
    user: User,
    body: String,
    created_at: String,
    updated_at: String,
    closed_at: Option<String>,
    merged_at: Option<String>,
    merge_commit_sha: Option<String>,
    assignee: Option<User>,
    assignees: Vec<User>,
    requested_reviewers: Vec<User>,
    // requested_teams: [], // FIXME:
    labels: Vec<Label>,
    // milestone: null, Option<> // FIXME:
    commits_url: String,
    review_comments_url: String,
    review_comment_url: String,
    comments_url: String,
    statuses_url: String,
    head: Base,
    pub base: Base,
    _links: HashMap<String, Href>,
    author_association: String,
    draft: bool,
    merged: bool,
    mergeable: Option<bool>,
    rebaseable: Option<bool>,
    mergeable_state: String,
    merged_by: Option<User>,
    comments: u64,
    review_comments: u64,
    maintainer_can_modify: bool,
    commits: u8,
    additions: u64,
    deletions: u64,
    changed_files: u64,
}

#[allow(deprecated)]
impl PullRequest {
    pub fn open(command: PullRequestOpenCommand) -> () {
        post_event!(&EVENT_BUS, &mut PullRequestOpenedEvent::new(command), PullRequestOpenedEvent);
    }

    pub fn assign(command: PullRequestAssignCommand) -> () {
        post_event!(&EVENT_BUS, &mut PullRequestAssignedEvent::new(command), PullRequestAssignedEvent);
    }

    pub fn close(command: PullRequestCloseCommand) -> () {
        post_event!(&EVENT_BUS, &mut PullRequestClosedEvent::new(command), PullRequestClosedEvent);
    }
}
