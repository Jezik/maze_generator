use crate::maze::cell::Cell;

mod cell;

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