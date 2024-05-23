use std::collections::HashMap;

use figment::{
    providers::{Format, Toml},
    Figment,
};
use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    records: Option<Vec<RecordItem>>,
}
#[derive(Serialize, Deserialize, Debug)]
struct RecordItem {
    repo: String,
    tag: String,
}

const RECORDS_FILENAME: &str = "records.toml";

impl Record {
    pub fn new() -> Self {
        let records: Record = Figment::new()
            .merge(Toml::file(RECORDS_FILENAME))
            .extract()
            .unwrap();

        records
    }

    pub fn print(&self) {
        match &self.records {
            Some(r) => println!("{:#?}", r),
            None => warn!("No record data available."),
        }
    }

    pub fn set(&mut self, repo: &str, tag: &str) {
        let item = RecordItem {
            repo: repo.to_string(),
            tag: tag.to_string(),
        };
        match &mut self.records {
            Some(r) => r.push(item),
            None => self.records = Some(vec![item]),
        }
    }

    pub fn get(&self, repo: &str) -> String {
        match &self.records {
            Some(r) => match r.iter().find(|i| i.repo == repo) {
                Some(i) => i.tag.clone(),
                None => String::from(""),
            },
            None => String::from(""),
        }
    }

    pub fn get_map(&self) -> HashMap<&str, String> {
        match &self.records {
            Some(r) => {
                let mut map = HashMap::new();
                for item in r {
                    map.insert(item.repo.as_str(), item.tag.clone());
                }
                map
            }
            None => HashMap::new(),
        }
    }

    pub fn set_map(&mut self, map: HashMap<&str, String>) {
        match &mut self.records {
            Some(r) => {
                for (key, value) in map {
                    if let Some(i) = r.iter_mut().find(|i| i.repo == key) {
                        i.tag = value.to_string();
                    } else {
                        let item = RecordItem {
                            repo: key.to_string(),
                            tag: value.to_string(),
                        };
                        r.push(item);
                    }
                }
            }
            None => {
                self.records = Some(
                    map.into_iter()
                        .map(|(repo, tag)| RecordItem {
                            repo: repo.to_string(),
                            tag,
                        })
                        .collect(),
                )
            }
        }
    }

    pub fn write(&self) {
        let toml_string = toml::to_string_pretty(&self).unwrap();
        std::fs::write(RECORDS_FILENAME, toml_string).unwrap();
    }
}
