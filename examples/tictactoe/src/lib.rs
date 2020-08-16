use valerie::prelude::components::*;
use valerie::prelude::*;

fn square(state: StateAtomic<&'static str>, next: StateAtomic<&'static str>) -> Node {
    button!(state.clone())
        .class("square")
        .on_event("click", state.clone(), move |s, _| {
            let old_value = next.value();
            s.put(old_value);
            let new_value = match old_value {
                "X" => "O",
                "O" => "X",
                _ => panic!(),
            };
            next.put(new_value);
        }
    ).into()
}

fn board() -> Node {
    let squares: [StateAtomic<&str>; 9] = [
        // Can't use array init shorthand because StateAtomic not Copy
        StateAtomic::new(""),
        StateAtomic::new(""),
        StateAtomic::new(""),
        StateAtomic::new(""),
        StateAtomic::new(""),
        StateAtomic::new(""),
        StateAtomic::new(""),
        StateAtomic::new(""),
        StateAtomic::new(""),
    ];

    let next_player= StateAtomic::new("X");

    div!(
        div!(
            "Next player: ", next_player.clone()
        ).class("status"),
        div!(
            square(squares[0].clone(), next_player.clone()),
            square(squares[1].clone(), next_player.clone()),
            square(squares[2].clone(), next_player.clone())
        ).class("board-row"),
        div!(
            square(squares[3].clone(), next_player.clone()),
            square(squares[4].clone(), next_player.clone()),
            square(squares[5].clone(), next_player.clone())
        ).class("board-row"),
        div!(
            square(squares[6].clone(), next_player.clone()),
            square(squares[7].clone(), next_player.clone()),
            square(squares[8].clone(), next_player.clone())
        ).class("board-row")
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
