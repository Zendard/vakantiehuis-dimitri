use std::str::FromStr;

use rocket::{form::Form, fs::NamedFile, http::CookieJar, response::Redirect};
use rocket_dyn_templates::{context, Template};
use surrealdb::sql::Datetime;

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

#[get("/boeken")]
async fn calendar_nl() -> Template {
    let bookings = vakantiehuis_dimitri::get_bookings().await;
    Template::render("nl/calendar", context! {bookings})
}

#[get("/activiteiten")]
async fn activities_nl() -> Option<NamedFile> {
    NamedFile::open("templates/nl/activities.html").await.ok()
}

#[get("/admin")]
async fn admin(_admin: vakantiehuis_dimitri::Admin) -> Template {
    let bookings = vakantiehuis_dimitri::get_bookings().await;
    Template::render("admin", context! {bookings})
}

#[post("/admin/add-booking", data = "<form>")]
async fn add_booking(form: Form<vakantiehuis_dimitri::Booking>) -> Redirect {
    if form.from > form.to {
        return Redirect::to("/admin?add-booking=wrongInput");
    }

    let mut overlap = false;

    vakantiehuis_dimitri::get_bookings()
        .await
        .iter()
        .for_each(|booking| {
            if Datetime::from_str(&booking.from) <= Datetime::from_str(&form.to)
                && Datetime::from_str(&form.from) <= Datetime::from_str(&booking.to)
            {
                overlap = true;
            }
        });
    if overlap {
        return Redirect::to("/admin?add-booking=alreadyBooked");
    };

    let error = vakantiehuis_dimitri::add_booking(form.into_inner()).await;
    match error {
        None => Redirect::to("/admin?add-booking=success"),
        Some(_) => Redirect::to("/admin?add-booking=error"),
    }
}

#[get("/admin/delete/<id>")]
async fn delete_booking(id: String) -> Redirect {
    vakantiehuis_dimitri::delete_booking(id).await;
    Redirect::to("/admin")
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
            routes![
                language_chooser,
                admin,
                check_login,
                admin_login,
                add_booking,
                delete_booking
            ],
        )
        .mount("/nl", routes![index_nl, calendar_nl, activities_nl])
        .register("/", catchers![admin_login_catcher])
        .mount("/", rocket::fs::FileServer::from("public"))
        .attach(Template::fairing())
}
