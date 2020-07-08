use wasm_bindgen_test::*;

use valerie::prelude::components::*;
use valerie::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

fn ui() -> Node {
    let value = StateMutex::new(String::new());
    let count = StateAtomic::new(0usize);

    div!(
        h3!("Value: ", value.clone()),
        h3!("Length: ", count.clone()),
        input!("text")
            .id("text-field")
            .double_bind(value.clone())
            .bind_func(count.clone(), |x| x.len()),
        br!(),
        input!("text")
            .id("text-field")
            .double_bind(value)
            .bind_func(count, |x| x.len())
    )
    .into()
}

#[wasm_bindgen_test]
pub fn run() {
    App::render_single(ui());
}
