use crate::maze::Maze;

mod user_interface;
mod maze;

//TODO: 3. Save result to a file
//TODO: 4. Load result from a file

fn main() {
    let mut maze = Maze::new();
    user_interface::run_application_logic(&mut maze);
}
