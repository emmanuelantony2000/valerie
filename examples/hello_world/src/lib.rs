#![no_std]

#[macro_use]
extern crate alloc;

use alloc::boxed::Box;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, Ordering};
use futures_intrusive::channel::shared::{state_broadcast_channel, StateReceiver, StateSender};
use valerie::html::{Button, Div, Html, Paragraph};
use valerie::{App, Component, Page};
use wasm_bindgen::prelude::*;

struct LaunchPage {
    value: Arc<AtomicU64>,
    tx: StateSender<()>,
    rx: StateReceiver<()>,
}

impl Page for LaunchPage {
    fn view(&mut self) -> Html {
        Html::new().push_loop(
            |_| {
                let value = Arc::clone(&self.value);
                let tx = self.tx.clone();

                Div::new()
                    .push(vec![
                        Paragraph::new()
                            .push(vec!["<strong>Value </strong>".view()])
                            .value(Arc::clone(&self.value), self.rx.clone())
                            .view(),
                        Button::new()
                            .push(vec!["Click me!".view()])
                            .on_click(Box::new(move || {
                                value.fetch_add(value.load(Ordering::SeqCst), Ordering::SeqCst);
                                while tx.send(()).is_err() {}
                            }))
                            .view(),
                    ])
                    .view()
            },
            1000,
        )
    }
}

impl LaunchPage {
    fn new() -> Self {
        let (tx, rx) = state_broadcast_channel();
        Self {
            value: Arc::new(AtomicU64::new(1)),
            tx,
            rx,
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    App::new()
        .push("hello_world", Box::new(LaunchPage::new()))
        .render();
}
