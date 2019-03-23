use super::repo::Repo;
use super::user::User;

#[allow(dead_code)]
pub struct Base {
    label: String,
    r#ref: String,
    sha: String,
    user: User,
    repo: Repo,
}
