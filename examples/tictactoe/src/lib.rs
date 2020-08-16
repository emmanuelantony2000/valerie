use valerie::prelude::components::*;
use valerie::prelude::*;
use valerie::Channel;

use futures_intrusive::channel::{shared::StateReceiver, StateId};
use std::fmt::Display;
use valerie::prelude::wasm_bindgen::__rt::core::fmt::Formatter;

#[derive(Copy, Clone, PartialEq)]
enum Square {
    Empty,
    X,
    O,
}

impl Square {
    pub fn rotate(self) -> Self {
        match self {
            Square::X => Square::O,
            Square::O => Square::X,
            _ => self
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Square::Empty => "",
            Square::X => "X",
            Square::O => "O",
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Status {
    Playing,
    Won,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Status::Playing => "Next player: ",
            Status::Won => "Winner: ",
        };
        write!(f, "{}", s)
    }
}

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
    let squares: [StateAtomic<Square>; 9] = [
        // Can't use array init shorthand because StateAtomic is not Copy
        StateAtomic::new(Square::Empty),
        StateAtomic::new(Square::Empty),
        StateAtomic::new(Square::Empty),
        StateAtomic::new(Square::Empty),
        StateAtomic::new(Square::Empty),
        StateAtomic::new(Square::Empty),
        StateAtomic::new(Square::Empty),
        StateAtomic::new(Square::Empty),
        StateAtomic::new(Square::Empty),
    ];

    let status = StateAtomic::new(Status::Playing);
    let next_player = StateAtomic::new(Square::X);

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

fn square(state: StateAtomic<Square>, status: StateAtomic<Status>, next_player: StateAtomic<Square>) -> Node {
    button!(state.clone())
        .class("square")
        .on_event("click", (), move |_, _| {
            if status.value() == Status::Playing && state.value() == Square::Empty {
                state.put(next_player.value());
            }
        })
        .into()
}

async fn turn_checker(squares: Vec<StateAtomic<Square>>, rx: StateReceiver<Channel>, next_player: StateAtomic<Square>, status: StateAtomic<Status>) {

    fn calculate_winner(squares: &Vec<Square>) -> bool {
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
            if a != Square::Empty && a == b && a == c {
                return true;
            }
        }
        return false;
    }

    let mut old = StateId::new();
    while let Some((new, _)) = rx.receive(old).await {
        let squares: Vec<Square> = squares.iter().map(|s| s.value()).collect();
        if calculate_winner(&squares) {
            status.put(Status::Won);
        } else {
            next_player.put(next_player.value().rotate());
        }
        old = new;
    }
}
