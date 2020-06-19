#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use valerie::html::Tag;
use valerie::{br, button, div, li, p, ul};
use valerie::{App, Component, Page};
use valerie::{StateTrait, StateAtomic, StateVec};
use wasm_bindgen::prelude::*;

struct LaunchPage {
    value: StateAtomic<u128>,
    double: StateAtomic<u128>,
    vec: StateVec<StateAtomic<u128>>,
}

impl Page for LaunchPage {
    fn view(&mut self) -> Tag {
        div!().push_loop(
            |_| {
                div!(
                    p!(
                        "Value ",
                        self.value.clone(),
                        br!(),
                        "Double ",
                        self.double.clone()
                    ),
                    button!("Multiply").on_click(
                        (self.value.clone(), self.vec.clone()),
                        move |(x, y)| {
                            *x *= 2;
                            y.push(x.clone());
                            y.push_atomic(x.value());
                        }
                    ),
                    button!("Divide").on_click(
                        (self.value.clone(), self.vec.clone()),
                        move |(x, y)| {
                            *x /= 2;
                            y.pop();
                            y.pop();
                        }
                    ),
                    self.vec.view(ul!(li!("Test")), |x| li!(x))
                )
            },
            2,
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
