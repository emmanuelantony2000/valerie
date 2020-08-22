use std::collections::HashMap;
use std::sync::Arc;

use crate::state::Relation;
use crate::{relation, singleton};

#[derive(Copy, Clone, PartialEq)]
pub enum SquareMark {
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

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SquareID(u8);

#[derive(Copy, Clone)]
pub struct Square {
    _id: SquareID,
    pub mark: SquareMark,
}

impl Square {
    pub fn new(id: SquareID) -> Self {
        Self {
            _id: id,
            mark: SquareMark::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Board {
    pub squares: [SquareID; 9],
}

impl Default for Board {
    fn default() -> Self {
        let mut squares = [SquareID(0); 9];
        for i in 0usize..9 {
            squares[i] = SquareID(i as u8);
        }
        Self { squares }
    }
}

impl<'a> Board {
    pub fn calculate_winner(&self) -> bool {
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
            // what mark is at each position?
            win.iter().map(|id| {
                Square::get(self.squares[*id]).unwrap().mark
            })
        }).any(|win| {
            // are any of the lines all the same mark?
            use SquareMark::Empty;
            win.fold_first(|a, b| {
                match a {
                    Empty => Empty,
                    _ => if a == b { a } else { Empty },
                }
            }).unwrap() != Empty
        })
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Status {
    Playing,
    Won,
}

impl Default for Status {
    fn default() -> Self { Self::Playing }
}

// Square::get(SquareID) -> (Arc<Square>, Ready)
relation!(Square, SquareID, Arc<Square>);

// GameBoard::get() -> Arc<Board>
singleton!(GameBoard, "game", Arc<Board>);
singleton!(GameStatus, "game", Status);
singleton!(NextPlayer, "next", SquareMark);
