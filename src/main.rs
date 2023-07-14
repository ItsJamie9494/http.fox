#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{Template};


#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
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
