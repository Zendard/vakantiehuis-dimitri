use rocket::{form::Form, fs::NamedFile, http::CookieJar, response::Redirect};
use rocket_dyn_templates::{context, Template};

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

#[get("/kalender")]
async fn calendar_nl() -> Template {
    let bookings = vakantiehuis_dimitri::get_bookings().await;
    Template::render("nl/calendar", context! {bookings})
}

#[get("/admin")]
async fn admin(_admin: vakantiehuis_dimitri::Admin) -> Template {
    let bookings = vakantiehuis_dimitri::get_bookings().await;
    Template::render("admin", context! {bookings})
}

#[post("/admin-login", data = "<form>")]
async fn check_login(
    form: Form<vakantiehuis_dimitri::AdminLogin>,
    jar: &CookieJar<'_>,
) -> Redirect {
    let password_input = &form.password.to_string();
    let is_correct = vakantiehuis_dimitri::check_password(password_input.to_string()).await;

    if let Some(cookie_value) = is_correct {
        jar.add_private(("password_hash", cookie_value));
        Redirect::to("/admin")
    } else {
        Redirect::to("/admin-login?error=wrong-password")
    }
}

#[catch(401)]
async fn admin_login_catcher() -> Option<NamedFile> {
    NamedFile::open("templates/admin_login.html").await.ok()
}

#[get("/admin-login")]
async fn admin_login() -> Option<NamedFile> {
    NamedFile::open("templates/admin_login.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![language_chooser, admin, check_login, admin_login],
        )
        .mount("/nl", routes![index_nl, calendar_nl])
        .register("/", catchers![admin_login_catcher])
        .mount("/", rocket::fs::FileServer::from("public"))
        .attach(Template::fairing())
}
