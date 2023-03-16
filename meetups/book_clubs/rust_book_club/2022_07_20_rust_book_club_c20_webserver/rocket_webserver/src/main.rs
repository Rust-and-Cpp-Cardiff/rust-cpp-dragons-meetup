use std::{fs, thread, time::Duration};

use rocket::{launch, catchers, routes, catch, get, fs::NamedFile};

#[launch]
fn rocket() -> _ {
     rocket::build()
     .register("/", catchers![not_found])
     .mount("/", routes![hello, sleep])
}

#[get("/")]
async fn hello() -> Option<NamedFile> {
    NamedFile::open("hello.html").await.ok()
}

#[get("/sleep")]
async fn sleep() -> Option<NamedFile> {
    thread::sleep(Duration::from_secs(5));
    NamedFile::open("hello.html").await.ok()
}

#[catch(404)]
async fn not_found(_: &rocket::Request<'_>) -> Option<NamedFile> {
    NamedFile::open("404.html").await.ok()
}