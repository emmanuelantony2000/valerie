use valerie::prelude::components::*;
use valerie::prelude::*;

fn launch_page() -> Node {
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

#[valerie(start)]
pub fn run() {
    App::render_single(launch_page());
}
