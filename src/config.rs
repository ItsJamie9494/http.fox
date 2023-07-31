use std::{
    env,
    fs::File,
    path::{Path, PathBuf},
};

use rocket::serde::Serialize;
use serde::Deserialize;

use crate::{credits::CreditsList, status::Statuses};

fn get_config_location() -> PathBuf {
    match env::var("HTTPFOX_CONFIG") {
        Ok(key) => {
            let p = Path::new(&key).to_owned();

            if !p.exists() {
                env::current_dir()
                    .map(|x| {
                        let mut x = x;
                        x.push(p);
                        x
                    })
                    .expect("Could not open current directory")
            } else {
                p
            }
        }
        Err(_) => Path::new("config.json").to_owned(),
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    context: Context,
    pub status: Statuses,
    pub credits: CreditsList,
}

impl Config {
    /// Helper function to get the context, as it cannot be a reference
    pub fn context(&self) -> Context {
        self.context.to_owned()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Context {
    pub title: String,
    pub base_url: String,
    pub raw_images_dir: PathBuf,
    pub images_dir: PathBuf,
}

impl Default for Context {
    fn default() -> Self {
        let config = get_config_location();

        if let Ok(file) = File::open(config.clone()) {
            match serde_json::from_reader(file) {
                Ok(conf) => conf,
                Err(err) => {
                    eprintln!(
                        "WARNING: Could not find config file at {}, using defaults",
                        config.display()
                    );
                    eprintln!("Error: {err}");
                    Self::create()
                }
            }
        } else {
            eprintln!(
                "Could not open config file {} for reading",
                config.display()
            );
            Self::create()
        }
    }
}

impl Context {
    fn create() -> Self {
        let current_dir = env::current_dir().expect("Could not open current directory");

        Context {
            title: "http.fox".to_owned(),
            base_url: "http://localhost:8000".to_owned(),
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
        }
    }
}
