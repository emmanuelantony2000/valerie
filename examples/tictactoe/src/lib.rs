#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(iterator_fold_self)]

mod state;

#[macro_use]
extern crate lazy_static;

use valerie::prelude::components::*;
use valerie::prelude::*;

use futures_intrusive::channel::{shared::StateSender, shared::StateReceiver, StateId};

use std::fmt::{Display, Formatter, Result};
use std::collections::HashMap;
use std::sync::Mutex;
use std::hash::Hash;
use std::cmp::Eq;
use std::sync::Arc;

use state::*;
use futures_intrusive::channel::shared::state_broadcast_channel;

// Model

#[derive(Copy, Clone, PartialEq)]
enum SquareMark {
    Empty,
    X,
    O,
}

impl Default for SquareMark {
    fn default() -> Self { Self::Empty }
}

impl SquareMark {
    pub fn next(self) -> Self {
        match self {
            SquareMark::X => SquareMark::O,
            SquareMark::O => SquareMark::X,
            _ => self
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct SquareID(u8);

#[derive(Copy, Clone)]
struct Square {
    _id: SquareID,
    mark: SquareMark,
}

// Square::get(SquareID) -> (ArcSquare<Square>, Ready)
relation!(Square, SquareID, Arc<Square>);

struct BoardType {
    board: [SquareID; 9],
}

impl Default for BoardType {
    fn default() -> Self {
        let mut board = [SquareID(0); 9];
        for i in 0usize..9 {
            board[i] = SquareID(i as u8);
        }
        Self { board }
    }
}

impl BoardType {
    fn calculate_winner(&self) -> bool {
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
        LINES.iter().map(|win| {
            win.iter().map(|id| {
                Square::get(self.board[*id]).unwrap().mark
            })
        }).any(|win| {
            use SquareMark::*;
            win.fold_first(|a, b| {
                match a {
                    Empty => Empty,
                    _ => if a == b { a } else { Empty },
                }
            }).unwrap() != Empty
        })
    }
}

// Board::get() -> ArcSquare<BoardType>
singleton!(Board, "board", Arc<BoardType>);

#[derive(Copy, Clone, PartialEq)]
enum Status {
    Playing,
    Won,
}

impl Default for Status {
    fn default() -> Self { Self::Playing }
}

singleton!(GameStatus, "game", Status);
singleton!(NextPlayer, "next", SquareMark);

// Model-View

#[valerie(start)]
pub fn run() {
    App::render_single(game());
}

fn game() -> Node {
    NextPlayer::set(SquareMark::X);
    execute(Board::turn_checker());
    div!(
        div!(
            GameStatus::formatted(move |s| {
                let s = match s {
                    Status::Playing => "Next player: ",
                    Status::Won => "Winner: ",
                };
                format!("{}: ", s)
            }),
            NextPlayer::formatted(move |p| { format!("{}", p)})
        ).class("status"),
        Board::node()
    ).class("game")
        .into()
}

impl Board {
    pub fn node() -> Node {
        let b = Board::get().board;
        div!(
            div!(
                square(b[0]),
                square(b[1]),
                square(b[2])
            ).class("board-row"),
            div!(
                square(b[3]),
                square(b[4]),
                square(b[5])
            ).class("board-row"),
            div!(
                square(b[6]),
                square(b[7]),
                square(b[8])
            ).class("board-row")
        ).into()
    }

    pub async fn turn_checker() {
        let rx = Board::subscribe();
        let mut old = StateId::new();
        while let Some((new, _)) = rx.receive(old).await {
            if Board::get().calculate_winner() {
                GameStatus::set(Status::Won);
            } else {
                NextPlayer::set(NextPlayer::get().next());
            }
            old = new;
        }
    }

}

impl Display for SquareMark {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            Self::Empty => "",
            Self::X => "X",
            Self::O => "O",
        };
        write!(f, "{}", s)
    }
}

fn square(id: SquareID) -> Node {
    button!(Square::display(id))
        .class("square")
        .on_event("click", (), move |_, _| {
            let status = GameStatus::get();
            let current = Square::get(id).unwrap().mark;
            if status == Status::Playing && current == SquareMark::Empty {
                let mut new_value = *Square::get(id).unwrap();
                new_value.mark = NextPlayer::get();
                Square::update(id, Arc::new(new_value));
                Board::notify();
            }
        })
        .into()
}
