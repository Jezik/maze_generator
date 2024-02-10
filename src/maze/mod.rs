use crate::maze::cell::{Cell, State};

use rand::Rng;

mod cell;

pub struct Maze {
    height: usize,
    width: usize,
    grid: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn new() -> Self {
        Maze { height: 0, width: 0, grid: Vec::new()}
    }

    fn generate_initial_grid(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        for _y in 0..height {
            let mut temp_vect: Vec<Cell> = Vec::new();
            for _x in 0..width {
                temp_vect.push(Cell::new());
            }
            self.grid.push(temp_vect);
        }

        // Define enter and exit in the maze for the future simplicity
        let _ = &self.grid[1][0].set_state(State::Empty); // Enter
        let _ = &self.grid[height - 2][width - 1].set_state(State::Empty); // Exit
    }

    pub fn generate_maze(&mut self, width: usize, height: usize) {
        self.grid.clear();
        self.generate_initial_grid(width, height);

        let mut stack: Vec<(usize, usize)> = Vec::new();
        let current_cell: (usize, usize) = (1, 1);
        self.grid[current_cell.0][current_cell.1].set_state(State::Empty);
        stack.push(current_cell);

        while let Some(cell) = stack.pop() {
            let neighbours = self.get_unvisited_neighbours(cell);

            if !neighbours.is_empty() {
                stack.push(cell);

                // Choose a random neighbor
                let neighbour_index = rand::thread_rng().gen_range(0 .. neighbours.len());
                let chosen_neighbour = neighbours[neighbour_index];

                // Remove the wall between current cell and chosen neighbor
                let (mid_row, mid_col) = ((cell.0 + chosen_neighbour.0) / 2, (cell.1 + chosen_neighbour.1) / 2);
                self.grid[mid_row][mid_col].set_state(State::Empty);
                self.grid[chosen_neighbour.0][chosen_neighbour.1].set_state(State::Empty);

                stack.push(chosen_neighbour);
            }
        }

    }

    fn get_unvisited_neighbours(&self, cell: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();

        if cell.0 >= 2 && *self.grid[cell.0 - 2][cell.1].get_state() == State::Wall {
            neighbours.push((cell.0 - 2, cell.1));
        }
        if cell.0 + 2 < self.height && *self.grid[cell.0 + 2][cell.1].get_state() == State::Wall {
            neighbours.push((cell.0 + 2, cell.1));
        }
        if cell.1 >= 2 && *self.grid[cell.0][cell.1 - 2].get_state() == State::Wall {
            neighbours.push((cell.0, cell.1 - 2));
        }
        if cell.1 + 2 < self.width && *self.grid[cell.0][cell.1 + 2].get_state() == State::Wall {
            neighbours.push((cell.0, cell.1 + 2));
        }

        neighbours
    }

    pub fn print_maze(&self) {
        for height_vect in &self.grid {
            for cell in height_vect {
                print!("{}", cell);
            }
            println!();
        }
        println!();
    }
}