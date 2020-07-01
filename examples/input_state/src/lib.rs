use valerie::prelude::components::*;
use valerie::prelude::*;

fn launch_page() -> web_sys::Node {
    let value = StateMutex::new(String::new());
    let count = StateAtomic::new(0);

    div!(
        p!("Value: ", value.clone()),
        p!("Length: ", count.clone()),
        input!("text",)
            .id("text-field")
            .double_bind(value.clone())
            .bind_func(count.clone(), |x| x.len()),
        br!(),
        input!("text",)
            .id("text-field")
            .double_bind(value)
            .bind_func(count, |x| x.len())
    )
    .into()
}

#[valerie(start)]
pub fn run() {
    App::new().push("input_state", launch_page).render();
}
