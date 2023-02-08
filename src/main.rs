mod init;

use octocrab::Octocrab;
use regex::Regex;
use std::{process::exit, sync::Arc};

const PR_COMMIT_REGEX: &str =
    "Merge pull request #(?P<id>[0-9]+) from (?P<branch>[^\\n]+)(?:\\s+(?P<title>.*))?$";

struct PullRequest {
    repository: String,
    id: u32,
    branch: String,
    title: Option<String>,
}
impl PullRequest {
    // Would be nice to implement this in FromStr, but I don't know how to access the
    // matcher from from_str() without initializing it each time.
    fn from_commit_message(
        matcher: &Regex,
        repository: (&str, &str),
        commit_message: &str,
    ) -> Result<Self, ()> {
        let Some(captures) = matcher.captures(commit_message) else {
            return Err(());
        };

        Ok(Self {
            repository: format!("{}/{}", repository.0, repository.1),
            id: captures
                .name("id")
                .ok_or(())?
                .as_str()
                .parse()
                .map_err(|_| ())?,
            branch: captures.name("branch").ok_or(())?.as_str().to_string(),
            title: captures
                .name("title")
                .map(|capture| capture.as_str().to_string()),
        })
    }

    fn get_url(&self) -> String {
        format!("https://github.com/{}/pull/{}", self.repository, self.id)
    }
}

struct Changelog {
    pull_requests: Vec<PullRequest>,
    info: String,
}
impl Changelog {
    fn is_empty(&self) -> bool {
        self.pull_requests.is_empty()
    }
}

fn usage(message: Option<&str>) -> ! {
    if let Some(message) = message {
        eprintln!("{message}");
        eprintln!("");
    }
    eprintln!("packagist-changelog");
    eprintln!("Tell Slack all pull requests that have been merged since the last deployment.");
    eprintln!("");
    eprintln!("USAGE:");
    eprintln!("    slack-changelog <REPO> <TOKEN> <WEBHOOK>");
    eprintln!("");
    eprintln!("ARGS");
    eprintln!("    <REPO>      The name of the GitHub repository this is being run on (to correctly generate URLs).");
    eprintln!("                Example: rust-lang/rust");
    eprintln!("    <TOKEN>     A GitHub authorization token, to access the API.");
    eprintln!("    <WEBHOOK>   The Slack webhook to post to.");
    exit(1);
}

struct Application {
    repository: (String, String),
    sdk_client: Arc<Octocrab>,
    matcher: Regex,
}
impl Application {
    fn new(args: Vec<String>) -> Result<Self, &'static str> {
        Ok(Self {
            repository: init::repository_tuple(args.get(1))?,
            sdk_client: init::setup_github_sdk(args.get(2))?,
            matcher: Regex::new(PR_COMMIT_REGEX)
                .map_err(|_| "Could not construct matcher from regular expression.")?,
        })
    }

    async fn list_merged_pull_requests_since(&self, base: &str) -> Vec<PullRequest> {
        let repo = self
            .sdk_client
            .repos(self.repository.0.as_str(), self.repository.1.as_str());
        let request_builder = repo.list_commits().sha(base);
        let Ok(_commits) = request_builder.send().await else {
            return Vec::new();
        };

        // TODO
        vec![]
    }
}

fn main() {
    // Args: {GitHub Repository, GitHub Token, Slack Webhook}.
    let args: Vec<String> = std::env::args().take(3).collect();
    let app: Application = match Application::new(args) {
        Ok(app) => app,
        Err(message) => usage(Some(message)),
    };
}
