use octocrab::{Octocrab, OctocrabBuilder};
use std::sync::Arc;

pub(crate) fn repository_tuple(arg: Option<&String>) -> Result<(String, String), &'static str> {
    let mut iter = arg.ok_or("GitHub repository not supplied.")?.splitn(2, '/');
    let owner = iter
        .next()
        .ok_or("Missing owner name in GitHub repository.")?;
    let repository = iter.next().ok_or("Missing name in GitHub repository.")?;
    Ok((owner.to_owned(), repository.to_owned()))
}

pub(crate) fn setup_github_sdk(arg: Option<&String>) -> Result<Arc<Octocrab>, &'static str> {
    let token = arg.ok_or("GitHub token not supplied.")?;
    let builder = OctocrabBuilder::new().personal_token(token.clone());
    octocrab::initialise(builder).map_err(|_| "Could not initialize GitHub SDK client.")
}
