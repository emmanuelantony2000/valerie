use valerie::prelude::components::*;
use valerie::prelude::*;

fn launch_page() -> web_sys::Node {
    let text = StateMutex::new(String::new());
    let list = StateVec::new();

    div!(
        div!(
            input!("text",)
                .double_bind(text.clone())
                .placeholder("Type something here"),
            button!("Insert").on_event("click", (text.clone(), list.clone()), |(text, list), _| {
                list.push_mutex(text.value());
                text.put(String::new());
            })
        ),
        list.clone().view(ol!(), |x| li!(
            x.clone(),
            button!("Remove").on_event("click", (x, list), |(elem, list), _| {
                // list.clone().remove(
                //     list.clone()
                //         .into_iter()
                //         .position(|y| y.value() == elem.value())
                //         .unwrap(),
                // )
                list.remove_elem(elem.clone()); // Currently not working...
            })
        ))
    )
    .into()
}

#[valerie(start)]
pub fn run() {
    App::new()
        .push("list_add_remove_items", launch_page)
        .render();
}
