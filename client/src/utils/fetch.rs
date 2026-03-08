use reqwest::Error;
use serde::{Deserialize, Serialize};
use shared::routes::auth::refresh::{RefreshRequest, RefreshResponse};

use crate::utils::{localstorage::LocalStorage, router::Router, session::Session};

pub static BASE_URL: &str = "http://localhost:8000";

#[derive(Clone, Copy)]
pub enum ClientModes {
    POST,
    GET,
}

pub enum NetErr {
    Reqwest(Error),
    Refresh,
}

impl From<Error> for NetErr {
    fn from(value: Error) -> Self {
        NetErr::Reqwest(value)
    }
}

impl Into<String> for NetErr {
    fn into(self) -> String {
        match self {
            NetErr::Reqwest(error) => error.to_string(),
            NetErr::Refresh => "Error occurred while refreshing tokens".into(),
        }
    }
}

fn get_client<Body>(
    mode: ClientModes,
    path: &str,
    body: &Option<Body>,
    access_token: Option<String>,
) -> reqwest::blocking::RequestBuilder
where
    Body: Serialize,
{
    let client = match mode {
        ClientModes::POST => reqwest::blocking::Client::new().post(format!("{BASE_URL}{path}")),
        ClientModes::GET => reqwest::blocking::Client::new().get(format!("{BASE_URL}{path}")),
    };

    let client = if let Some(access_token) = access_token {
        client.bearer_auth(access_token)
    } else {
        client
    };

    if let Some(body) = body {
        match mode {
            ClientModes::POST => {
                let req_body = serde_json::to_string(&body).unwrap();
                client.body(req_body)
            }
            ClientModes::GET => client.query(&body),
        }
    } else {
        client
    }
}

pub fn refresh_the_token(refresh_token: Option<String>) -> Result<(), ()> {
    match refresh_token {
        Some(refresh_token) => {
            let res = reqwest::blocking::Client::new()
                .post(format!("{BASE_URL}/auth/refresh"))
                .body(serde_json::to_string(&RefreshRequest { refresh_token }).unwrap())
                .send();

            match res {
                Ok(res) => {
                    let body = res.text();
                    match body {
                        Ok(body) => match serde_json::from_str::<Response<RefreshResponse>>(&body) {
                            Ok(body) => {
                                if body.success{
                                    println!("REFRESH SUCCESS INNER");
                                    Session::set_token(body.data.unwrap());
                                    Ok(())
                                }else{
                                    println!("REFRESH FAIL INNER {}",body.message);
                                    Err(())
                                }
                            }
                            Err(err) => {
                                println!("REFRESH FAIL INNER: {}", err);
                                Err(())
                            }
                        },
                        Err(err) => {
                            println!("REFRESH FAIL INNER: {}", err);
                            Router::set("auth/login");
                            Err(())
                        }
                    }
                }
                Err(err) => {
                    println!("REFRESH FAIL INNER: {}", err);
                    Router::set("auth/login");

                    Err(())
                }
            }
        }
        None => {
            println!("REFRESH FAIL INNER");

            Router::set("auth/login");
            Err(())
        }
    }
}

fn inner_fetch<Body>(
    method: ClientModes,
    path: &str,
    body: &Option<Body>,
    attempt: usize,
) -> Result<reqwest::blocking::Response, NetErr>
where
    Body: Serialize,
{
    let (access_token, refresh_token) = Session::get_tokens();

    let client = get_client(method, &path, body, access_token);

    let res = client.send()?;
    if res.status().as_u16() == 401 {
        if attempt < 3 {
            match refresh_the_token(refresh_token) {
                Ok(_) => inner_fetch(method, path, body, attempt + 1),
                Err(_) => Err(NetErr::Refresh),
            }
        } else {
            Err(NetErr::Refresh)
        }
    } else {
        Ok(res)
    }
}

pub fn public_fetch<Body>(
    method: ClientModes,
    path: &str,
    body: &Option<Body>,
) -> Result<reqwest::blocking::Response, NetErr>
where
    Body: Serialize,
{
    let client = get_client(method, &path, body, None);
    let res = client.send()?;
    Ok(res)
}

pub fn fetch<Body>(
    method: ClientModes,
    path: &str,
    body: &Option<Body>,
) -> Result<reqwest::blocking::Response, NetErr>
where
    Body: Serialize,
{
    inner_fetch(method, path, body, 0)
}

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}
