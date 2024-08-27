use rocket::request::{self, FromRequest, Request};
use rocket::serde::{Deserialize, Serialize};
use rocket::FromForm;
use rocket::{http::Status, request::Outcome};

#[derive(Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct Booking {
    id: Option<String>,
    pub from: String,
    pub to: String,
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
        .query(
            "SELECT time::format(time::floor(from, 1d), '%Y-%m-%d') AS from,
                time::format(time::floor(to,1d), '%Y-%m-%d') AS to,
                type::string(meta::id(id)) AS id FROM booked",
        )
        .await
        .unwrap()
        .take(0)
        .ok();

    bookings.unwrap()
}

pub async fn add_booking(booking: Booking) -> Option<()> {
    let result = connect_to_db()
        .await
        .query("CREATE booked SET from=<datetime>$from, to=<datetime>$to")
        .bind(booking)
        .await;

    match result {
        Ok(_) => None,
        Err(_) => Some(()),
    }
}

pub async fn delete_booking(id: String) {
    connect_to_db()
        .await
        .query("DELETE type::thing('booked', $id_string)")
        .bind(("id_string", id))
        .await
        .unwrap();
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
