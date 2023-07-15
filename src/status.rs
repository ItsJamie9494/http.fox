use std::error::Error;
use std::str::FromStr;

use serde_json::Value;

const STATUS_JSON: &[u8] = include_bytes!("status.json");

/// An HTTP status
type Status = (i32, String);

pub struct Statuses {
    list: Vec<Status>,
}

fn parse_json() -> Result<Vec<Status>, Box<dyn Error>> {
    let value: Value = serde_json::from_slice(STATUS_JSON)?;
    let mut list = Vec::new();

    if value.is_object() {
        let obj = value.as_object().expect("Invalid Statuses Format");

        for item in obj {
            let status = (
                i32::from_str(item.0)?,
                item.1.as_str().expect("Invalid Format").to_string(),
            );
            list.push(status)
        }
    }

    Ok(list)
}

impl Default for Statuses {
    fn default() -> Self {
        let list = parse_json().expect("Failed to parse Statuses list");
        Self { list }
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
}
