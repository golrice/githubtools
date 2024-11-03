use clap::{Parser, Subcommand};
use reqwest;
use std::collections::HashMap;

#[derive(Parser)]
#[command(author, version, about, long_about = None, name = "githubtools")]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Activities { name: String },
    Repo { owner: String, name: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.cmd {
        Commands::Activities { name } => {
            let activities = fetch_activities(name.trim()).await?;
            let mut map = HashMap::new();
            for act in activities.iter() {
                let count = map
                    .entry(format!("{}:{}", act["type"], act["repo"]["name"]))
                    .or_insert(0);
                *count += 1;
            }
            map.iter()
                .for_each(|(event, count)| println!("{}, {} times", event, count));
        }
        Commands::Repo { owner, name } => {
            let repo_infos = fetch_repo(&owner, &name).await?;
            repo_infos
                .iter()
                .for_each(|info| println!("{} at {}", info["type"], info["created_at"]));
        }
    }

    Ok(())
}

async fn fetch_activities(
    query: &str,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/users/{}/events", query);
    let response: Vec<serde_json::Value> = client
        .get(url)
        .header("User-Agent", "githubtools")
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

async fn fetch_repo(
    owner: &str,
    repo: &str,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/events", owner, repo);
    let response: Vec<serde_json::Value> = client
        .get(url)
        .header("User-Agent", "githubtools")
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
