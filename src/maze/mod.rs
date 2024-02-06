use crate::maze::cell::{Cell, State};

mod cell;

pub struct MazeStruct {
    maze: Vec<Vec<Cell>>,
}

impl MazeStruct {
    pub fn new(width: i32, height: i32) -> Self {
        let mut maze: Vec<Vec<Cell>> = Vec::new();
        for y in 0..height {
            let mut temp_vect: Vec<Cell> = Vec::new();
            for x in 0..width {
                temp_vect.push(Cell::new(x, y));
            }
            maze.push(temp_vect);
        }

        // Define enter and exit in the maze for the future simplicity
        let _ = &maze[1][0].set_state(State::Empty); // Enter
        let _ = &maze[0][1].set_bool_visited(false);
        let _ = &maze[height as usize - 2][width as usize - 1].set_state(State::Empty);
        let _ = &maze[height as usize - 2][width as usize - 1].set_bool_visited(false);

        MazeStruct {maze}
    }

    fn get_maze_height(&self) -> usize {
        self.maze.len()
    }

    fn get_maze_width(&self) -> usize {
        self.maze[0].len()
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