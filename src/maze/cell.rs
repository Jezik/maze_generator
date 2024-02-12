use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub enum State {
    Wall,
    Path,
    Empty,
}

pub struct Cell {
    state: State,
    is_visited: bool,
}

impl Cell {
    pub fn new(state: State, is_visited: bool) -> Self {
        Cell {state, is_visited}
    }

    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn is_visited(&self) -> bool {
        self.is_visited
    }

    pub fn set_visited(&mut self) {
        self.is_visited = true;
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.state {
            State::Wall => write!(f, "▓▓"),
            State::Path => write!(f, "##"),
            State::Empty => write!(f, "  "),
        }
    }
}