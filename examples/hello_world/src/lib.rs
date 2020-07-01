use valerie::prelude::components::*;
use valerie::prelude::*;

fn launch_page() -> web_sys::Node {
    h1!("Hello World").into()
}

#[valerie(start)]
pub fn run() {
    App::new().push("hello_world", launch_page).render();
}
