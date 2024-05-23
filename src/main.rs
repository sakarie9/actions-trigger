use std::collections::HashMap;

use log::{debug, error, info, warn};
use reqwest::Error;

mod request;
use request::Request;

mod record;
use record::Record;

mod config;
use config::{Config, Trigger};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        // .format_target(false)
        .format_timestamp(None)
        .init();

    let config = Config::new();
    debug!("Config: {:?}", config);

    let target_repos: Vec<&str> = config
        .triggers
        .iter()
        .map(|trigger| trigger.target_repo.as_str())
        .collect();
    debug!("Target repos: {:?}", target_repos);

    checkupdates(&config, target_repos).await;

    Ok(())
}

async fn checkupdates(config: &Config, repos: Vec<&str>) {
    let mut latest_map = HashMap::new();
    let mut records = Record::new();
    // let record_map = records.get_map();

    for repo in repos {
        info!("========================================");
        info!("[{}]: checking updates", repo);
        let tag = get_latest_tag(repo).await;
        if tag.is_empty() {
            continue;
        }

        let record_tag = records.get(repo);

        if tag != record_tag {
            info!("[{}]: tag has changed: {} -> {}", repo, record_tag, tag);
            // Trigger a workflow here
            let trigger = config.get_trigger_by_target_repo(repo);
            match trigger {
                Some(t) => {
                    trigger_workflow(&t).await;
                }
                None => error!("[{}]: No config found", repo),
            }
        } else {
            info!("[{}]: no updates", repo);
        }

        latest_map.insert(repo, tag);
    }

    // merge latest_map with records
    records.set_map(latest_map);
    // debug!("{:#?}", records);
    records.write();
}

async fn trigger_workflow(trigger: &Trigger) {
    if trigger.workflow_id.is_empty() || trigger.workflow_repo.is_empty() {
        error!(
            "[{}]: Missing workflow_id or workflow_repo",
            trigger.target_repo
        );
        return;
    }
    info!(
        "[{}]: Triggering workflow {} on repo {}",
        trigger.target_repo, trigger.workflow_id, trigger.workflow_repo
    );
    let url = format!(
        "https://api.github.com/repos/{}/actions/workflows/{}/dispatches",
        trigger.workflow_repo, trigger.workflow_id
    );
    let response = Request::new()
        .post(
            url,
            trigger.token.as_ref().unwrap().to_string(),
            trigger.workflow_ref.to_string(),
            String::from(""),
        )
        .await;
    match &response {
        Ok(_) => info!(
            "[{}]: Triggered workflow {} on repo {}",
            trigger.target_repo, trigger.workflow_id, trigger.workflow_repo
        ),
        Err(e) => error!("[{}]: Error during the request: {}", trigger.target_repo, e),
    }
}

async fn get_latest_tag(repo: &str) -> String {
    let url = format!("https://api.github.com/repos/{repo}/releases");
    let response = Request::new().get(url).await;
    match &response {
        Ok(contents) => {
            let tag = contents[0]["tag_name"].as_str().unwrap_or("");
            debug!("[{}]: latest tag: {:?}", repo, tag);
            tag.to_string()
        }
        Err(e) => {
            error!("[{}]: Error during the request: {}", repo, e);
            String::from("")
        }
    }
}
