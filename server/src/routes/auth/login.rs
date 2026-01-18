use crate::{
    db::auth::{
        jwt::{get_access_token_from_refresh, get_new_refresh_token},
        login::check_password,
    },
    lib::Response,
};
use rocket::{State, post, serde::json::Json};
use shared::routes::auth::login::{LoginRequest, LoginResponse};
use sqlx::PgPool;

#[post("/login", data = "<payload>")]
pub async fn login(pool: &State<PgPool>, payload: Json<LoginRequest>) -> Response<LoginResponse> {
    let LoginRequest { email, password } = payload.0;
    let user = check_password(pool, &email, &password).await;
    if user.is_ok() {
        let user = user.unwrap();
        let refresh_token = get_new_refresh_token(pool, user.id).await;
        if refresh_token.is_err() {
            return Response::internal_error("COULD NOT GENERATE REFRESH TOKEN", None);
        }
        let refresh_token = refresh_token.unwrap();
        let new_tokens = get_access_token_from_refresh(pool, &refresh_token).await;
        if new_tokens.is_err() {
            return Response::internal_error("COULD NOT GENERATE ACCESS TOKEN", None);
        }
        let new_tokens = new_tokens.unwrap();
        return Response::success(
            "SUCCESS",
            LoginResponse {
                access_token: new_tokens.0.clone(),
                refresh_token: new_tokens.1.clone(),
            },
        );
    };
    return Response::unauthorized("UNAUTHORIZED", None);
}
