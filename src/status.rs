use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

const STATUS_JSON: &[u8] = include_bytes!("status.json");

/// An HTTP status
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status {
    pub name: String,
    pub official: bool,
    pub header: Option<Vec<String>>,
    pub links: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct Statuses {
    list: HashMap<String, Status>,
    not_implemented_list: Vec<i32>,
}

#[derive(Deserialize)]
struct StatusJson {
    available: HashMap<String, Status>,
    unavailable: Vec<i32>,
}

fn parse_json() -> Result<(HashMap<String, Status>, Vec<i32>), Box<dyn Error>> {
    let value: StatusJson = serde_json::from_slice(STATUS_JSON)?;
    let mut not_impl = Vec::new();

    for item in value.unavailable {
        not_impl.push(item);
    }

    not_impl.sort();

    Ok((value.available, not_impl))
}

impl Default for Statuses {
    fn default() -> Self {
        let (list, not_impl) = parse_json().expect("Failed to parse Statuses list");
        Self {
            list,
            not_implemented_list: not_impl,
        }
    }
}

impl Statuses {
    fn search_status(&self, status: &str) -> Option<(&String, &Status)> {
        self.list.get_key_value(status)
    }

    pub fn status_exists(&self, status: &str) -> bool {
        self.search_status(status).is_some()
    }

    pub fn message(&self, status: &str) -> Option<&String> {
        if let Some(status) = self.search_status(status) {
            Some(&status.1.name)
        } else {
            None
        }
    }

    pub fn not_implemented(&self) -> Vec<i32> {
        self.not_implemented_list.clone()
    }

    pub fn all_statuses(&self) -> Vec<(String, Status)> {
        let mut list = self
            .list
            .clone()
            .into_iter()
            .collect::<Vec<(String, Status)>>();

        list.sort_by(|a, b| a.0.cmp(&b.0));
        return list;
    }
}
