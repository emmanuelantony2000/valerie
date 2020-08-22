#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(iterator_fold_self)]

mod model;

use model::*;

#[macro_use]
mod state;

use state::*;

#[macro_use]
extern crate lazy_static;

use valerie::prelude::components::*;
use valerie::prelude::*;

use std::fmt::{Display, Formatter, Result};
use std::sync::Arc;

#[valerie(start)]
pub fn run() {
    App::render_single(game());
}

fn game() -> Node {
    NextPlayer::set(SquareMark::X);
    execute(GameBoard::turn_checker());
    div!(
        div!(
            GameStatus::formatted(move |s| {
                let s = match s {
                    Status::Playing => "Next player: ",
                    Status::Won => "Winner: ",
                };
                format!("{}: ", s)
            }),
            NextPlayer::display()
        ).class("status"),
        GameBoard::node()
    ).class("game").into()
}

impl GameBoard {
    pub fn node() -> Node {
        let b = Self::get();
        let b = b.squares();
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
        let rx = Self::subscribe();
        let mut old = StateId::new();
        while let Some((new, _)) = rx.receive(old).await {
            if GameBoard::get().calculate_winner() {
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
            use model::{Status::Playing, SquareMark::Empty};

            let status = GameStatus::get();
            let current = Square::get(id).unwrap().mark;
            if status == Playing && current == Empty {
                let mut new_value = *Square::get(id).unwrap();
                new_value.mark = NextPlayer::get();
                Square::update(id, Arc::new(new_value));
                GameBoard::notify();
            }
        })
        .into()
}
