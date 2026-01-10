use rocket::State;
use shared::{Response, routes::users::search::{SearchUser, SearchUserResult}};
use sqlx::{PgPool};

use crate::db::{auth::jwt::Claims, users::search};

#[get("/search?<name>&<page>&<limit>")]
pub async fn search_users(pool:&State<PgPool>,name:&str,page:i64,limit:i64,_claims: Claims)->Response<SearchUserResult>{
    let users = search::search_users(pool, name, limit, page).await;
    match users {
        Ok(users) => Response::success("Users fetched", SearchUserResult::new(users.into_iter().map(|v|{v.into()}).collect::<Vec<SearchUser>>()) ),
        Err(err) => Response::internal_error(&err.to_string(), None),
    }
}