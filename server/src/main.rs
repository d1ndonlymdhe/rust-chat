#[macro_use]
extern crate rocket;

use std::{env, str::FromStr};

use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgConnectOptions};

use crate::routes::{auth::{login::login, refresh::refresh, signup::signup}, users::search::search_users};

mod routes;
mod db;

#[get("/")]
fn index() -> &'static str {
    return "Hello World";
}
#[launch]
async fn rocket() -> _ {
    let _ = dotenv();
    let db_url = env::var("DATABASE_URL").expect("DB_URL not set");
    let connect_options = PgConnectOptions::from_str(&db_url)
        .unwrap();
    let pool = PgPool::connect_with(connect_options)
        .await
        .expect("Unable to connect to database");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    rocket::build()
    .manage(pool)
    .mount("/", routes![index])
    .mount("/auth", routes![signup,login,refresh])
    .mount("/users",routes![search_users])

}
