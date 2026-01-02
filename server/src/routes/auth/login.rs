use rocket::{State, serde::json::Json};
use shared::{Response, routes::auth::login::{LoginRequest, LoginResponse}};
use sqlx::SqlitePool;

use crate::db::auth::{jwt::get_new_refresh_token, login::check_password};

#[post("/login",data="<payload>")]
pub async fn login(pool: &State<SqlitePool>, payload:Json<LoginRequest>)->Response<LoginResponse>{
    let LoginRequest {email,password} = payload.0;
    let user = check_password(pool, &email, &password).await;
    if user.is_ok() {
        let user = user.unwrap();
        let refresh_token = get_new_refresh_token(pool, user.id).await;
        if refresh_token.is_err() {
            return Response::internal_error("COULD NOT GENERATE REFRESH TOKEN", None);
        }

    };
    return Response::unauthorized("UNAUTHORIZED", None)
}