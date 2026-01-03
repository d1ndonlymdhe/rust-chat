use reqwest::Error;
use serde::{Serialize, de::DeserializeOwned};
use shared::routes::auth::refresh::RefreshRequest;
use std::{
    cell::RefCell,
    fmt::format,
    ops::Deref,
    rc::Rc,
    sync::{
        Arc, Mutex, Once, OnceLock,
        mpsc::{self, Receiver, Sender},
    },
    thread,
};
use ui::{
    components::{
        common::{Alignment, Component, Length},
        layout::Layout,
        root::UIRoot,
        text_layout::TextLayout,
    },
    raylib::color::Color,
};

use crate::auth::{
    auth_screen,
    login::{AUTH_STATE, AuthState, TextInputType, as_state, text_input},
};
mod auth;
extern crate ui;

pub static BASE_URL: &'static str = "http://localhost:3000";

pub struct HttpClient {
    access_token: Option<String>,
    refresh_token: Option<String>,
}

fn get_tokens() -> (Option<String>, Option<String>) {
    let http_client = &HTTP_CLIENT.get().unwrap().lock().unwrap();
    (
        http_client.access_token.clone(),
        http_client.refresh_token.clone(),
    )
}

enum ClientModes {
    POST,
    GET,
}

fn get_client<Body>(
    mode: ClientModes,
    path: &str,
    body: Option<Body>,
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

    let client = if let Some(body) = body {
        match mode {
            ClientModes::POST => {
                let req_body = serde_json::to_string(&body).unwrap();
                client.body(req_body)
            }
            ClientModes::GET => client.query(&body),
        }
    } else {
        client
    };
    client
}

// fn get_refresh_client(){
//     let client = reqwest::blocking::Client::new().post(format!("{BASE_URL}/refresh"));
//     let mut lock = HTTP_CLIENT.get().unwrap().lock().unwrap();
//     let refresh_token = lock.refresh_token.clone();
//     if let Some(refresh_token) = refresh_token {
//         let client = client.body(serde_json::from_value(RefreshRequest{
//             refresh_token:
//         }))
//     }else{
//         let x = AUTH_STATE.lock().unwrap();

//     }
// }

pub fn post<Body>(path: &str, body: Option<Body>) -> Result<reqwest::blocking::Response, Error>
where
    Body: Serialize,
{
    let (access_token, refresh_token) = get_tokens();

    let client = get_client(ClientModes::POST, &path, body, access_token);

    let res = client.send()?;
    if res.status().as_u16() == 401 {
        println!("UNAUTHORIZED ATTEMPTING REFRESH");
    }
    return Ok(res);
}

fn init_http_client() {
    let client = reqwest::blocking::Client::new();
    let client = Arc::new(Mutex::new(HttpClient {
        access_token: None,
        refresh_token: None,
    }));
    HTTP_CLIENT.set(client).ok().unwrap();
}

pub static HTTP_CLIENT: OnceLock<Arc<Mutex<HttpClient>>> = OnceLock::new();

pub static UI_REBUILD_SIGNAL_SEND: OnceLock<Sender<()>> = OnceLock::new();

fn init_channel() -> Receiver<()> {
    let (tx, rx) = mpsc::channel();
    UI_REBUILD_SIGNAL_SEND.set(tx).ok().unwrap();
    rx
}


#[derive(Clone)]
struct ContainerRoute {
    name: String,
    component: LazyComponent,
    outlet_id: String,
    sub_routes: Vec<Route>,
}

impl ContainerRoute {
    fn new(
        name: &str,
        component: LazyComponent,
        outlet_id: &str,
        sub_routes: Vec<Route>,
    ) -> Self {
        return Self {
            name: name.into(),
            component,
            outlet_id: outlet_id.into(),
            sub_routes,
        };
    }
}

type LazyComponent = Rc<dyn Fn() -> Component>;
#[derive(Clone)]
struct LeafRoute {
    name: String,
    component: LazyComponent,
}

impl LeafRoute {
    fn new(name: &str, component: LazyComponent) -> Self {
        return Self {
            name: name.into(),
            component,
        };
    }
}

#[derive(Clone)]
enum Route {
    ContainerRoute(ContainerRoute),
    LeafRoute(LeafRoute),
}

