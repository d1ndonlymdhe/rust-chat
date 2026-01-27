use rocket::{State, get};
use shared::{routes::users::search::{SearchUser, SearchUserResult}};
use sqlx::{PgPool};

use crate::{db::{auth::{jwt::Claims, signup}, users::search}, lib::Response};

#[get("/search?<name>&<page>&<limit>")]
pub async fn search_users(pool:&State<PgPool>,name:&str,page:i64,limit:i64,_claims: Claims)->Response<SearchUserResult>{
    let users = search::search_users(pool, name, limit, page).await;
    match users {
        Ok(users) => Response::success("Users fetched", SearchUserResult::new(users.into_iter().map(|v|{v.into()}).collect::<Vec<SearchUser>>()) ),
        Err(err) => Response::internal_error(&err.to_string(), None),
    }
}

#[get("/me")]
pub async fn search_me(pool:&State<PgPool>,claims: Claims)->Response<SearchUser>{
    let user = signup::get_user_by_id(pool, claims.user_id).await;
    match user {
        Ok(user) => {
            match user {
                Some(u) => Response::success("User fetched", SearchUser { id: u.id, username: u.username }),
                None => Response::not_found("User not found", None),
            }
        },
        Err(err) => Response::internal_error(&err.to_string(), None),
    }
}