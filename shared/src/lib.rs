use serde::{Serialize,Deserialize};
#[derive(Serialize,Deserialize)]
pub struct Response<T> where T: Serialize{
    success: bool,
    message: String,
    data: T,
}

impl<T> Response<T> where T:Serialize {
    pub fn new(success: bool, message: &str, data: T) -> Self {
        return Self {
            success,
            message: message.into(),
            data,
        };
    }
}
