use crate::maze::Maze;

mod user_interface;
mod maze;

fn main() {
    let mut maze = Maze::new();
    user_interface::run_application_logic(&mut maze);
}
