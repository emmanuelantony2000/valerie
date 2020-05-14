use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use valerie::html::{Button, Div, Paragraph};
use valerie::App;
use valerie::Component;
use wasm_bindgen::prelude::*;

struct LaunchPage {}

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

impl LaunchPage {
    fn view(tx: Sender<&'static str>) -> impl Component {
        Div::view().push(vec![
            Box::new(Paragraph::view().value("Hello World!")),
            Box::new(Button::view().value("Press")),
        ])
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    let (tx, rx) = mpsc::channel();

    App::new()
        .push("hello_world", Box::new(LaunchPage::view(tx.clone())))
        .render(rx);
    
    tx.send("hello").unwrap();
}
