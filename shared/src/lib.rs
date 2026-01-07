pub mod db;
pub mod routes;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug,Clone, Copy)]
pub struct AnyErr(pub ());
impl From<()> for AnyErr {
    fn from(_value: ()) -> Self {
        return AnyErr(());
    }
}


#[cfg(feature = "server")]
type WebBox<T> = rocket::serde::json::Json<T>;

#[cfg(not(feature = "server"))]
type WebBox<T> = T;

#[cfg(feature = "server")]
fn Json<T>(d: T) -> WebBox<T>
where
    T: Serialize + DeserializeOwned,
{
    return rocket::serde::json::Json(d);
}

#[cfg(not(feature = "server"))]
fn Json<T>(d: T) -> WebBox<T>
where
    T: Serialize + DeserializeOwned,
{
    return d;
}



#[cfg(feature = "server")]
#[derive(rocket::response::Responder)]
// #[cfg_attr(feature = "server", derive(rocket::response::Responder))]
pub enum Response<T> {
    #[cfg_attr(feature = "server", response(status = 200))]
    // #[response(status = 200)]
    Success(WebBox<ResponseStruct<T>>),
    #[cfg_attr(feature = "server", response(status = 404))]
    // #[response(status = 404)]
    NotFound(WebBox<ResponseStruct<Option<T>>>),
    #[cfg_attr(feature = "server", response(status = 400))]
    // #[response(status = 400)]
    BadRequest(WebBox<ResponseStruct<Option<T>>>),
    #[cfg_attr(feature = "server", response(status = 500))]
    // #[response(status = 500)]
    InternalError(WebBox<ResponseStruct<Option<T>>>),
    #[cfg_attr(feature = "server", response(status = 401))]
    // #[response(status = 401)]
    Unauthorized(WebBox<ResponseStruct<Option<T>>>),
}


#[cfg(feature = "server")]
impl<T> Response<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn success(message: &str, data: T) -> Self {
        Response::Success(Json(ResponseStruct::new(true, message, data)))
    }
    pub fn not_found(message: &str, data: Option<T>) -> Self {
        Response::NotFound(Json(ResponseStruct::new(false, message, data)))
    }
    pub fn bad_request(message: &str, data: Option<T>) -> Self {
        Response::BadRequest(Json(ResponseStruct::new(false, message, data)))
    }
    pub fn internal_error(message: &str, data: Option<T>) -> Self {
        Response::InternalError(Json(ResponseStruct::new(false, message, data)))
    }
    pub fn unauthorized(message: &str, data: Option<T>) -> Self {
        Response::Unauthorized(Json(ResponseStruct::new(false, message, data)))
    }
}

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize)]
pub struct ResponseStruct<T> {
    pub success: bool,
    pub message: String,
    pub data: T,
}

#[cfg(not(feature = "server"))]
#[derive(Serialize, Deserialize)]
pub struct ResponseStruct<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

#[cfg(feature = "server")]
impl<T> ResponseStruct<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new(success: bool, message: &str, data: T) -> Self {
        return Self {
            success,
            message: message.into(),
            data,
        };
    }
}
