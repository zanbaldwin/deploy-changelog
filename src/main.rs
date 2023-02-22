mod dto;
mod ext;
mod init;
mod models;

use crate::ext::{DeploymentExtension, ListDeploymentsQueryParameters};
use clap::Parser;
use ext::{ComparisonExtension, PageQueryParameters};
use octocrab::Octocrab;
use regex::Regex;
use std::env;

const ERROR_INIT: i32 = 1;
const ERROR_API: i32 = 2;

const DEFAULT_BRANCH: &str = "main";
const PR_COMMIT_REGEX: &str =
    "Merge pull request #(?P<id>[0-9]+) from (?P<branch>[^\\n]+)(?:\\s+(?P<title>.*))?$";

struct Application {
    repository: String,
    github: Octocrab,
    matcher: Regex,
}
impl Application {
    /// Initialise an application
    fn new(repository: String, token: String) -> Result<Self, &'static str> {
        Ok(Self {
            repository,
            github: init::setup_octocrab_client(token)?,
            matcher: Regex::new(PR_COMMIT_REGEX)
                .map_err(|_| "Could not construct matcher from regular expression.")?,
        })
    }

    /// Fetch the last deployment.
    ///
    /// If this is being run as one of the steps in a deployment, then the second to last deployment
    /// is selected (skipping the deployment that is being run right now).
    async fn get_deployment(
        &self,
        event: Option<String>,
    ) -> Result<models::Deployment, &'static str> {
        let index = if event == Some("deployment".to_string()) {
            1
        } else {
            0
        };

        let deployment = self
            .github
            .list_deployments(&self.repository, ListDeploymentsQueryParameters::new())
            .await
            .map_err(|_| "Could not retrieve deployments from GitHub API.")?
            .into_iter()
            .nth(index)
            .ok_or("Previous deployment does not exist.")?;
        Ok(deployment)
    }

    /// Query GitHub for the comparison between to commit "ish" references.
    /// Filter out the noise, and return the full list of commits (collecting all paginated results).
    async fn get_commits_since(
        &self,
        base: String,
        head: String,
    ) -> Result<Vec<models::Commit>, &'static str> {
        // TODO: Page<Comparison> doesn't work because the API response is not an array of objects,
        //       but an object in which an attribute of that object is an array of objects that get
        //       paginated over.
        let comparison = self
            .github
            .compare(&self.repository, &base, &head, PageQueryParameters::new())
            .await
            .map_err(|_| "Cannot fetch commits since last deployment from GitHub API.")?;

        let commits: Vec<models::Commit> = Vec::new();

        let mut page = Some(comparison);
        while let Some(current) = page {
            let items = current.items;
            println!("{items:?}");

            page = self
                .github
                .get_page(&current.next)
                .await
                .map_err(|_| "Failed to fetch the complete list of commits since last deployment from GitHub API.")?;
        }

        Ok(commits)
    }
}

/// It's a main function. What do you expect?
/// I'm only putting this comment in here because it breaks up the functions.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = dotenv::dotenv();
    let init::Cli {
        command,
        repo,
        token,
        head,
    } = init::Cli::parse();

    let app: Application = match Application::new(repo, token) {
        Ok(app) => app,
        Err(message) => init::error(message, ERROR_INIT),
    };

    let deployment = match app.get_deployment(env::var("GITHUB_EVENT_NAME").ok()).await {
        Ok(deployment) => deployment,
        Err(message) => init::error(message, ERROR_API),
    };

    let commits = match app.get_commits_since(deployment.sha, head).await {
        Ok(commits) => commits,
        Err(message) => init::error(message, ERROR_API),
    };

    let info = String::from("???");
    let changelog = dto::Changelog::new(&app.matcher, &app.repository, info, commits);
}
