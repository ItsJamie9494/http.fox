use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::Value;

const CREDITS_JSON: &[u8] = include_bytes!("credits.json");

#[derive(Clone)]
pub struct CreditsList {
    list: Vec<Credits>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Credits {
    name: String,
    url: String,
    codes: Vec<i32>,
}

fn parse_json() -> Result<Vec<Credits>, Box<dyn Error>> {
    let value: Value = serde_json::from_slice(CREDITS_JSON)?;

    match value {
        Value::Object(contributors) => {
            let mut contribution_list = Vec::new();

            for contributor in contributors {
                let credits: Credits = serde_json::from_value(contributor.1)?;

                contribution_list.push(credits);
            }
            Ok(contribution_list)
        }
        _ => Err(String::from("Invalid JSON Format").into()),
    }
}

impl Default for CreditsList {
    fn default() -> Self {
        Self {
            list: parse_json().expect("Failed to parse Credits list"),
        }
    }
}

impl CreditsList {
    pub fn search_credits(self, status: i32) -> Option<Credits> {
        let search = self.list.iter().position(|x| x.codes.contains(&status));

        if let Some(pos) = search {
            self.list.get(pos).cloned()
        } else {
            None
        }
    }

    pub fn all_credits(&self) -> Vec<Credits> {
        self.list.clone()
    }
}
