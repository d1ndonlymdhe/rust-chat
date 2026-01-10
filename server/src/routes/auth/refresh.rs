use rocket::{State, serde::json::Json};
use shared::{Response, routes::auth::refresh::{RefreshRequest, RefreshResponse}};
use sqlx::PgPool;

use crate::db::auth::jwt::get_access_token_from_refresh;

#[post("/refresh",data="<payload>")]
pub async fn refresh(pool: &State<PgPool>, payload:Json<RefreshRequest>)->Response<RefreshResponse>{
    let RefreshRequest {refresh_token} = payload.0;
    let tokens = get_access_token_from_refresh(pool, &refresh_token).await;
    match tokens {
        Ok(tokens) => {
            let (access_token,refresh_token) = tokens;
            Response::success("Refreshed", RefreshResponse { refresh_token, access_token })
        },
        Err(_) => {
            Response::unauthorized("BAD TOKEN", None)
        },
    }
}