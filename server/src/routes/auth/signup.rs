use rocket::{State, serde::json::Json};
use shared::{
    Response, routes::auth::signup::{SignupRequest, SignupResponse}
};
use sqlx::SqlitePool;

use crate::db;

#[post("/signup", data = "<payload>")]
pub async fn signup(
    pool: &State<SqlitePool>,
    payload: Json<SignupRequest>,
) -> Response<SignupResponse> {
    let SignupRequest { email, password } = payload.0;
    let new_user = db::auth::signup::signup(pool, &email, &password).await;
    match new_user {
        Ok(id) => Response::success("User created successfully",id),
        Err(e) => {
            match e {
                db::auth::signup::SignupError::UserAlreadyExists => {
                    Response::bad_request("User already exists",None)
                },
                db::auth::signup::SignupError::Sqlx(error) => {
                    let e_string: String = error.to_string();
                    Response::internal_error(&e_string, None)
                }
            }
        }
    }
}
