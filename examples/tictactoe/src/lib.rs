#![allow(incomplete_features)]
#![feature(iterator_fold_self)]
#![feature(async_closure)]

#[macro_use]
mod fetch;
mod model;
#[macro_use]
mod store;
mod view;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

extern crate console_error_panic_hook;

use valerie::prelude::*;

use std::panic;

#[valerie(start)]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    info!("run");
    App::render_single(view::game());
}
