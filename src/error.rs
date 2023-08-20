use rocket::{fs::NamedFile, Catcher, Request, State};

use crate::config::Config;

async fn handle_error(config: &Config, code: i32) -> NamedFile {
    let mut img_path = config.context.images_dir.clone();

    match code {
        404 => img_path.push("404.png"),
        _ => img_path.push("500.png"),
    }

    NamedFile::open(img_path)
        .await
        .expect("Could not find error image")
}

#[catch(404)]
async fn not_found(req: &Request<'_>) -> NamedFile {
    let config = req
        .guard::<&State<Config>>()
        .await
        .expect("Could not get Config data");

    handle_error(config.inner(), 404).await
}

pub fn error_catchers() -> Vec<Catcher> {
    catchers![not_found]
}
