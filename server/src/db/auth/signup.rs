use macros::db_err;
use macros::db_func;
use serde::Serialize;
pub struct User {
    id: i64,
    username: String,
    hash_password: String,
    refresh_token: Option<String>,
    created_at: String,
    updated_at: String
}

#[derive(Serialize)]
pub struct IdOnly {
    id: i64
}



#[db_func]
async fn get_user_as_email(username:&str) -> Result<User,sqlx::Error> {
    let res = sqlx::query_as!(User,r#"SELECT id,username,refresh_token,hash_password,created_at as "created_at!:String",updated_at as "updated_at!:String" from users where username = $1"#,username).fetch_one(pool).await;
    return  res;
}

#[db_func]
async fn create_account(username:&str,password:&str) -> Result<IdOnly,sqlx::Error> {
    let hashed_password = bcrypt::hash(password,bcrypt::DEFAULT_COST).unwrap();
    let res = sqlx::query_as!(IdOnly,"INSERT INTO users (username,hash_password) VALUES ($1,$2) returning id",username,hashed_password).fetch_one(pool).await;
    return res;
}


#[db_err]
pub enum SignupError{
    UserAlreadyExists
}
impl Into<String> for SignupError {
    fn into(self) -> String {
        match self {
            SignupError::UserAlreadyExists => "User already exists".into(),
            SignupError::Sqlx(error) => error.to_string(),
        }
    }
}

#[db_func]
pub async fn signup(username:&str, password:&str)->Result<IdOnly,SignupError>{
    let already_exists = get_user_as_email(pool, username).await.is_ok();
    if already_exists {
        Err(SignupError::UserAlreadyExists)
    }else {
        let user = create_account(pool, username, password).await;
        user.map_err(|err|{err.into()})
    }
}