use wasm_bindgen_test::*;

use valerie::prelude::components::*;
use valerie::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

fn ui() -> Node {
    let text = StateMutex::new(String::new());
    let list = StateVec::new();

    div!(
        div!(
            input!("text")
                .double_bind(text.clone())
                .placeholder("Type something here"),
            button!("Insert").on_event("click", (text.clone(), list.clone()), |(text, list), _| {
                list.push_mutex(text.value());
                text.put(String::new());
            })
        ),
        list.clone().view(ul!(), |x| list_item(x, list).into())
    )
    .into()
}

fn list_item(x: StateMutex<String>, list: StateVec<StateMutex<String>>) -> impl Component {
    li!(
        x.clone(),
        button!("Remove").on_event("click", (x, list), |(x, list), _| {
            list.remove_elem(x.clone());
        })
    )
}

#[wasm_bindgen_test]
pub fn run() {
    App::render_single(ui());
}
