#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use valerie::html::{button, div, p, br, Tag, ul, li};
use valerie::{State, StateAtomic, StateVec};
use valerie::{App, Component, Page};
use wasm_bindgen::prelude::*;

struct LaunchPage {
    value: StateAtomic<u128>,
    double: StateAtomic<u128>,
    vec: StateVec<StateAtomic<u128>>,
}

impl Page for LaunchPage {
    fn view(&mut self) -> Tag {
        div(&[]).push_loop(
            |_| {
                div(&[
                    p(&[
                        "Value ".view(),
                        self.value.clone().view(),
                        br().view(),
                        "Double ".view(),
                        self.double.clone().view(),
                    ])
                    .view(),
                    button(&["Multiply".view()])
                        .on_click((self.value.clone(), self.vec.clone()), move |(x, y)| {
                            *x *= 2;
                            y.push(x.value());
                            x.update();
                        })
                        .view(),
                    button(&["Divide".view()])
                        .on_click((self.value.clone(), self.vec.clone()), move |(x, y)| {
                            *x /= 2;
                            y.remove(y.len() - 1);
                            x.update();
                        })
                        .view(),
                    self.vec.view(ul(&[]).view(), |x| li(&[x.view()]).view()),
                ])
                .view()
            },
            1000,
        )
    }
}

impl LaunchPage {
    fn new() -> Self {
        let value = StateAtomic::new(1);
        let double = StateAtomic::from(&value, |x| x * 2);
        let vec = StateVec::new();
        Self { value, double, vec }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    App::new()
        .push("hello_world", Box::new(LaunchPage::new()))
        .render();
}