impl Route {
    fn container(
        name: &str,
        component: LazyComponent,
        outlet_id: &str,
        sub_routes: Vec<Route>,
    ) -> Self {
        return Route::ContainerRoute(ContainerRoute::new(
            name, component, outlet_id, sub_routes,
        ));
    }
    fn leaf(name: &str, component: LazyComponent) -> Self {
        return Route::LeafRoute(LeafRoute::new(name, component));
    }
    fn name(&self) -> String {
        match self {
            Route::ContainerRoute(container_route) => container_route.name.clone(),
            Route::LeafRoute(leaf_route) => leaf_route.name.clone(),
        }
    }
}

fn build_route(path: Vec<String>, route: Route) -> Component {
    match route {
        Route::ContainerRoute(container_route) => {
            let mut path = path;
            let remaining_path = path.split_off(1);
            let next_path = &path[0];
            println!("Next path = {next_path}");
            let next_route = container_route
                .sub_routes
                .into_iter()
                .find(|v| v.name() == *next_path);
            match next_route {
                Some(r) => {
                    println!("Filled by = {}", r.name());
                    let func = container_route.component.clone();
                    let component = func();
                    let for_borrow = component.clone();
                    let component_binding = for_borrow.borrow_mut();
                    let outlet = component_binding.get_by_id(&container_route.outlet_id);
                    if let Some(outlet) = outlet {
                        let child_component = build_route(remaining_path, r);
                        outlet.borrow_mut().set_children(vec![child_component]);
                        return component;
                    } else {
                        panic!("Outlet with ID {} not found", container_route.outlet_id)
                    }
                }
                None => {
                    panic!("NO MATCHING ROUTE FOUND")
                }
            }
        }
        Route::LeafRoute(leaf_route) => leaf_route.component.clone()(),
    }
}



fn main() {
    let ui_rebuild_signal_recv = init_channel();

    let r = Route::container(
        "root",
        Rc::new(|| root()),
        "root_outlet",
        vec![Route::container(
            "auth",
            Rc::new(|| auth_screen_2()),
            "auth_outlet",
            vec![
                Route::leaf("login", Rc::new(|| login_page())),
                Route::leaf("signup", Rc::new(|| signup_page())),
            ],
        )],
    );

    UIRoot::start(
        Box::new(move || build_route(vec!["auth".into(), "login".into()], r.clone())),
        (1920, 1000),
        "Hello from lib",
        ui_rebuild_signal_recv,
    );
}

fn outlet(id: &str) -> Component {
    Layout::get_col_builder()
        .dim((Length::FILL, Length::FILL))
        .dbg_name(id)
        .build()
}

fn root() -> Component {
    Layout::get_row_builder()
        .bg_color(Color::WHEAT)
        .main_align(Alignment::Center)
        .children(vec![
            Layout::get_col_builder()
                .dim((Length::FillPer(50), Length::FILL))
                .children(vec![outlet("root_outlet")])
                .build(),
        ])
        .build()
}

fn auth_screen_2() -> Component {
    Layout::get_row_builder()
        .dim((Length::FILL, Length::FILL))
        .main_align(Alignment::Center)
        .children(vec![
            Layout::get_col_builder()
                .dim((Length::FillPer(60), Length::FILL))
                .children(vec![outlet("auth_outlet")])
                .build(),
        ])
        .build()
}

fn login_page() -> Component {
    let email_box = {
        // let login_params = {
        //     let state = AUTH_STATE.lock().unwrap();
        //     state.get_login_params()
        // };
        text_input(
            "".into(),
            // login_params.0,
            as_state(move |new_email| {
                // let mut state = AUTH_STATE.lock().unwrap();
                // state.set_login_state(new_email, &login_params.1);
            }),
            TextInputType::Text,
        )
    };
    let pass_box = {
        // let login_params = {
        //     let state = AUTH_STATE.lock().unwrap();
        //     state.get_login_params()
        // };
        text_input(
            "".into(),
            // login_params.0,
            as_state(move |new_email| {
                // let mut state = AUTH_STATE.lock().unwrap();
                // state.set_login_state(new_email, &login_params.1);
            }),
            TextInputType::Text,
        )
    };

    let form_children = vec![
        TextLayout::get_builder()
            .dim((Length::FILL, Length::FIT))
            .content("Email: ")
            .build(),
        email_box,
        TextLayout::get_builder()
            .dim((Length::FILL, Length::FIT))
            .content("Password: ")
            .build(),
        pass_box,
        TextLayout::get_builder()
            .padding((5, 5, 5, 5))
            .content("Continue")
            .on_click(Box::new(|_| {
                // execute_login();
                false
            }))
            .bg_color(Color::BEIGE)
            .build(),
        TextLayout::get_builder()
            .padding((5, 5, 5, 5))
            .bg_color(Color::BEIGE)
            .dim((Length::FIT, Length::FIT))
            .wrap(false)
            .content("Signup Instead")
            .dbg_name("SwitchSignup")
            .on_click(Box::new(move |_| {
                // let mut state = AUTH_STATE.lock().unwrap();
                // if !loading {
                //     state.toggle_active_screen();
                // }
                false
            }))
            .build(),
    ];
    // if let Some(err) = error.clone() {
    //     let err_msg = format!("Error: {}", err);
    //     form_children.push(TextLayout::get_builder().content(&err_msg).build());
    // }
    // if loading {
    //     form_children.push(
    //         TextLayout::get_builder()
    //             .content(if loading { "LOADING" } else { "NOT LOADING" })
    //             .build(),
    //     )
    // }
    return Layout::get_col_builder()
        .dim((Length::FILL, Length::FILL))
        .bg_color(Color::RED)
        .flex(9.5)
        .cross_align(Alignment::Center)
        .padding((10, 10, 10, 10))
        .gap(30)
        .children(vec![
            TextLayout::get_builder()
                .content("Login")
                .font_size(40)
                .build(),
            Layout::get_col_builder()
                .gap(10)
                .cross_align(Alignment::Center)
                .children(form_children)
                .build(),
        ])
        .build();
}

