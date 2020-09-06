use alloc::sync::Arc;

use serde::{Deserialize, Serialize};
use valerie::store::{Mutator, Relation};
use valerie::{remote, singleton};

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

impl Mutator<Arc<Square>> for SquareChange {
    fn mutate(&self, v: &Arc<Square>) -> Arc<Square> {
        let mut v: Square = Square::clone(v);
        match self {
            Self::Mark(mark) => v.mark = *mark,
        }
        Arc::new(v)
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
        let template = Arc::new(Square::default());
        LINES
            .iter()
            .map(|win| {
                // what mark is at each position?
                win.iter()
                    .map(|id| Square::get(self.squares[*id], &template).0.mark)
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

// Square::get(SquareID) -> (ArcSquare, Ready)
remote!(
    Square,
    SquareID,
    Arc<Square>,
    "https://jsonplaceholder.typicode.com/todos/1"
);

// GameBoard::get() -> Arc<Board>
singleton!(GameBoard, Arc<Board>);
singleton!(GameStatus, Status);
singleton!(NextPlayer, SquareMark);
