#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(iterator_fold_self)]
#![feature(async_closure)]

mod model;

use model::*;

#[macro_use]
mod state;

use state::*;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

extern crate console_error_panic_hook;

use valerie::prelude::components::*;
use valerie::prelude::*;

use std::fmt::{Display, Formatter, Result};
use std::panic;

#[valerie(start)]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    info!("run");
    App::render_single(game());
}

fn game() -> Node {
    info!("game");
    NextPlayer::mutate(NextPlayerChange::Start);
    execute(GameBoard::turn_checker());
    div!(
        div!(
            div!(
                GameStatus::formatted(move |s| {
                    debug!("GS");
                    let s = match s {
                        Status::Playing => "Next player: ",
                        Status::Won => "Winner: ",
                    };
                    format!("{}", s)
                }),
                NextPlayer::formatted(move |p| { debug!("NP"); format!("{}", p) })
            ).class("status"),
            GameBoard::node()
        ).class("game-board")
    ).class("game").into()
}

impl GameBoard {
    pub fn node() -> Node {
        info!("board");
        let board = Self::get();
        let board = &board.squares;
        debug!("board length: {}", board.len());
        let mut parent = div!();
        for row in board.chunks(3) {
            info!("row length: {}", row.len());
            let mut row_div: Tag<html::elements::Div> = div!().class("board-row");
            for id in row {
                info!("square");
                row_div = row_div.push(square(*id));
            }
            parent = parent.push(row_div);
        }
        parent.into()
    }

    pub async fn turn_checker() {
        info!("turn_checker");
        let rx = Self::subscribe();
        let mut old = StateId::new();
        info!("ready");
        while let Some((new, _)) = rx.receive(old).await {
            info!("turn");
            if GameBoard::get().calculate_winner() {
                GameStatus::mutate(StatusChange::Won);
            } else {
                NextPlayer::mutate(NextPlayerChange::Next);
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
    info!("square");
    button!(Square::formatted(id, |s| { debug!("Sq"); format!("{}", s.mark)}))
        .class("square")
        .on_event("click", (), move |_, _| {
            use model::{Status::Playing, SquareMark::Empty};
            let status = GameStatus::get();
            let current = Square::get(id).unwrap().mark;
            if status == Playing && current == Empty {
                Square::mutate(id, SquareChange::Mark(NextPlayer::get()));
                GameBoard::notify();
            }
        })
        .into()
}
