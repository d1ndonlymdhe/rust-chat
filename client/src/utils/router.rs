use std::sync::{Mutex, OnceLock};

use ui::components::{
    common::{Component, Length},
    layout::Layout,
};

pub struct Router {
    current_path: String,
    path_stack: Vec<String>,
    path_changed: bool,
}

impl Router {
    pub fn current_path(&self) -> Vec<String> {
        return self
            .current_path
            .split("/")
            .map(|v| v.into())
            .into_iter()
            .collect();
    }
    pub fn path_changed(&self) -> bool {
        return self.path_changed;
    }
    pub fn reset_path_changed(&mut self) {
        self.path_changed = false;
    }
    pub fn new(init_route: &str) -> Self {
        Self {
            current_path: init_route.into(),
            path_stack: vec![],
            path_changed: true,
        }
    }

    pub fn push(&mut self, new_path: &str) {
        self.path_stack.push(self.current_path.clone());
        self.current_path = new_path.into();
        self.path_changed = true;
    }

    pub fn can_go_back(&self) -> bool {
        return !self.path_stack.is_empty();
    }

    pub fn back(&mut self) {
        self.current_path = match self.path_stack.last() {
            Some(p) => {
                self.path_changed = true;
                p.clone()
            }
            None => panic!("Can't go back use can_go_back to determine"),
        };
    }
}

type LazyComponent = Box<dyn Fn() -> Component>;

pub struct ContainerRoute {
    name: String,
    lazy_component: LazyComponent,
    outlet_id: String,
    sub_routes: Vec<Route>,
    on_mount: Box<dyn Fn() -> ()>,
    on_dismount: Box<dyn Fn() -> ()>,
}

impl ContainerRoute {
    pub fn new(
        name: &str,
        lazy_component: LazyComponent,
        outlet_id: &str,
        sub_routes: Vec<Route>,
        on_mount: Box<dyn Fn() -> ()>,
        on_dismount: Box<dyn Fn() -> ()>,
    ) -> Self {
        return Self {
            name: name.into(),
            lazy_component,
            outlet_id: outlet_id.into(),
            sub_routes,
            on_mount: on_mount,
            on_dismount: on_dismount,
        };
    }
}

pub struct LeafRoute {
    name: String,
    lazy_component: LazyComponent,
    on_mount: Box<dyn Fn() -> ()>,
    on_dismount: Box<dyn Fn() -> ()>,
}

impl LeafRoute {
    pub fn new(
        name: &str,
        on_mount: Box<dyn Fn() -> ()>,
        on_dismount: Box<dyn Fn() -> ()>,
        lazy_component: LazyComponent,
    ) -> Self {
        return Self {
            name: name.into(),
            lazy_component,
            on_mount,
            on_dismount,
        };
    }
}

pub enum Route {
    ContainerRoute(ContainerRoute),
    LeafRoute(LeafRoute),
}

impl Route {
    pub fn container(
        name: &str,
        on_mount: Box<dyn Fn() -> ()>,
        on_dismount: Box<dyn Fn() -> ()>,
        outlet_id: &str,
        lazy_component: LazyComponent,
        sub_routes: Vec<Route>,
    ) -> Self {
        return Route::ContainerRoute(ContainerRoute::new(
            name,
            lazy_component,
            outlet_id,
            sub_routes,
            on_mount,
            on_dismount,
        ));
    }
    pub fn leaf(
        name: &str,
        on_mount: Box<dyn Fn() -> ()>,
        on_dismount: Box<dyn Fn() -> ()>,
        lazy_component: LazyComponent,
    ) -> Self {
        return Route::LeafRoute(LeafRoute::new(name, on_mount, on_dismount, lazy_component));
    }
    pub fn name(&self) -> String {
        match self {
            Route::ContainerRoute(container_route) => container_route.name.clone(),
            Route::LeafRoute(leaf_route) => leaf_route.name.clone(),
        }
    }
    pub fn on_mount(&self) {
        match self {
            Route::ContainerRoute(container_route) => (container_route.on_mount)(),
            Route::LeafRoute(leaf_route) => (leaf_route.on_mount)(),
        }
    }
    pub fn on_dismount(&self) {
        match self {
            Route::ContainerRoute(container_route) => (container_route.on_dismount)(),
            Route::LeafRoute(leaf_route) => (leaf_route.on_dismount)(),
        }
    }
}

pub fn build_route(path: Vec<String>, route: Route, path_changed: bool) -> Component {
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
                    let component = (container_route.lazy_component)();
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
        Route::LeafRoute(leaf_route) => (leaf_route.lazy_component)(),
    }
}

pub fn outlet(id: &str) -> Component {
    Layout::get_col_builder()
        .dim((Length::FILL, Length::FILL))
        .dbg_name(id)
        .build()
}

pub static ROUTER: OnceLock<Mutex<Router>> = OnceLock::new();
