use wasm_bindgen_test::*;

use valerie::prelude::components::*;
use valerie::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

fn state_atomic_ui() -> impl Component {
    let counter = StateAtomic::new(1);
    let double_counter = StateAtomic::from(&counter, |x| x * 2);

    div!(
        h3!("Single count ", counter),
        h3!("Double count ", double_counter)
    )
}

fn state_mutex_ui() -> impl Component {
    let string = StateMutex::new(String::from("This is a test."));
    let length = StateMutex::from(&string, |x| {
        let mut result = String::from("The length is ");
        result.push_str(&x.len().to_string());
        result
    });

    div!(h3!("String: ", string), h3!(length))
}

fn state_vec_ui() -> impl Component {
    let vec = StateVec::with_capacity(10);
    (0..10).for_each(|x| vec.insert(x, StateAtomic::new(x)));

    div!(
        vec.view(ul!(), |x| li!(x)),
        br!(),
        vec.view(ol!(), |x| li!("Element ", x))
    )
}

fn ui() -> Node {
    div!(
        "StateAtomic from",
        br!(),
        state_atomic_ui(),
        br!(),
        br!(),
        "StateMutex from",
        br!(),
        state_mutex_ui(),
        br!(),
        br!(),
        "StateVec",
        br!(),
        state_vec_ui(),
        br!(),
        br!()
    )
    .into()
}

#[wasm_bindgen_test]
fn run() {
    App::render_single(ui());
}
