use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::function;
// use crate::state;

// pub type Globals = Option<state::StateTrait>;

struct Route(&'static str, Box<dyn Fn() -> crate::Node>);

/// The `App` struct for creating an App
///
/// *Routing is not currently implemented.*
#[derive(Default)]
pub struct App {
    routes: Vec<Route>,
    start: Option<&'static str>,
}

impl App {
    /// Create a new `App` instance.
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {
            routes: Vec::new(),
            start: None,
        }
    }

    /// Push routes inside the `App` struct.
    pub fn push(
        &mut self,
        route: &'static str,
        component: impl Fn() -> crate::Node + 'static,
    ) -> &mut Self {
        if self.routes.iter().any(|x| x.0 == route) {
            panic!("Same routes for two pages");
        }

        self.routes.push(Route(route, Box::new(component)));
        self.start = Some(route);
        self
    }

    /// Specify the starting route.
    pub fn start(&mut self, start: &'static str) -> &mut Self {
        self.start = Some(start);
        self
    }

    /// Render the `App`.
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

    /// Render a single page.
    ///
    /// # Examples
    ///
    /// ```
    /// # use valerie::prelude::*;
    /// # use valerie::prelude::components::*;
    /// # use wasm_bindgen_test::*;
    /// fn ui() -> Node {
    ///     h1!("Hello, World!")
    ///     .into()
    /// }
    /// # wasm_bindgen_test_configure!(run_in_browser);
    /// # #[wasm_bindgen_test]
    /// fn run() {
    ///     App::render_single(ui());
    /// }
    /// ```
    pub fn render_single(function: crate::Node) {
        if let Some(x) = function::body().first_child() {
            function::body().replace_child(&function, &x).unwrap();
        } else {
            function::body().append_child(&function).unwrap();
        }
    }
}
