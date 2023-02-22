use regex::Regex;

use crate::models::Commit;

pub(crate) struct PullRequestCommit {
    repository: String,
    id: u32,
    branch: String,
    title: Option<String>,
}
impl PullRequestCommit {
    // Would be nice to implement this in FromStr, but I don't know how to access the
    // matcher from from_str() without initializing it each time.
    pub(crate) fn from_commit_message(
        matcher: &Regex,
        repository: &str,
        commit_message: &str,
    ) -> Result<Self, ()> {
        let Some(captures) = matcher.captures(commit_message) else {
            return Err(());
        };

        Ok(Self {
            repository: repository.to_string(),
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

    pub(crate) fn get_url(&self) -> String {
        format!("https://github.com/{}/pull/{}", self.repository, self.id)
    }
}

pub(crate) struct Changelog {
    pull_requests: Vec<PullRequestCommit>,
    info: String,
}
impl Changelog {
    pub(crate) fn new(
        matcher: &Regex,
        repository: &str,
        info: String,
        commits: Vec<Commit>,
    ) -> Self {
        Self {
            info,
            pull_requests: commits
                .into_iter()
                .filter_map(|commit| {
                    PullRequestCommit::from_commit_message(
                        matcher,
                        repository,
                        &commit.commit.message,
                    )
                    .ok()
                })
                .collect(),
        }
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.pull_requests.is_empty()
    }
}
