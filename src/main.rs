#[macro_use]
extern crate rocket;

use config::Config;
use error::error_catchers;
use rocket::{fs::NamedFile, State};
use rocket_dyn_templates::{context, Template};

pub mod config;
mod error;
pub mod png;
pub mod status;

#[get("/")]
fn index(config: &State<Config>) -> Template {
    let not_impl = config.status.not_implemented();
    Template::render(
        "index",
        context! {
            global: config::context(),
            unimplemented: not_impl,
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

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config = Config::default();

    let _server = rocket::build()
        .mount("/", routes![index, img])
        .register("/", error_catchers())
        .attach(Template::fairing())
        .manage(config)
        .launch()
        .await?;

    Ok(())
}
