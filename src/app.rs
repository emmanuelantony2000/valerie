use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::function;
// use crate::state;

// pub type Globals = Option<state::StateTrait>;

#[derive(Default)]
pub struct App {
    routes: Vec<(&'static str, Box<dyn Fn() -> web_sys::Node>)>,
    start: Option<&'static str>,
}

impl App {
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {
            routes: Vec::new(),
            start: None,
        }
    }

    pub fn push(
        &mut self,
        route: &'static str,
        component: impl Fn() -> web_sys::Node + 'static,
    ) -> &mut Self {
        if self.routes.iter().any(|x| x.0 == route) {
            panic!("Same routes for two pages");
        }

        self.routes.push((route, Box::new(component)));
        self.start = Some(route);
        self
    }

    pub fn start(&mut self, start: &'static str) -> &mut Self {
        self.start = Some(start);
        self
    }

    pub fn render(&mut self) {
        let route = self
            .routes
            .iter()
            .position(|x| x.0 == self.start.unwrap())
            .unwrap();
        if let Some(x) = function::body().first_child() {
            function::body()
                .replace_child(&self.routes[route].1(), &x)
                .unwrap();
        } else {
            function::body()
                .append_child(&self.routes[route].1())
                .unwrap();
        }
    }

    pub fn render_single(function: web_sys::Node) {
        if let Some(x) = function::body().first_child() {
            function::body().replace_child(&function, &x).unwrap();
        } else {
            function::body().append_child(&function).unwrap();
        }
    }
}
