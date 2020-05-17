#![no_std]

extern crate alloc;

// use std::sync::atomic::{AtomicBool, Ordering};
// use std::sync::mpsc::{self, Sender};
// use std::sync::Arc;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::sync::Arc;
use core::ops::Deref;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;
use futures::channel::mpsc;
use futures::channel::mpsc::Sender;
use valerie::html::{Button, Div, Paragraph};
use valerie::App;
use valerie::Component;
use valerie::{Element, Node};
use wasm_bindgen::prelude::*;
use spin::Mutex;

struct LaunchPage {
    value: Arc<AtomicUsize>,
    view: Option<Element>,
    app: Arc<Mutex<App>>,
}

// impl LaunchPage {
//     fn view(tx: Sender<u8>) -> impl Component {
//         let mut button = Arc::new(AtomicBool::new(true));

//         Div::view().push(vec![
//             if button.load(Ordering::SeqCst) {
//                 Box::new(Paragraph::view().value("Hello World!"))
//             } else {
//                 Box::new(Paragraph::view())
//             },
//             Box::new(Button::view().value("Press").on_click(Box::new(|| button = !button), tx)),
//         ])
//     }
// }

// impl Component for LaunchPage {}

// impl Deref for LaunchPage {
//     type Target = Node;

//     fn deref(&self) -> &Self::Target {
//         &self.view()
//     }
// }

// impl From<LaunchPage> for Element {
//     fn from(x: LaunchPage) -> Self {
//         x.view().into()
//     }
// }

// impl Page for LaunchPage {}

impl Component for LaunchPage {
    fn view(&mut self) -> Element {
        let value = Arc::clone(&self.value);
        let app = Arc::clone(&self.app);

        self.view = Some(Div::new()
            .push(&[
                Paragraph::new().value("Hello there!!").into(),
                Paragraph::new()
                    .value(self.value.load(Ordering::SeqCst).to_string().as_ref())
                    .into(),
                Button::new()
                    .value("Click me!")
                    .on_click(Box::new(move || {
                        value.fetch_add(1, Ordering::SeqCst);
                        app.lock().render();
                    }))
                    .into(),
            ])
            .into());

        self.view.clone().unwrap()
    }
}

impl Deref for LaunchPage {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.view.as_ref().unwrap()
    }
}

impl LaunchPage {
    fn new(app: Arc<Mutex<App>>) -> Self {
        Self {
            value: Arc::new(AtomicUsize::new(0)),
            view: None,
            app,
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    // let (tx, rx) = mpsc::channel(10);

    // while rx.try_next().is_err() {}
    let app = Arc::new(Mutex::new(App::new()));
    app.lock().push("hello_world", Box::new(LaunchPage::new(Arc::clone(&app))));
    app.lock().render();
        // .render();
}
