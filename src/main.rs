// general source
// https://blog.logrocket.com/rust-web-apps-using-rocket-framework/

#![feature(decl_macro)]
use rocket_contrib::templates::Template;
use rocket::get;
use rocket::Request;
use rocket::response::content::Json;
use rocket::request::Form;
use rocket::catch;
use rocket::routes;
use rocket::catchers;
use serde::Serialize;


struct Book {
    title: String,
    autor: String,
    isbn: String,
}

#[derive(Serialize)]
struct Context {
    first_name: String,
    last_name: String
}

#[get("/")]
fn index() -> Template {
    let context = Context {
        first_name: String::from("hardcoded firstname"),
        last_name: String::from("hardcoded last name")
    };
    Template::render("home", context)
}

#[get("/hello")]
fn api_hello() -> Json<& 'static str> {
    Json("{
        'status': 'success',
    'message': 'Hello API!'
    }")
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    println!("{}", req);
    format!("not found: {}",req.uri())
}


fn main() {
    rocket::ignite()
    .register(catchers![not_found])
    .mount("/", routes![index])
    .mount("/api", routes![api_hello])
    .attach(Template::fairing())
    .launch();
}