fn signup_page() -> Component {
    let email_box = {
        // let login_params = {
        //     let state = AUTH_STATE.lock().unwrap();
        //     state.get_login_params()
        // };
        text_input(
            "".into(),
            // login_params.0,
            as_state(move |new_email| {
                // let mut state = AUTH_STATE.lock().unwrap();
                // state.set_login_state(new_email, &login_params.1);
            }),
            TextInputType::Text,
        )
    };
    let pass_box = {
        // let login_params = {
        //     let state = AUTH_STATE.lock().unwrap();
        //     state.get_login_params()
        // };
        text_input(
            "".into(),
            // login_params.0,
            as_state(move |new_email| {
                // let mut state = AUTH_STATE.lock().unwrap();
                // state.set_login_state(new_email, &login_params.1);
            }),
            TextInputType::Text,
        )
    };

    let form_children = vec![
        TextLayout::get_builder()
            .dim((Length::FILL, Length::FIT))
            .content("Email: ")
            .build(),
        email_box,
        TextLayout::get_builder()
            .dim((Length::FILL, Length::FIT))
            .content("Password: ")
            .build(),
        pass_box,
        TextLayout::get_builder()
            .padding((5, 5, 5, 5))
            .content("Continue")
            .on_click(Box::new(|_| {
                // execute_login();
                false
            }))
            .bg_color(Color::BEIGE)
            .build(),
        TextLayout::get_builder()
            .padding((5, 5, 5, 5))
            .bg_color(Color::BEIGE)
            .dim((Length::FIT, Length::FIT))
            .wrap(false)
            .content("Login Instead")
            .dbg_name("SwitchSignup")
            .on_click(Box::new(move |_| {
                // let mut state = AUTH_STATE.lock().unwrap();
                // if !loading {
                //     state.toggle_active_screen();
                // }
                false
            }))
            .build(),
    ];
    // if let Some(err) = error.clone() {
    //     let err_msg = format!("Error: {}", err);
    //     form_children.push(TextLayout::get_builder().content(&err_msg).build());
    // }
    // if loading {
    //     form_children.push(
    //         TextLayout::get_builder()
    //             .content(if loading { "LOADING" } else { "NOT LOADING" })
    //             .build(),
    //     )
    // }
    return Layout::get_col_builder()
        .dim((Length::FILL, Length::FILL))
        .bg_color(Color::RED)
        .flex(9.5)
        .cross_align(Alignment::Center)
        .padding((10, 10, 10, 10))
        .gap(30)
        .children(vec![
            TextLayout::get_builder()
                .content("Signup")
                .font_size(40)
                .build(),
            Layout::get_col_builder()
                .gap(10)
                .cross_align(Alignment::Center)
                .children(form_children)
                .build(),
        ])
        .build();
}

fn ui_builder() -> Component {
    let auth_screen = auth_screen();

    return Layout::get_row_builder()
        .bg_color(Color::WHEAT)
        .main_align(Alignment::Center)
        .children(vec![
            Layout::get_col_builder()
                .dim((Length::FillPer(50), Length::FILL))
                .children(vec![auth_screen])
                .build(),
        ])
        .build();
}
