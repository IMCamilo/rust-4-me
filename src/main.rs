#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content;

#[get("/health")]
fn health() -> content::Json<&'static str>{
    content::Json("{'status':'UP'}")
}

#[get("/<dni>/<name>")]
fn get_rustacean(name: String, dni: String) -> content::Json<String> {
    let detail:String = format!("{{ '{}':'{}'}}", dni, name);
    content::Json(detail)
}

#[post("/<dni>")]
fn create_rustacean(dni: String) -> content::Json<String> {
    content::Json(format!("{{'created':'{}'}}", dni))
}

#[put("/<dni>")]
fn update_rustacean(dni: String) -> content::Json<String> {
    content::Json(format!("{{'updated':'{}'}}", dni))
}

#[delete("/<dni>")]
fn delete_rustacean(dni: String) -> content::Json<String> {
    content::Json(format!("{{'deleted':'{}'}}", dni))
}

fn main() {
    rocket::ignite().mount("/gs", routes![
    health,
    get_rustacean,
    create_rustacean,
    update_rustacean,
    delete_rustacean
    ]).launch();
}
