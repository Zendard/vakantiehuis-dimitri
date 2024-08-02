use rocket::request::{self, FromRequest, Request};
use rocket::serde::{Deserialize, Serialize};
use rocket::FromForm;
use rocket::{http::Status, request::Outcome};
use surrealdb::sql::Datetime;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Booking {
    from: Datetime,
    to: Datetime,
    name: Option<String>,
}

#[derive(FromForm)]
pub struct AdminLogin {
    pub password: String,
}

pub struct Admin;
#[derive(Debug)]
pub struct AuthorizationError;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = AuthorizationError;
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let password = env!("ADMIN_PASSWORD");

        let cookie = req.cookies().get_private("password_hash");

        if cookie.is_none() {
            return Outcome::Error((Status::Unauthorized, AuthorizationError));
        }

        if cookie.unwrap().value() == password {
            Outcome::Success(Admin)
        } else {
            return Outcome::Error((Status::Unauthorized, AuthorizationError));
        }
    }
}

pub async fn get_bookings() -> Vec<Booking> {
    let bookings: Option<Vec<Booking>> = connect_to_db()
        .await
        .query("SELECT from,to FROM booked")
        .await
        .unwrap()
        .take(0)
        .ok();

    bookings.unwrap()
}

async fn connect_to_db() -> surrealdb::Surreal<surrealdb::engine::remote::ws::Client> {
    let db = surrealdb::Surreal::new::<surrealdb::engine::remote::ws::Ws>("localhost:5000")
        .await
        .unwrap();

    db.signin(surrealdb::opt::auth::Root {
        username: "root",
        password: env!("DB_PASSWORD"),
    })
    .await
    .unwrap();

    db.use_ns("zok").use_db("main").await.unwrap();

    db
}

pub async fn check_password(password_input: String) -> Option<String> {
    let correct_password = env!("ADMIN_PASSWORD");

    if password_input == correct_password {
        Some(password_input)
    } else {
        None
    }
}