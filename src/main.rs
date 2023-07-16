#[macro_use]
extern crate rocket;

use config::Config;
use error::error_catchers;
use rocket::{fs::NamedFile, State};
use rocket_dyn_templates::{context, Template};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub mod config;
mod error;
pub mod png;
pub mod status;

#[get("/")]
fn index(config: &State<Config>) -> Template {
    let missing_codes = config.status.not_implemented();
    let codes = config.status.all_statuses();
    Template::render(
        "index",
        context! {
            global: config::context(),
            codes,
            missing_codes,
        },
    )
}

#[get("/<img>")]
async fn img(img: &str, config: &State<Config>) -> Option<NamedFile> {
    let mut img_path = config.images_dir.clone();

    if img.contains("png") {
        img_path.push(img);
    } else {
        img_path.push(format!("{img}.png"))
    }

    if img_path.exists() && img_path.is_file() {
        NamedFile::open(img_path).await.ok()
    } else {
        None
    }
}

#[get("/status/<code>")]
async fn status_details(code: &str, config: &State<Config>) -> Option<Template> {
    if config.status.status_exists(i32::from_str(code).ok()?) {
        let message = config.status.message(i32::from_str(code).ok()?)?;

        Some(Template::render(
            format!("statuses/{code}"),
            context! {
                global: config::context(),
                code,
                message
            },
        ))
    } else {
        None
    }
}

#[get("/static/<type>/<asset>")]
pub async fn static_files(r#type: String, asset: PathBuf) -> Option<NamedFile> {
    match &r#type as &str {
        "css" => {
            let path = Path::new("./static/css").join(asset);
            if path.is_dir() {
                return None;
            }
            NamedFile::open(path).await.ok()
        }
        "js" => {
            let path = Path::new("./static/js").join(asset);
            if path.is_dir() {
                return None;
            }
            NamedFile::open(path).await.ok()
        }
        "img" => {
            let path = Path::new("./static/img").join(asset);
            if path.is_dir() {
                return None;
            }
            NamedFile::open(path).await.ok()
        }
        _ => None,
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config = Config::default();

    let _server = rocket::build()
        .mount("/", routes![index, img, status_details, static_files])
        .register("/", error_catchers())
        .attach(Template::fairing())
        .manage(config)
        .launch()
        .await?;

    Ok(())
}
