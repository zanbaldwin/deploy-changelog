use crate::DEFAULT_BRANCH;
use clap::{Parser, Subcommand};
use octocrab::{Octocrab, OctocrabBuilder};
use std::process::exit;

#[derive(Parser, Debug)]
#[command(author, version, about, next_line_help = false)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
    pub(crate) repo: String,
    #[arg(long, default_value = DEFAULT_BRANCH)]
    pub(crate) head: String,
    #[arg(short, long, env = "GITHUB_TOKEN")]
    pub(crate) token: String,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    List,
    Notify {
        #[arg(short, long)]
        webhook: String,
    },
}

pub(crate) fn setup_octocrab_client(token: String) -> Result<Octocrab, &'static str> {
    let client = OctocrabBuilder::new()
        .personal_token(token.clone())
        .build()
        .map_err(|_| "Could not initialize GitHub SDK client.")?;
    Ok(client)
}

pub(crate) fn error(message: &str, code: i32) -> ! {
    eprintln!("{message}");
    exit(code);
}
