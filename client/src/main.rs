use reqwest::Error;
use serde::Serialize;
use std::{
    rc::Rc,
    sync::{
        Arc, Mutex, OnceLock,
        mpsc::{self, Receiver, Sender},
    },
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
    login::{TextInputType, as_state, text_input},
};
mod auth;
extern crate ui;

pub static BASE_URL: &'static str = "http://localhost:3000";
pub static ROUTER: OnceLock<Mutex<Router>> = OnceLock::new();

struct Router {
    current_path: String,
    path_stack: Vec<String>,
    path_changed: bool,
}

impl Router {
    fn new() -> Self {
        Self {
            current_path: "".into(),
            path_stack: vec![],
            path_changed: true,
        }
    }
    fn push(&mut self, new_path: &str) {
        self.path_stack.push(self.current_path.clone());
        self.current_path = new_path.into();
        self.path_changed = true;
    }
    fn can_go_back(&self) -> bool {
        return !self.path_stack.is_empty();
    }
    fn back(&mut self) {
        self.current_path = match self.path_stack.last() {
            Some(p) => {
                self.path_changed = true;
                p.clone()
            }
            None => panic!("Can't go back use can_go_back to determine"),
        };
    }
}

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

struct ContainerRoute {
    name: String,
    component: LazyComponent,
    outlet_id: String,
    sub_routes: Vec<Route>,
    on_mount: Box<dyn Fn() -> ()>,
    on_dismount: Box<dyn Fn() -> ()>,
}

impl ContainerRoute {
    fn new(
        name: &str,
        component: LazyComponent,
        outlet_id: &str,
        sub_routes: Vec<Route>,
        on_mount: Box<dyn Fn() -> ()>,
        on_dismount: Box<dyn Fn() -> ()>,
    ) -> Self {
        return Self {
            name: name.into(),
            component,
            outlet_id: outlet_id.into(),
            sub_routes,
            on_mount: on_mount,
            on_dismount: on_dismount,
        };
    }
}

type LazyComponent = Rc<dyn Fn() -> Component>;

struct LeafRoute {
    name: String,
    component: LazyComponent,
    on_mount: Box<dyn Fn() -> ()>,
    on_dismount: Box<dyn Fn() -> ()>,
}

impl LeafRoute {
    fn new(
        name: &str,
        on_mount: Box<dyn Fn() -> ()>,
        on_dismount: Box<dyn Fn() -> ()>,
        component: LazyComponent,
    ) -> Self {
        return Self {
            name: name.into(),
            component,
            on_mount,
            on_dismount,
        };
    }
}

enum Route {
    ContainerRoute(ContainerRoute),
    LeafRoute(LeafRoute),
}

impl Route {
    fn container(
        name: &str,
        component: LazyComponent,
        outlet_id: &str,
        on_mount: Box<dyn Fn() -> ()>,
        on_dismount: Box<dyn Fn() -> ()>,
        sub_routes: Vec<Route>,
    ) -> Self {
        return Route::ContainerRoute(ContainerRoute::new(
            name,
            component,
            outlet_id,
            sub_routes,
            on_mount,
            on_dismount,
        ));
    }
    fn leaf(
        name: &str,
        on_mount: Box<dyn Fn() -> ()>,
        on_dismount: Box<dyn Fn() -> ()>,
        component: LazyComponent,
    ) -> Self {
        return Route::LeafRoute(LeafRoute::new(name, on_mount, on_dismount, component));
    }
    fn name(&self) -> String {
        match self {
            Route::ContainerRoute(container_route) => container_route.name.clone(),
            Route::LeafRoute(leaf_route) => leaf_route.name.clone(),
        }
    }
    fn on_mount(&self) {
        match self {
            Route::ContainerRoute(container_route) => (container_route.on_mount)(),
            Route::LeafRoute(leaf_route) => (leaf_route.on_mount)(),
        }
    }
    fn on_dismount(&self) {
        match self {
            Route::ContainerRoute(container_route) => (container_route.on_dismount)(),
            Route::LeafRoute(leaf_route) => (leaf_route.on_dismount)(),
        }
    }
}

