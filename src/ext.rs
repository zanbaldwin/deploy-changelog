//! Octocrab Extensions

use crate::models::{Comparison, Deployment};
use octocrab::{Octocrab, Page, Result};
use serde;

#[derive(serde::Serialize)]
pub struct ListDeploymentsQueryParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}
impl ListDeploymentsQueryParameters {
    pub fn new() -> Self {
        Self {
            sha: None,
            r#ref: None,
            task: None,
            environment: None,
            per_page: None,
            page: None,
        }
    }

    /// SHA or branch to start listing commits from. Default: the repository’s default
    /// branch (usually master/main).
    pub fn sha(mut self, sha: impl Into<String>) -> Self {
        self.sha = Some(sha.into());
        self
    }

    /// Alias for [`ListCommitsBuilder::sha`], setting a branch will replace the SHA or vice versa.
    pub fn r#ref(mut self, reference: impl Into<String>) -> Self {
        self.r#ref = Some(reference.into());
        self
    }

    /// Only commits containing this file path will be returned.
    pub fn task(mut self, task: impl Into<String>) -> Self {
        self.task = Some(task.into());
        self
    }

    /// GitHub login or email address by which to filter by commit author.
    pub fn environment(mut self, environment: impl Into<String>) -> Self {
        self.environment = Some(environment.into());
        self
    }

    /// Results per page (max: 100, default: 30).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch. (default: 1)
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }
}

#[async_trait::async_trait]
pub(crate) trait DeploymentExtension {
    async fn list_deployments(
        &self,
        repo: &str,
        query: ListDeploymentsQueryParameters,
    ) -> Result<Page<Deployment>>;
}
#[async_trait::async_trait]
impl DeploymentExtension for Octocrab {
    async fn list_deployments(
        &self,
        repo: &str,
        query: ListDeploymentsQueryParameters,
    ) -> Result<Page<Deployment>> {
        let url = format!("/repos/{}/deployments", repo);
        println!("{url}");
        self.get(url, Some(&query)).await
    }
}

#[derive(serde::Serialize)]
pub struct PageQueryParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}
impl PageQueryParameters {
    pub fn new() -> Self {
        Self {
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max: 100, default: 30).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch. (default: 1)
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }
}

#[async_trait::async_trait]
pub(crate) trait ComparisonExtension {
    async fn compare(
        &self,
        repo: &str,
        base: &str,
        head: &str,
        query: PageQueryParameters,
    ) -> Result<Page<Comparison>>;
}
#[async_trait::async_trait]
impl ComparisonExtension for Octocrab {
    async fn compare(
        &self,
        repo: &str,
        base: &str,
        head: &str,
        query: PageQueryParameters,
    ) -> Result<Page<Comparison>> {
        let url = format!("/repos/{repo}/compare/{base}...{head}");
        self.get(url, Some(&query)).await
    }
}
