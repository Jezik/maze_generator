use std::fmt::{Display, Formatter};

enum State {
    Wall,
    //Path,
    Empty,
}

pub struct Cell {
    x_coord: i32,
    y_coord: i32,
    state: State,
    is_visited: bool,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Cell {
        if x == 0 || y == 0 || x % 2 == 0 || y % 2 == 0 {
            return Cell {x_coord: x, y_coord: y, state: State::Wall, is_visited: false};
        }
        Cell {x_coord: x, y_coord: y, state: State::Empty, is_visited:false}
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.state {
            State::Wall => write!(f, "▓▓"),
            State::Empty => write!(f, "  "),
        }
    }
}