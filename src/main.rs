#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content;

#[get("/health")]
fn health() -> content::Json<&'static str>{
    content::Json("{'status':'UP'}")
}

#[get("/<rut>/<name>")]
fn get_detail(name: String, rut: String) -> content::Json<String> {
    let detail:String = format!("{{ '{}':'{}'}}", rut, name);
    content::Json(detail)
}

#[post("/<rut>")]
fn new_person(rut: String) -> content::Json<String> {
    content::Json(format!("{{'created':'{}'}}", rut))
}

#[put("/<rut>")]
fn update_person(rut: String) -> content::Json<String> {
    content::Json(format!("{{'updated':'{}'}}", rut))
}

#[delete("/<rut>")]
fn del_person(rut: String) -> content::Json<String> {
    content::Json(format!("{{'deleted':'{}'}}", rut))
}

fn main() {
    rocket::ignite().mount("/gs", routes![get_detail,
    health,
    new_person,
    update_person,
    del_person]).launch();
}