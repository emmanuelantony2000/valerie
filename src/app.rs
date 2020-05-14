use crate::Component;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Default)]
pub struct App {
    routes: HashMap<&'static str, Box<dyn Component>>,
    start: Option<&'static str>,
}

impl App {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            start: None,
        }
    }

    pub fn push(mut self, route: &'static str, component: Box<dyn Component>) -> Self {
        self.routes.insert(route, component);
        self.start = Some(route);
        self
    }

    pub fn start(mut self, start: &'static str) -> Self {
        self.start = Some(start);
        self
    }

    pub fn render(self) {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(self.routes.get(self.start.unwrap()).unwrap())
            .unwrap();
    }

    // pub fn render(self, rx: Receiver<&'static str>) {
    //     let f = Arc::new(RefCell::new(None));
    //     let g = Arc::clone(&f);

    //     *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {

    //     })));

    //     // wasm_bindgen_futures::spawn_local(async move {
    //     //     let mut page = self.routes.get(self.start.unwrap()).unwrap();

    //     //     while let Ok(x) = rx.recv() {
    //     //         web_sys::window()
    //     //             .unwrap()
    //     //             .document()
    //     //             .unwrap()
    //     //             .body()
    //     //             .unwrap()
    //     //             .replace_child(self.routes.get(self.start.unwrap()).unwrap(), page)
    //     //             .unwrap();

    //     //         page = self.routes.get(self.start.unwrap()).unwrap();
    //     //     }
    //     // });
    // }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("No global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("Document should have a body")
}
