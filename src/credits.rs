use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CreditsList {
    list: Vec<Credits>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Credits {
    name: String,
    url: String,
    codes: Vec<i32>,
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
