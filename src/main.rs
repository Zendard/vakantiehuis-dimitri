use rocket::fs::NamedFile;

#[macro_use]
extern crate rocket;

#[get("/")]
async fn language_chooser() -> Option<NamedFile> {
    NamedFile::open("templates/language_chooser.html")
        .await
        .ok()
}

#[get("/")]
async fn index_nl() -> Option<NamedFile> {
    NamedFile::open("templates/nl/index.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![language_chooser])
        .mount("/nl", routes![index_nl])
        .mount("/", rocket::fs::FileServer::from("public"))
}
