use serde::{Deserialize, Serialize, de::DeserializeOwned};
use rocket::serde::json::Json as RocketJson;



#[derive(rocket::response::Responder)]

pub enum Response<T> {

    #[response(status = 200)]
    Success(RocketJson<ResponseStruct<T>>),
    
    #[response(status = 404)]
    NotFound(RocketJson<ResponseStruct<Option<T>>>),
    
    #[response(status = 400)]
    BadRequest(RocketJson<ResponseStruct<Option<T>>>),
    
    #[response(status = 500)]
    InternalError(RocketJson<ResponseStruct<Option<T>>>),
    
    #[response(status = 401)]
    Unauthorized(RocketJson<ResponseStruct<Option<T>>>),
}


impl<T> Response<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn success(message: &str, data: T) -> Self {
        Response::Success(RocketJson(ResponseStruct::new(true, message, data)))
    }
    pub fn not_found(message: &str, data: Option<T>) -> Self {
        Response::NotFound(RocketJson(ResponseStruct::new(false, message, data)))
    }
    pub fn bad_request(message: &str, data: Option<T>) -> Self {
        Response::BadRequest(RocketJson(ResponseStruct::new(false, message, data)))
    }
    pub fn internal_error(message: &str, data: Option<T>) -> Self {
        Response::InternalError(RocketJson(ResponseStruct::new(false, message, data)))
    }
    pub fn unauthorized(message: &str, data: Option<T>) -> Self {
        Response::Unauthorized(RocketJson(ResponseStruct::new(false, message, data)))
    }
}

#[derive(Serialize, Deserialize)]
pub struct ResponseStruct<T> {
    pub success: bool,
    pub message: String,
    pub data: T,
}


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
