use octocrab::models::repos::RepoCommit;
use octocrab::repos::RepoHandler;
use octocrab::{Page, Result};
use serde;

trait OctocrabDeploymentExtension {
    fn list_deployments(&self) -> ListDeploymentsBuilder<'_, '_>;
}

impl<'octo> OctocrabDeploymentExtension for RepoHandler<'octo> {
    fn list_deployments(&self) -> ListDeploymentsBuilder<'_, '_> {
        ListDeploymentsBuilder::new(self)
    }
}

#[derive(serde::Serialize)]
pub struct ListDeploymentsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sha: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    task: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListDeploymentsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            sha: None,
            reference: None,
            task: None,
            environment: None,
            per_page: None,
            page: None,
        }
    }

    pub async fn send(self) -> Result<Page<RepoCommit>> {
        let url = format!(
            "repos/{owner}/{repo}/commits",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.get(url, Some(&self)).await
    }
}
