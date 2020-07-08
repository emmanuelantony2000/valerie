use wasm_bindgen_test::*;

use valerie::prelude::components::*;
use valerie::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

fn ui() -> Node {
    let value = StateAtomic::new(0isize);

    div!(
        h1!("Value ", value.clone()),
        button!("Add 1").on_event("click", value.clone(), move |x, _| {
            *x += 1;
        }),
        button!("Subtract 1").on_event("click", value.clone(), move |x, _| {
            *x -= 1;
        })
    )
    .into()
}

#[wasm_bindgen_test]
pub fn run() {
    App::render_single(ui());
}
