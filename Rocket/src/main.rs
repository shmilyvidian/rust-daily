#[macro_use] extern crate rocket;

#[get("/word/<name>")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/hello", routes![world])
        .launch()
        .await;
}