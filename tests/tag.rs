use wasm_bindgen_test::*;

use valerie::prelude::components::*;
use valerie::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

fn new_ui() -> impl Component {
    Tag::<html::elements::Div>::new().push("Hello, World!")
}

fn push_ui() -> impl Component {
    div!().push("Hello, World!")
}

fn push_multiple_ui() -> impl Component {
    div!().push_multiple(vec![Node::from("Hello, "), Node::from("World!")])
}

fn push_loop_ui() -> impl Component {
    div!().push_loop(5, |x| h3!(x))
}

fn on_event_ui() -> impl Component {
    let message = StateMutex::new(String::from("App is running"));
    button!(message.clone())
        .on_event("mouseover", message.clone(), |x, _| {
            x.put("Mouse pointer is in me".to_string())
        })
        .on_event("mouseout", message.clone(), |x, _| {
            x.put("Mouse pointer is outside".to_string())
        })
        .on_event("mousedown", message.clone(), |x, _| {
            x.put("Mouse button pressed".to_string())
        })
}

fn remove_event_ui() -> impl Component {
    let message = StateMutex::new(String::from("App is running"));
    button!(message.clone())
        .on_event("mouseover", message.clone(), |x, _| {
            x.put("Mouse pointer is in me".to_string());
        })
        .on_event("mouseout", message.clone(), |x, _| {
            x.put("Mouse pointer is outside".to_string());
        })
        .on_event("mousedown", message.clone(), |x, t| {
            x.put("Mouse button pressed".to_string());
            t.remove_event("mouseout");
        })
}

fn id_ui() -> impl Component {
    div!("Hello, World!").id("hello-world-id")
}

fn get_id_ui() -> impl Component {
    let heading = h1!("Hello, World!").id("hello-world-id");
    div!(heading.clone(), br!(), "id ", heading.get_id())
}

fn class_ui() -> impl Component {
    div!("Hello, World!").class("heading")
}

fn get_class_ui() -> impl Component {
    let heading = h1!("Hello, World!").class("heading");
    div!(heading.clone(), br!(), "class ", heading.get_class())
}

fn attr_ui() -> impl Component {
    div!("Hello, World!").attr("id", "hello-world")
}

fn get_attr_ui() -> impl Component {
    let heading = h1!("Hello, World!").attr("id", "hello-world");
    div!(
        heading.clone(),
        br!(),
        "attr ",
        heading.get_attr("id").unwrap()
    )
}

fn bind_ui() -> impl Component {
    let state = StateMutex::new(String::new());
    div!(
        state.clone(),
        br!(),
        input!("text").bind(state.clone()),
        input!("text").bind(state)
    )
}

fn double_bind_ui() -> impl Component {
    let state = StateMutex::new(String::new());
    div!(
        state.clone(),
        br!(),
        input!("text").double_bind(state.clone()),
        input!("text").double_bind(state)
    )
}

fn bind_func_ui() -> impl Component {
    let state = StateAtomic::new(0usize);
    div!(
        state.clone(),
        br!(),
        input!("text").bind_func(state, |x| x.len())
    )
}

fn placeholder_ui() -> impl Component {
    input!("text").placeholder("Enter something...")
}

fn ui() -> Node {
    div!(
        "new",
        br!(),
        new_ui(),
        br!(),
        br!(),
        "push",
        br!(),
        push_ui(),
        br!(),
        br!(),
        "push_multiple",
        br!(),
        push_multiple_ui(),
        br!(),
        br!(),
        "push_loop",
        br!(),
        push_loop_ui(),
        br!(),
        br!(),
        "on_event",
        br!(),
        on_event_ui(),
        br!(),
        br!(),
        "remove_event",
        br!(),
        remove_event_ui(),
        br!(),
        br!(),
        "id",
        br!(),
        id_ui(),
        br!(),
        br!(),
        "get_id",
        br!(),
        get_id_ui(),
        br!(),
        br!(),
        "class",
        br!(),
        class_ui(),
        br!(),
        br!(),
        "get_class",
        br!(),
        get_class_ui(),
        br!(),
        br!(),
        "attr",
        br!(),
        attr_ui(),
        br!(),
        br!(),
        "get_attr",
        br!(),
        get_attr_ui(),
        br!(),
        br!(),
        "bind",
        br!(),
        bind_ui(),
        br!(),
        br!(),
        "double_bind",
        br!(),
        double_bind_ui(),
        br!(),
        br!(),
        "bind_func",
        br!(),
        bind_func_ui(),
        br!(),
        br!(),
        "placeholder",
        br!(),
        placeholder_ui(),
        br!(),
        br!()
    )
    .into()
}

#[wasm_bindgen_test]
fn run() {
    App::render_single(ui());
}
