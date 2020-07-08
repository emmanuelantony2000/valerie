use wasm_bindgen_test::*;

use valerie::prelude::components::*;
use valerie::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

fn ui() -> Node {
    h1!("Hello, World!").into()
}

#[wasm_bindgen_test]
pub fn run() {
    App::render_single(ui());
}
