use chrono::{DateTime, Utc};
use octocrab::models::{repos::GitUserTime, App};
use serde;
use url::Url;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[non_exhaustive]
pub struct Deployment {
    pub url: String,
    pub id: u64,
    pub node_id: String,
    pub sha: String,
    #[serde(rename = "ref")]
    pub r#ref: String,
    pub task: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_environment: Option<String>,
    pub environment: String,
    pub description: Option<String>,
    pub creator: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub statuses_url: Url,
    pub repository_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transient_environment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_environment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<App>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Comparison {
    pub url: Url,
    pub html_url: Url,
    pub permalink_url: Url,
    pub diff_url: Url,
    pub patch_url: Url,
    pub base_commit: Commit,
    pub merge_base_commit: Commit,
    pub status: String,
    pub ahead_by: i64,
    pub behind_by: i64,
    pub total_commits: i64,
    pub commits: Vec<Commit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<FileDiff>>,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[non_exhaustive]
pub struct Commit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committer: Option<User>,
    pub commit: CommitData,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct CommitData {
    pub url: Url,
    pub author: GitUserTime,
    pub committer: GitUserTime,
    pub message: String,
    pub tree: Tree,
    pub comment_count: u64,
    pub verification: Option<Verification>,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Tree {
    pub url: Url,
    pub sha: String,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Verification {
    pub verified: bool,
    pub reason: VerificationReason,
    pub signature: Option<String>,
    pub payload: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationReason {
    ExpiredKey,
    NotSigningKey,
    #[serde(rename = "gpgverify_error")]
    GpgVerifyError,
    #[serde(rename = "gpgverify_unavailable")]
    GpgVerifyUnavailable,
    Unsigned,
    UnknownSignatureType,
    NoUser,
    UnverifiedEmail,
    BadEmail,
    MalformedSignature,
    Invalid,
    Valid,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Deserialize)]
#[non_exhaustive]
pub struct User {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: Url,
    pub gravatar_id: String,
    pub url: Url,
    pub html_url: Url,
    pub followers_url: Url,
    pub following_url: Url,
    pub gists_url: Url,
    pub starred_url: Url,
    pub subscriptions_url: Url,
    pub organizations_url: Url,
    pub repos_url: Url,
    pub events_url: Url,
    pub received_events_url: Url,
    pub r#type: String,
    pub site_admin: bool,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileDiff {
    pub sha: String,
    pub filename: String,
    pub status: FileDiffStatus,
    pub additions: u64,
    pub deletions: u64,
    pub changes: u64,
    pub blob_url: Url,
    pub raw_url: Url,
    pub contents_url: Url,
    pub patch: Option<String>,
    pub previous_filename: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum FileDiffStatus {
    Added,
    Removed,
    Modified,
    Renamed,
    Copied,
    Changed,
    Unchanged,
}
