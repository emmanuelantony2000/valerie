use valerie::prelude::components::*;
use valerie::prelude::*;

fn square(_i: u8) -> Node {
    let state = StateMutex::new(" ");
    button!(state.clone())
        .attr("class", "square")
        .on_event("click", state.clone(), move |x, _| {
            x.put("X");
        }
    ).into()
}

fn board() -> Node {
    const STATUS: &str = "Next player: X";

    div!(
        div!(STATUS).attr("class", "status"),
        div!(
           square(0),
           square(1),
           square(2)
        ).attr("class", "board-row"),
        div!(
           square(3),
           square(4),
           square(5)
        ).attr("class", "board-row"),
        div!(
           square(6),
           square(7),
           square(8)
        ).attr("class", "board-row")
    ).into()
}

fn game() -> Node {
    div!(
        div!(
            board()
        ).attr("class", "game-board"),
        div!(
            div!(),
            ol!()
        ).attr("class", "game-info")
    ).attr("class", "game").into()
}

#[valerie(start)]
pub fn run() {
    App::render_single(game());
}
