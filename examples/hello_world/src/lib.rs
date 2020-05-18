#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::sync::Arc;
use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering;
use spin::Mutex;
use valerie::html::{Button, Div, Paragraph};
use valerie::App;
use valerie::Component;
use valerie::Element;
use wasm_bindgen::prelude::*;

struct LaunchPage {
    value: Arc<AtomicU64>,
    view: Option<Element>,
    app: Arc<Mutex<App>>,
}

impl Component for LaunchPage {
    fn view(&mut self) -> Element {
        // let value = Arc::clone(&self.value);
        // let app = Arc::clone(&self.app);

        // self.view = Some(
        //     Div::new()
        //         .push(&[
        //             Paragraph::new().value("Hello there!!").into(),
        //             Paragraph::new()
        //                 .value(self.value.load(Ordering::SeqCst).to_string().as_ref())
        //                 .into(),
        //             Button::new()
        //                 .value("Click me!")
        //                 .on_click(Box::new(move || {
        //                     value.fetch_add(value.load(Ordering::SeqCst), Ordering::SeqCst);
        //                     app.lock().render();
        //                 }))
        //                 .into(),
        //         ])
        //         .into(),
        // );

        self.view = Some(
            Div::new()
                .push_loop(
                    |_| {
                        let value = Arc::clone(&self.value);
                        let app = Arc::clone(&self.app);

                        Div::new()
                            .push(&[
                                Paragraph::new().value("Hello there!!").into(),
                                Paragraph::new()
                                    .value(self.value.load(Ordering::SeqCst))
                                    .into(),
                                Button::new()
                                    .value("Click me!")
                                    .on_click(Box::new(move || {
                                        value.fetch_add(
                                            value.load(Ordering::SeqCst),
                                            Ordering::SeqCst,
                                        );
                                        app.lock().render();
                                    }))
                                    .into(),
                            ])
                            .into()
                    },
                    1000,
                )
                .into(),
        );

        self.view.clone().unwrap()
    }
}

impl LaunchPage {
    fn new(app: Arc<Mutex<App>>) -> Self {
        Self {
            value: Arc::new(AtomicU64::new(1)),
            view: None,
            app,
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    let app = Arc::new(Mutex::new(App::new()));

    app.lock()
        .push("hello_world", Box::new(LaunchPage::new(Arc::clone(&app))));
    app.lock().render();
}
