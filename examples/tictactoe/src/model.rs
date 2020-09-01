use std::collections::HashMap;
use std::sync::Arc;

use crate::store::{Mutator, Relation};
use crate::{remote, singleton};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Copy, Clone, PartialEq, Debug, Deserialize)]
pub enum SquareMark {
    Empty,
    X,
    O,
}

impl Default for SquareMark {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug)]
pub enum NextPlayerChange {
    Start,
    Next,
}

impl Mutator<SquareMark> for NextPlayerChange {
    fn mutate(&self, v: &SquareMark) -> SquareMark {
        use SquareMark::*;
        match self {
            Self::Start => X,
            Self::Next => match v {
                X => O,
                O => X,
                Empty => Empty,
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct SquareID(u8);

#[derive(Copy, Clone, Deserialize)]
pub struct Square {
    id: SquareID,
    pub mark: SquareMark,
}

impl Square {
    pub fn new(id: SquareID) -> Self {
        Self {
            id,
            mark: SquareMark::default(),
        }
    }
}

impl Default for Square {
    fn default() -> Self {
        Square {
            id: SquareID(0),
            mark: SquareMark::default(),
        }
    }
}

#[derive(Debug)]
pub enum SquareChange {
    Mark(SquareMark),
}

impl Mutator<ArcSquare> for SquareChange {
    fn mutate(&self, v: &ArcSquare) -> ArcSquare {
        let mut v: Square = Square::clone(&(*v).0);
        match self {
            Self::Mark(mark) => v.mark = *mark,
        }
        ArcSquare(Arc::new(v))
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
            let square = Square::new(SquareID(i as u8));
            Square::new(square.id);
            squares[i] = square.id;
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
        let template = ArcSquare::default();
        LINES
            .iter()
            .map(|win| {
                // what mark is at each position?
                win.iter()
                    .map(|id| Square::get(self.squares[*id], &template).0.0.mark)
            })
            .any(|win| {
                // are any of the lines all the same mark?
                use SquareMark::Empty;
                win.fold_first(|a, b| match a {
                    Empty => Empty,
                    _ => {
                        if a == b {
                            a
                        } else {
                            Empty
                        }
                    }
                })
                .unwrap()
                    != Empty
            })
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Status {
    Playing,
    Won,
}

impl Default for Status {
    fn default() -> Self {
        Self::Playing
    }
}

#[derive(Debug)]
pub enum StatusChange {
    Won,
}

impl Mutator<Status> for StatusChange {
    fn mutate(&self, _v: &Status) -> Status {
        match self {
            Self::Won => Status::Won,
        }
    }
}

#[derive(Clone)]
pub struct ArcSquare(pub Arc<Square>);

// Square::get(SquareID) -> (SquareType, Ready)
remote!(Square, SquareID, ArcSquare);

// GameBoard::get() -> Arc<Board>
singleton!(GameBoard, Arc<Board>);
singleton!(GameStatus, Status);
singleton!(NextPlayer, SquareMark);

impl Default for ArcSquare {
    fn default() -> Self {
        Self(Arc::new(Square::default()))
    }
}

impl<'de> Deserialize<'de> for ArcSquare {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let data = Square::deserialize(deserializer)?;
        Ok(Self(Arc::new(data)))
    }
}
