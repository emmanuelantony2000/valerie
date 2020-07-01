use valerie::prelude::components::*;
use valerie::prelude::*;
use wasm_timer::Delay;

fn launch_page() -> web_sys::Node {
    let timer = StateAtomic::new(0);

    execute(time(1, timer.clone()));
    p!("Seconds passed: ", timer).into()
}

async fn time(n: u64, mut timer: StateAtomic<usize>) {
    while Delay::new(core::time::Duration::from_secs(n)).await.is_ok() {
        timer += 1;
    }
}

#[valerie(start)]
pub fn run() {
    App::new().push("time_counter", launch_page).render();
}
