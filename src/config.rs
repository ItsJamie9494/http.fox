use std::{env, path::PathBuf};

use rocket::serde::Serialize;

pub struct Config {
    pub raw_images_dir: PathBuf,
    pub images_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let current_dir = env::current_dir().expect("Could not find current running directory");
        Config {
            raw_images_dir: {
                let mut dir = current_dir.clone();
                dir.push("images");
                dir.push("raw");
                dir
            },
            images_dir: {
                let mut dir = current_dir.clone();
                dir.push("images");
                dir
            },
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
    base_url: "http://127.0.0.1",
};

/// Create a new Context object with the Global context set
pub fn context() -> GlobalCtx<'static> {
    GLOBAL_CONTEXT
}
