use crate::Component;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use futures::channel::mpsc::{Receiver, Sender};
use futures::lock::Mutex;
use futures::stream::StreamExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

#[derive(Default)]
pub struct App {
    routes: BTreeMap<&'static str, Box<dyn Component>>,
    start: Option<&'static str>,
}

impl App {
    pub fn new() -> Self {
        Self {
            routes: BTreeMap::new(),
            start: None,
        }
    }

    pub fn push(&mut self, route: &'static str, component: Box<dyn Component>) -> &mut Self {
        self.routes.insert(route, component);
        self.start = Some(route);
        self
    }

    pub fn start(&mut self, start: &'static str) -> &mut Self {
        self.start = Some(start);
        self
    }

    pub fn render(&mut self) {
        if let Some(x) = body().first_child() {
            body()
                .replace_child(
                    &self.routes.get_mut(self.start.unwrap()).unwrap().view(),
                    &x,
                )
                .unwrap();
        } else {
            body()
                .append_child(&self.routes.get_mut(self.start.unwrap()).unwrap().view())
                .unwrap();
        }

        // wasm_bindgen_futures::spawn_local(self.render_loop(rx));

        // tx.try_send(Some("iter")).unwrap();

        // while rx.try_next().is_err() {}

        // body().append_child(&self.routes.get_mut(self.start.unwrap()).unwrap().view()).unwrap();

        // let closure = Closure::wrap(Box::new(move || self.render(rx)) as Box<dyn FnMut()>);
        // request_animation_frame(&closure);
        // closure.forget();
    }

    // async fn render_loop(mut self, mut rx: Receiver<Option<&'static str>>) {
    //     while let Some(Some(_)) = rx.next().await {
    //         web_sys::window()
    //             .unwrap()
    //             .document()
    //             .unwrap()
    //             .body()
    //             .unwrap()
    //             .replace_child(&self.routes.get_mut(self.start.unwrap()).unwrap().view())
    //             .unwrap();
    //     }
    // }

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
