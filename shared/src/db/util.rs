#[cfg(feature = "server")]
pub type SqlJson<T> = sqlx::types::Json<T>;
#[cfg(not(feature = "server"))]
pub type SqlJson<T> = T;