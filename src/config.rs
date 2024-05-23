use core::{panic, time};

use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde_as(as = "serde_with::DurationSeconds<u64>")]
    #[serde(default = "default_interval")]
    pub interval: time::Duration,
    pub token: Option<String>,
    pub triggers: Vec<Trigger>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Trigger {
    pub target_repo: String,
    pub workflow_repo: String,
    pub workflow_id: String,
    #[serde(default = "default_ref")]
    pub workflow_ref: String,
    pub token: Option<String>,
    pub inputs: Option<String>,
}

fn default_interval() -> time::Duration {
    time::Duration::from_secs(60 * 60)
}
fn default_ref() -> String {
    String::from("main")
}

impl Config {
    pub fn new() -> Self {
        let filename: &str = "config.toml";

        let config: Config = Figment::new()
            .merge(Toml::file(filename))
            .extract::<Config>()
            .unwrap();

        config
    }

    pub fn get_trigger_by_target_repo(&self, target_repo: &str) -> Option<Trigger> {
        let mut t: Trigger;

        if let Some(trigger) = self
            .triggers
            .iter()
            .find(|trigger| trigger.target_repo == target_repo)
        {
            t = trigger.clone();
            match &self.token {
                Some(token) => t.token = Some(token.clone()),
                None => {
                    if t.token.is_none() {
                        panic!("No token provided for {}", target_repo);
                    }
                }
            }
            Some(t)
        } else {
            None
        }
    }
}
