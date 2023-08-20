use std::path::PathBuf;

use rocket::serde::Serialize;
use serde::Deserialize;

use crate::{credits::CreditsList, status::Statuses};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub context: Context,
    pub status: Statuses,
    pub credits: CreditsList,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Context {
    pub title: String,
    pub base_url: String,
    pub raw_images_dir: PathBuf,
    pub images_dir: PathBuf,
}
