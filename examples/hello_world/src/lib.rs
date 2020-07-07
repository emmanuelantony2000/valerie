use valerie::prelude::components::*;
use valerie::prelude::*;

fn launch_page() -> Node {
    h1!("Hello, World!").into()
}

#[valerie(start)]
pub fn run() {
    App::render_single(launch_page());
}
