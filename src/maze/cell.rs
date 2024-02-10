use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub enum State {
    Wall,
    //Path,
    Empty,
}

pub struct Cell {
    state: State,
}

impl Cell {
    pub fn new() -> Self {
        Cell {state: State::Wall}
    }

    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    pub fn get_state(&self) -> &State {
        &self.state
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