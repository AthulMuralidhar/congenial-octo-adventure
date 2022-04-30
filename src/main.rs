// general source
// https://blog.logrocket.com/rust-web-apps-using-rocket-framework/


struct Book {
    title: String,
    autor: String,
    isbn: String,
}

#[get("/")]
fn index() -> Template {

}

fn main() {
    println!("Hello, world!");
}
