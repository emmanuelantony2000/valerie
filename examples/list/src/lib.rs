#![no_std]

#[macro_use]
extern crate alloc;

use alloc::boxed::Box;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, Ordering};
use valerie::html::{Button, Div, Html, Paragraph};
use valerie::{channel, StateReceiver, StateSender};
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
                let value1 = self.value();
                let value2 = self.value();
                let tx1 = self.tx();
                let tx2 = self.tx();

                Div::new()
                    .push(vec![
                        Paragraph::new()
                            .push(vec!["<strong>Value </strong>".view()])
                            .value(Arc::clone(&self.value), self.rx.clone())
                            .view(),
                        Button::new()
                            .push(vec!["Multiply".view()])
                            .on_click(Box::new(move || {
                                value1.fetch_add(value1.load(Ordering::SeqCst), Ordering::SeqCst);
                                while tx1.send(()).is_err() {}
                            }))
                            .view(),
                        Button::new()
                            .push(vec!["Divide".view()])
                            .on_click(Box::new(move || {
                                value2.fetch_sub(value2.load(Ordering::SeqCst)/2, Ordering::SeqCst);
                                while tx2.send(()).is_err() {}
                            }))
                            .view(),
                    ])
                    .view()
            },
            100,
        )
    }
}

impl LaunchPage {
    fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            value: Arc::new(AtomicU64::new(1)),
            tx,
            rx,
        }
    }

    fn value(&self) -> Arc<AtomicU64> {
        Arc::clone(&self.value)
    }

    fn tx(&self) -> StateSender<()> {
        self.tx.clone()
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    App::new()
        .push("hello_world", Box::new(LaunchPage::new()))
        .render();
}
