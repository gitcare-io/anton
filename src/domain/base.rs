use crate::domain::{repo::Repo, user::User};

#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Base {
    label: String,
    r#ref: String,
    sha: String,
    user: User,
    pub repo: Repo,
}
