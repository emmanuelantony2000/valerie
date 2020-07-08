use wasm_bindgen_test::*;

use valerie::prelude::components::*;
use valerie::prelude::*;
use wasm_timer::Delay;

wasm_bindgen_test_configure!(run_in_browser);

fn ui() -> Node {
    let timer = StateAtomic::new(0);

    execute(time(1, timer.clone()));
    p!("Seconds passed: ", timer).into()
}

async fn time(n: u64, mut timer: StateAtomic<usize>) {
    while Delay::new(core::time::Duration::from_secs(n)).await.is_ok() {
        timer += 1;
    }
}

#[wasm_bindgen_test]
pub fn run() {
    App::render_single(ui());
}
