use valerie::prelude::components::*;
use valerie::prelude::*;

fn launch_page() -> Node {
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

#[valerie(start)]
pub fn run() {
    App::render_single(launch_page());
}
