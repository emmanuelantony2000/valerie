use valerie::prelude::components::*;
use valerie::prelude::*;
use valerie::Channel;

use futures_intrusive::channel::{shared::StateReceiver, StateId};

#[valerie(start)]
pub fn run() {
    App::render_single(game());
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

fn board() -> Node {
    let squares: [StateAtomic<&str>; 9] = [
        // Can't use array init shorthand because StateAtomic is not Copy
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

    let status = StateAtomic::new("Next player: ");
    let next_player = StateAtomic::new("X");

    for square in squares.iter() {
        wasm_bindgen_futures::spawn_local(turn_checker(squares.to_vec(), square.rx(), next_player.clone(), status.clone()));
    }

    div!(
        div!(
            status.clone(),
            next_player.clone()
        ).class("status"),
        div!(
            square(squares[0].clone(), status.clone(), next_player.clone()),
            square(squares[1].clone(), status.clone(), next_player.clone()),
            square(squares[2].clone(), status.clone(), next_player.clone())
        ).class("board-row"),
        div!(
            square(squares[3].clone(), status.clone(), next_player.clone()),
            square(squares[4].clone(), status.clone(), next_player.clone()),
            square(squares[5].clone(), status.clone(), next_player.clone())
        ).class("board-row"),
        div!(
            square(squares[6].clone(), status.clone(), next_player.clone()),
            square(squares[7].clone(), status.clone(), next_player.clone()),
            square(squares[8].clone(), status.clone(), next_player.clone())
        ).class("board-row")
    ).into()
}

fn square(state: StateAtomic<&'static str>, status: StateAtomic<&'static str>, next_player: StateAtomic<&'static str>) -> Node {
    button!(state.clone())
        .class("square")
        .on_event("click", (), move |_, _| {
            if status.value() == "Next player: " && state.value() == "" {
                state.put(next_player.value());
            }
        })
        .into()
}

async fn turn_checker(squares: Vec<StateAtomic<&'static str>>, rx: StateReceiver<Channel>, next_player: StateAtomic<&'static str>, status: StateAtomic<&'static str>) {

    fn calculate_winner(squares: &Vec<&str>) -> bool {
        const LINES: [[usize; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        for [a, b, c] in &LINES {
            let a = squares[*a];
            let b = squares[*b];
            let c = squares[*c];
            if a != "" && a == b && a == c {
                return true;
            }
        }
        return false;
    }

    let mut old = StateId::new();
    while let Some((new, _)) = rx.receive(old).await {
        let squares: Vec<&str> = squares.iter().map(|s| s.value()).collect();
        if calculate_winner(&squares) {
            status.put("Winner: ");
        } else {
            next_player.put(match next_player.value() {
                "X" => "O",
                "O" => "X",
                _ => panic!("Unexpected player"),
            });
        }
        old = new;
    }
}
