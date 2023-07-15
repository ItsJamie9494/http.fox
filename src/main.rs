#[macro_use]
extern crate rocket;

use std::path::PathBuf;

use config::Config;
use rocket::{fs::NamedFile, State};
use rocket_dyn_templates::Template;

pub mod config;
pub mod png;
pub mod status;

#[get("/")]
fn index() -> Template {
    Template::render("index", config::context())
}

#[get("/images/<img>")]
async fn images(img: PathBuf, config: &State<Config>) -> Option<NamedFile> {
    let mut img_path = config.images_dir.clone();

    img_path.push(img);
    if !img_path.is_dir() && img_path.is_file() {
        NamedFile::open(img_path).await.ok()
    } else {
        None
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config = Config::default();

    let _server = rocket::build()
        .mount("/", routes![index, images])
        .attach(Template::fairing())
        .manage(config)
        .launch()
        .await?;

    Ok(())
}
