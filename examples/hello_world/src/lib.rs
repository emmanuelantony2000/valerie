#![no_std]

#[macro_use]
extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering;
use spin::Mutex;
use valerie::html::{Button, Div, Html, Paragraph};
use valerie::App;
use valerie::Element;
use valerie::{Component, Page};
use wasm_bindgen::prelude::*;

struct LaunchPage {
    value: Arc<AtomicU64>,
    // view: Option<Html>,
    app: Arc<Mutex<App>>,
}

impl Page for LaunchPage {
    fn view(&mut self) -> Html {
        // let value = Arc::clone(&self.value);
        // let app = Arc::clone(&self.app);

        Html::new().push_loop(
            |_| {
                let value = Arc::clone(&self.value);
                let app = Arc::clone(&self.app);

                Div::new()
                    .push(&[
                        // Paragraph::new().push(&["Hello, there!!!".view()]).view(),
                        Paragraph::new()
                            .push(&[self.value.load(Ordering::SeqCst).view(), "<strong> hey</strong>".view()])
                            .view(),
                        Button::new()
                            .push(&["Click me!".view()])
                            .on_click(Box::new(move || {
                                value.fetch_add(value.load(Ordering::SeqCst), Ordering::SeqCst);
                                app.lock().render();
                            }))
                            .view(),
                    ])
                    .view()
            },
            100,
        )

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

        // self.view = Some(
        //     Div::new()
        //         .push_loop(
        //             |_| {
        //                 let value = Arc::clone(&self.value);
        //                 let app = Arc::clone(&self.app);

        //                 Div::new()
        //                     .push(&[
        //                         Paragraph::new().value("Hello there!!").into(),
        //                         Paragraph::new()
        //                             .value(self.value.load(Ordering::SeqCst))
        //                             .into(),
        //                         Button::new()
        //                             .value("Click me!")
        //                             .on_click(Box::new(move || {
        //                                 value.fetch_add(
        //                                     value.load(Ordering::SeqCst),
        //                                     Ordering::SeqCst,
        //                                 );
        //                                 app.lock().render();
        //                             }))
        //                             .into(),
        //                     ])
        //                     .into()
        //             },
        //             1000,
        //         )
        //         .into(),
        // );

        // self.view.clone().unwrap()
    }
}

impl LaunchPage {
    fn new(app: Arc<Mutex<App>>) -> Self {
        Self {
            value: Arc::new(AtomicU64::new(1)),
            // view: None,
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
