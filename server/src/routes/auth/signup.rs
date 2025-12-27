use rocket::{State, serde::{json::Json}};
use serde::{Deserialize};
use sqlx::SqlitePool;
use shared::Response;
use crate::db::{self, auth::signup::IdOnly};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String
}
#[post("/",data="<payload>")]
pub async fn signup(pool: &State<SqlitePool>,payload: Json<SignupRequest>) -> Json<Response<Option<IdOnly>>>{
    let SignupRequest{email,password} = payload.0;
    let new_user = db::auth::signup::signup(pool, &email, &password).await;
    match new_user {
        Ok(id) => {
            Json(Response::new(true, "User created successfully", Some(id)))
        },
        Err(e) => {
            let e_string:String = e.into();
            Json(Response::new(false, &e_string, None))
        },
    }
}
