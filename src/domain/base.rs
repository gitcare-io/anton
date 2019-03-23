use super::repo::Repo;
use super::user::User;

#[allow(dead_code)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Base {
    label: String,
    r#ref: String,
    sha: String,
    user: User,
    repo: Repo,
}