fn build_route(path: Vec<String>, route: Route, path_changed: bool) -> Component {
    match route {
        Route::ContainerRoute(container_route) => {
            let mut path = path;
            let remaining_path = path.split_off(1);
            let next_path = &path[0];
            let next_route = {
                let mut ret_route = None;
                for route in container_route.sub_routes.into_iter() {
                    if &route.name() == next_path {
                        // route.on_mount();
                        if path_changed {
                            route.on_mount();
                        }
                        ret_route = Some(route);
                    } else {
                        route.on_dismount();
                    }
                }
                ret_route
            };
            match next_route {
                Some(r) => {
                    let func = container_route.component.clone();
                    let component = func();
                    let for_borrow = component.clone();
                    let component_binding = for_borrow.borrow_mut();
                    let outlet = component_binding.get_by_id(&container_route.outlet_id);
                    if let Some(outlet) = outlet {
                        let child_component = build_route(remaining_path, r, path_changed);
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

fn no_op() -> Box<dyn Fn() -> ()> {
    return Box::new(|| {});
}

fn main() {
    let ui_rebuild_signal_recv = init_channel();
    ROUTER.set(Mutex::new(Router::new())).ok().unwrap();
    {
        let mut router = ROUTER.get().unwrap().lock().unwrap();
        router.push("auth/login");
    }
    UIRoot::start(
        Box::new(move || {
            let r = Route::container(
                "root",
                Rc::new(|| root()),
                "root_outlet",
                no_op(),
                no_op(),
                vec![Route::container(
                    "auth",
                    Rc::new(|| auth_screen_2()),
                    "auth_outlet",
                    no_op(),
                    no_op(),
                    vec![
                        Route::leaf(
                            "login",
                            Box::new(|| {
                                let mut state = LOGIN_PAGE_STATE.lock().unwrap();
                                state.replace(LoginPageState::new());
                            }),
                            Box::new(|| {
                                let mut state = LOGIN_PAGE_STATE.lock().unwrap();
                                state.take();
                            }),
                            Rc::new(|| login_page()),
                        ),
                        Route::leaf(
                            "signup",
                            Box::new(|| {
                                let mut state = SIGNUP_PAGE_STATE.lock().unwrap();
                                state.replace(LoginPageState::new());
                            }),
                            Box::new(|| {
                                let mut state = SIGNUP_PAGE_STATE.lock().unwrap();
                                state.take();
                            }),
                            Rc::new(|| signup_page()),
                        ),
                    ],
                )],
            );

            let path = {
                let router = ROUTER.get().unwrap().lock().unwrap();
                let current_path = router.current_path.clone();
                current_path
                    .split("/")
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
            };

            let router = &mut ROUTER.get().unwrap().lock().unwrap();
            let path_changed = &mut router.path_changed;

            let c = build_route(path, r, *path_changed);
            *path_changed = false;
            c
        }),
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

struct LoginPageState {
    username: String,
    password: String,
    loading: bool,
    error: Option<String>,
}

impl LoginPageState {
    fn new() -> Self {
        return Self {
            username: "".into(),
            password: "".into(),
            loading: false,
            error: None,
        };
    }
    fn set_password(&mut self, new_password: String) {
        self.password = new_password;
    }
    fn set_username(&mut self, new_username: String) {
        self.username = new_username;
    }
    fn set_loading(&mut self, new_loading: bool) {
        self.loading = new_loading;
    }
    fn set_error(&mut self, new_error: Option<String>) {
        self.error = new_error;
    }
}

static LOGIN_PAGE_STATE: Mutex<Option<LoginPageState>> = Mutex::new(None);

fn login_page() -> Component {
    let email_box = {
        let username = {
            let state = LOGIN_PAGE_STATE.lock().unwrap();
            let state = state.as_ref().unwrap();
            state.username.clone()
        };
        text_input(
            username,
            as_state(move |new_email| {
                let mut state = LOGIN_PAGE_STATE.lock().unwrap();
                let state = state.as_mut().unwrap();
                state.set_username(new_email.into())
            }),
            TextInputType::Text,
        )
    };
    let pass_box = {
        let password = {
            let state = LOGIN_PAGE_STATE.lock().unwrap();
            let state = state.as_ref().unwrap();
            state.password.clone()
        };
        text_input(
            password,
            as_state(move |new_email| {
                let mut state = LOGIN_PAGE_STATE.lock().unwrap();
                let state = state.as_mut().unwrap();
                state.set_password(new_email.into())
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
                let mut router = ROUTER.get().unwrap().lock().unwrap();
                router.push("auth/signup");
                false
            }))
            .build(),
    ];
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

type SignupPageState = LoginPageState;
static SIGNUP_PAGE_STATE: Mutex<Option<SignupPageState>> = Mutex::new(None);

fn signup_page() -> Component {
    let email_box = {
        let username = {
            let state = SIGNUP_PAGE_STATE.lock().unwrap();
            let state = state.as_ref().unwrap();
            state.username.clone()
        };
        text_input(
            username,
            as_state(move |new_email| {
                let mut state = SIGNUP_PAGE_STATE.lock().unwrap();
                let state = state.as_mut().unwrap();
                state.set_username(new_email.into())
            }),
            TextInputType::Text,
        )
    };
    let pass_box = {
        let password = {
            let state = SIGNUP_PAGE_STATE.lock().unwrap();
            let state = state.as_ref().unwrap();
            state.password.clone()
        };
        text_input(
            password,
            as_state(move |new_email| {
                let mut state = SIGNUP_PAGE_STATE.lock().unwrap();
                let state = state.as_mut().unwrap();
                state.set_password(new_email.into())
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
            .dbg_name("Switch Login")
            .on_click(Box::new(move |_| {
                let mut router = ROUTER.get().unwrap().lock().unwrap();
                router.push("auth/login");
                false
            }))
            .build(),
    ];
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
