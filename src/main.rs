use rocket::fs::NamedFile;

#[macro_use]
extern crate rocket;

#[get("/")]
async fn language_chooser() -> Option<NamedFile> {
    NamedFile::open("templates/language_chooser.html")
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![language_chooser])
        .mount("/", rocket::fs::FileServer::from("public"))
}
