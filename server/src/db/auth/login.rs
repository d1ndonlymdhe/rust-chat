use macros::{db_err, db_func};
use shared::{db::signup::User};
use sqlx::{query_as};
use shared::AnyErr;

#[db_err]
#[derive(Debug)]
pub enum LoginError{
    WrongPassword,
}


#[db_func]
pub async fn check_password(username:&str,password:&str) -> Result<User,LoginError>{
    let user = query_as!(User,
        r#"SELECT id,username,hash_password,created_at as "created_at!:String",updated_at as "updated_at!:String" from users where username = $1"#
        ,username).fetch_optional(pool).await;
    if let Err(e) = user {
        return Err(e.into());
    }
    let user = user.unwrap();
    if let Some(user) = user {
        let pass = bcrypt::verify(password, &user.hash_password).unwrap();
        if pass {
            return Ok(user);
        }else{
            return Err(LoginError::WrongPassword);
        }
    }
    return Err(LoginError::WrongPassword);
}

