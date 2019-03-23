use super::base::Base;
use super::href::Href;
use super::label::Label;
use super::user::User;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct PullRequest {
    url: String,
    id: u64,
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
    assignee: User,
    assignees: Vec<User>,
    requested_reviewers: Vec<User>,
    // requested_teams: [], // FIXME: ??
    labels: Vec<Label>,
    // milestone: null, Option<> // FIXME:
    commits_url: String,
    review_comments_url: String,
    review_comment_url: String,
    comments_url: String,
    statuses_url: String,
    head: Base,
    base: Base,
    _links: HashMap<String, Href>,
    author_association: String,
    draft: bool,
    merged: bool,
    mergeable: Option<String>,
    rebaseable: Option<String>,
    mergeable_state: String,
    merged_by: Option<String>,
    comments: u64,
    review_comments: u64,
    maintainer_can_modify: bool,
    commits: u8,
    additions: u64,
    deletions: u64,
    changed_files: u64,
}
