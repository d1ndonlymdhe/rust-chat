use chrono::Utc;
use macros::{db_err, db_func,any_cast};
use serde::{Deserialize, Serialize};
use shared::db::signup::IdOnly;
use sqlx::{query, query_as};

pub struct AnyErr(pub ());
impl From<()> for AnyErr {
    fn from(_value: ()) -> Self {
        return AnyErr(());
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    version: i64,
    user_id: i64,
    exp: chrono::DateTime<chrono::Utc>,
}

impl Claims {
    pub fn new_v1(user_id: i64, exp: chrono::DateTime<chrono::Utc>) -> Self {
        return Claims {
            version: 1,
            user_id: user_id,
            exp,
        };
    }
}

pub enum CompareRefreshError {
    NotFound,
    Expired,
    Revoked,
    Other,
}

#[derive(Debug, PartialEq)]
#[any_cast]
pub enum JWTError {
    Expired,
    Other,
}

pub fn get_refresh_claims(token: &str) -> Result<Claims, JWTError> {
    let key = std::env::var("JWT_REFRESH_KEY").unwrap();
    let key = jsonwebtoken::DecodingKey::from_base64_secret(&key).unwrap();

    let claims = jsonwebtoken::decode::<Claims>(
        token,
        &key,
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::default()),
    );
    match claims {
        Ok(d) => {
            let claims = d.claims;
            return Ok(claims);
        }
        Err(e) => match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => return Err(JWTError::Expired),
            _ => return Err(JWTError::Other),
        },
    }
}

pub fn get_access_claims(token: &str) -> Result<Claims, JWTError> {
    let key = std::env::var("JWT_ACCESS_KEY").unwrap();
    let key = jsonwebtoken::DecodingKey::from_base64_secret(&key).unwrap();

    let claims = jsonwebtoken::decode::<Claims>(
        token,
        &key,
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::default()),
    );
    match claims {
        Ok(d) => {
            let claims = d.claims;
            return Ok(claims);
        }
        Err(e) => match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => return Err(JWTError::Expired),
            _ => return Err(JWTError::Other),
        },
    }
}

pub fn get_refresh_token(user_id: i64) -> String {
    // 3 days expiry for refresh token
    let expiration = Utc::now().checked_add_days(chrono::Days::new(3)).unwrap();
    let claims = Claims::new_v1(user_id, expiration);
    let refresh_token_key = std::env::var("JWT_REFRESH_KEY").unwrap();
    let new_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &(jsonwebtoken::EncodingKey::from_base64_secret(&refresh_token_key).unwrap()),
    )
    .unwrap();
    return new_token;
}

pub fn get_access_token(user_id: i64) -> String {
    // 15 mins expiry for access token
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(15))
        .unwrap();
    let claims = Claims::new_v1(user_id, expiration);
    let refresh_token_key = std::env::var("JWT_REFRESH_KEY").unwrap();
    let new_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &(jsonwebtoken::EncodingKey::from_base64_secret(&refresh_token_key).unwrap()),
    )
    .unwrap();
    return new_token;
}

#[db_func]
pub async fn get_access_token_from_refresh(
    refresh_token: &str,
) -> Result<(String, String), AnyErr> {
    let claims = get_refresh_claims(refresh_token)?;
    let user_id = claims.user_id;
    let access_token = get_access_token(user_id);
    let new_refresh_token = refresh_refresh_token(pool, refresh_token).await?;
    return Ok((access_token, new_refresh_token));
}

#[db_func]
pub async fn get_new_refresh_token(user_id: i64) -> Result<String, ()> {
    let token = get_refresh_token(user_id);
    let r = add_new_token_to_new_family(pool, user_id, &token).await;
    if r.is_err() {
        return Err(());
    }
    return Ok(token);
}

#[db_err]
pub enum RefreshRefreshTokenErr {
    ExpiredToken,
    InvalidToken,
}

#[db_func]
pub async fn refresh_refresh_token(token: &str) -> Result<String, RefreshRefreshTokenErr> {
    let claims = get_refresh_claims(token);

    match claims {
        Ok(claims) => {
            let user_id: i64 = claims.user_id;
            let new_token = get_refresh_token(user_id);
            add_token(pool, &new_token, &token).await?;
            Ok(new_token)
        }
        Err(e) => match e {
            JWTError::Expired => Err(RefreshRefreshTokenErr::ExpiredToken),
            JWTError::Other => Err(RefreshRefreshTokenErr::InvalidToken),
        },
    }
}

#[db_func]
pub async fn add_new_token_to_family(token: &str, family_id: i64) -> Result<(), sqlx::Error> {
    let mut txn = pool.begin().await.unwrap();
    // Invalidate all existing tokens in family
    query!(
        "UPDATE token_family_rel SET status = 'expired' WHERE token_family_id = $1",
        family_id
    )
    .execute(&mut *txn)
    .await?;
    let res = query_as!(
        IdOnly,
        "INSERT INTO token (token) VALUES ($1) returning id",
        token
    )
    .fetch_one(&mut *txn)
    .await?;
    query!(
        "INSERT INTO token_family_rel (token_family_id,status,token_id) VALUES ($1,$2,$3)",
        family_id,
        "ACTIVE",
        res.id
    )
    .execute(&mut *txn)
    .await?;
    txn.commit().await.unwrap();
    return Ok(());
}

#[db_func]
pub async fn add_token(token: &str, old_token: &str) -> Result<(), RefreshRefreshTokenErr> {
    let mut txn = pool.begin().await.unwrap();
    let family_id = query_as!(IdOnly,"SELECT tfr.token_family_id as id FROM token_family_rel tfr JOIN token t on t.id = tfr.token_id WHERE t.token = $1",old_token).fetch_optional(&mut *txn).await?;
    if family_id.is_none() {
        return Err(RefreshRefreshTokenErr::InvalidToken);
    }
    let family_id = family_id.unwrap().id;
    query!(
        "UPDATE token_family_rel SET status = 'expired' WHERE token_family_id = $1",
        family_id
    )
    .execute(&mut *txn)
    .await?;
    let res = query_as!(
        IdOnly,
        "INSERT INTO token (token) VALUES ($1) returning id",
        token
    )
    .fetch_one(&mut *txn)
    .await?;
    query!(
        "INSERT INTO token_family_rel (token_family_id,status,token_id) VALUES ($1,$2,$3)",
        family_id,
        "ACTIVE",
        res.id
    )
    .execute(&mut *txn)
    .await?;
    txn.commit().await.unwrap();
    return Ok(());
}

#[db_func]
pub async fn add_new_token_to_new_family(user_id: i64, token: &str) -> Result<IdOnly, sqlx::Error> {
    let mut txn = pool.begin().await.unwrap();
    let family_id = query_as!(
        IdOnly,
        "INSERT INTO token_family (user_id) VALUES ($1) returning id",
        user_id
    )
    .fetch_one(&mut *txn)
    .await?
    .id;
    query!(
        "UPDATE token_family_rel SET status = 'expired' WHERE token_family_id = $1",
        family_id
    )
    .execute(&mut *txn)
    .await?;
    let res = query_as!(
        IdOnly,
        "INSERT INTO token (token) VALUES ($1) returning id",
        token
    )
    .fetch_one(&mut *txn)
    .await?;
    query!(
        "INSERT INTO token_family_rel (token_family_id,status,token_id) VALUES ($1,$2,$3)",
        family_id,
        "ACTIVE",
        res.id
    )
    .execute(&mut *txn)
    .await?;
    return Ok(res);
}
