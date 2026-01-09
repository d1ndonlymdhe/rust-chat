use macros::db_func;
use sqlx::{Execute, query_as};
use shared::db::signup::User;
#[db_func]
pub async fn search_users(name: &str,limit:i64,page:i64)->Result<Vec<User>,sqlx::Error>{
    let offset = limit * (page-1);
    let name = format!("%{name}%");
    println!("Searching users with name pattern: {}", name);
    println!("Limit: {}, Offset: {}", limit, offset);
    let sql = query_as!(User,r#"SELECT id,username,hash_password,created_at as "created_at!:String",updated_at as "updated_at!:String" from users where username LIKE $1 LIMIT $2 OFFSET $3"#,name,limit,offset).sql();
    println!("search sql:{}",sql);
    let res = query_as!(User,r#"SELECT id,username,hash_password,created_at as "created_at!:String",updated_at as "updated_at!:String" from users where username LIKE $1 LIMIT $2 OFFSET $3"#,name,limit,offset)    
    .fetch_all(pool).await;
    return res;
}