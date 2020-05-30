#![no_std]

#[macro_use]
extern crate alloc;

use alloc::boxed::Box;
use valerie::html::{Button, Div, Html, Paragraph};
use valerie::StateAtomic;
use valerie::{App, Component, Page};
use wasm_bindgen::prelude::*;

struct LaunchPage {
    value: StateAtomic<u128>,
    double: StateAtomic<u128>,
}

impl Page for LaunchPage {
    fn view(&mut self) -> Html {
        Html::new().push_loop(
            |_| {
                Div::new()
                    .push(vec![
                        Paragraph::new()
                            .push(vec!["<strong>Value </strong>".view()])
                            .value(self.value.clone())
                            .view(),
                        Paragraph::new()
                            .push(vec!["<strong>Double </strong>".view()])
                            .value(self.double.clone())
                            .view(),
                        Button::new()
                            .push(vec!["Multiply".view()])
                            .on_click(&self.value, |x| {
                                *x *= 2;
                            })
                            .view(),
                        Button::new()
                            .push(vec!["Divide".view()])
                            .on_click(&self.value, |x| {
                                *x /= 2;
                            })
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
        let value = StateAtomic::new(1);
        let double = StateAtomic::from(&value, |x| x * 2);
        Self { value, double }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    App::new()
        .push("hello_world", Box::new(LaunchPage::new()))
        .render();
}
