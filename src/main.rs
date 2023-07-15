#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;

pub mod config;
pub mod png;

#[get("/")]
fn index() -> Template {
    Template::render("index", config::context())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _server = rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch()
        .await?;

    Ok(())
}
