use valerie::prelude::components::*;
use valerie::prelude::*;

fn square(i: u8) -> Node {
    button!(format!("{}", i)).attr("class", "square").into()
}

fn board() -> Node {
    fn render_square(i: u8) -> Node {
        square(i)
    }

    const STATUS: &str = "Next player: X";

    div!(
        div!(STATUS).attr("class", "status"),
        div!(
           render_square(0),
           render_square(1),
           render_square(2)
        ).attr("class", "board-row"),
        div!(
           render_square(3),
           render_square(4),
           render_square(5)
        ).attr("class", "board-row"),
        div!(
           render_square(6),
           render_square(7),
           render_square(8)
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
