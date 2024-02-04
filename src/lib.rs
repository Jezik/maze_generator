mod cell {
    use std::fmt::{Display, Formatter};
    use crate::cell;

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
}

pub mod maze {
    use crate::cell::Cell;

    pub struct MazeStruct {
        maze: Vec<Vec<Cell>>,
    }

    impl MazeStruct {
        fn new() -> MazeStruct {
            MazeStruct {maze: Vec::new()}
        }

        pub fn print_maze(&self) {
            for y_vect in &self.maze {
                for cell in y_vect {
                    print!("{}", cell);
                }
                println!();
            }
            println!();
        }
    }

    pub fn create_initial_matrix() -> MazeStruct {
        let mut maze_struct = MazeStruct::new();
        for y in 0..11 {
            let mut temp_vect: Vec<Cell> = Vec::new();
            for x in 0..11 {
                temp_vect.push(Cell::new(x, y));
            }
            maze_struct.maze.push(temp_vect);
        }
        maze_struct
    }
}