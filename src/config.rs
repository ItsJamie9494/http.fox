use std::{env, path::PathBuf};

use rocket::serde::Serialize;

use crate::status::Statuses;

pub struct Config {
    pub raw_images_dir: PathBuf,
    pub images_dir: PathBuf,
    pub status: Statuses,
}

impl Default for Config {
    fn default() -> Self {
        let current_dir = env::current_dir().expect("Could not find current running directory");
        Config {
            raw_images_dir: {
                let mut dir = current_dir.clone();
                dir.push("static");
                dir.push("images");
                dir.push("raw");
                dir
            },
            images_dir: {
                let mut dir = current_dir.clone();
                dir.push("static");
                dir.push("images");
                dir
            },
            status: Statuses::default(),
        }
    }
}

#[derive(Serialize)]
pub struct GlobalCtx<'a> {
    title: &'a str,
    base_url: &'a str,
}

const GLOBAL_CONTEXT: GlobalCtx<'static> = GlobalCtx {
    title: "http.fox",
    base_url: "http://localhost:8000",
};

/// Create a new Context object with the Global context set
pub fn context() -> GlobalCtx<'static> {
    GLOBAL_CONTEXT
}
