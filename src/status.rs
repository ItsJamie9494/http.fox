use std::error::Error;
use std::str::FromStr;

use serde::Deserialize;
use serde_json::{Map, Value};

const STATUS_JSON: &[u8] = include_bytes!("status.json");

/// An HTTP status
type Status = (i32, String);

pub struct Statuses {
    list: Vec<Status>,
    not_implemented_list: Vec<i32>,
}

#[derive(Deserialize)]
struct StatusJson {
    available: Map<String, Value>,
    unavailable: Vec<i32>,
}

fn parse_json() -> Result<(Vec<Status>, Vec<i32>), Box<dyn Error>> {
    let value: StatusJson = serde_json::from_slice(STATUS_JSON)?;
    let mut list = Vec::new();
    let mut not_impl = Vec::new();

    for item in value.available {
        let status = (
            i32::from_str(&item.0)?,
            item.1.as_str().expect("Invalid Format").to_string(),
        );
        list.push(status)
    }

    for item in value.unavailable {
        not_impl.push(item);
    }

    Ok((list, not_impl))
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
    fn search_status(&self, status: i32) -> Option<&Status> {
        let search = self.list.binary_search_by_key(&status, |s| s.0);
        if let Ok(pos) = search {
            self.list.get(pos)
        } else {
            None
        }
    }

    pub fn status_exists(&self, status: i32) -> bool {
        self.search_status(status).is_some()
    }

    pub fn message(&self, status: i32) -> Option<&str> {
        if let Some(status) = self.search_status(status) {
            Some(&status.1)
        } else {
            None
        }
    }

    pub fn not_implemented(&self) -> Vec<i32> {
        self.not_implemented_list.clone()
    }

    pub fn all_statuses(&self) -> Vec<Status> {
        self.list.clone()
    }
}
