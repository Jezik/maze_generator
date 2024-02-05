use std::{error, io, process};
use crate::maze;

fn print_menu() {
    println!("***Maze Generator Menu***\nChoose command(1 ... 5)\n\
    1. Generate mew maze\n\
    2. Save maze to a file\n\
    3. Load maze from a file\n\
    4. Solve maze\n\
    5. Quit\n");
}

enum Menu {
    GenerateMaze,
    SaveMaze,
    LoadMaze,
    SolveMaze,
    Quit,
}

impl Menu {
    fn from_str(s: &str) -> Result<Menu, Box<dyn error::Error>> {
        match s.trim() {
            "1" => Ok(Menu::GenerateMaze),
            "2" => Ok(Menu::SaveMaze),
            "3" => Ok(Menu::LoadMaze),
            "4" => Ok(Menu::SolveMaze),
            "5" => Ok(Menu::Quit),
            _ => Err("Error...".into()),
        }
    }
}

fn match_user_input(menu: &Menu) {
    match menu {
        Menu::GenerateMaze => {
            println!("We are going to generate a new maze\n");
            let mut maze= maze::create_initial_matrix();
            maze.print_maze();
        },
        Menu::SaveMaze => println!("The maze will be saved to a file\n"),
        Menu::LoadMaze => println!("The maze will be loaded from a file\n"),
        Menu::SolveMaze => println!("The solution to the current maze will be shown\n"),
        Menu::Quit => {
            println!("Thanks for using Rust for this, bye! :)");
            process::exit(0);
        },
    }
}

pub fn run_application_logic() {
    loop {
        print_menu();

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Can't read input");

        if let Ok(menu) = Menu::from_str(&user_input) {
            match_user_input(&menu);
        } else {
            println!("Error has occurred. Try one more time.\nAcceptable command is an integer within the range [1..5]\n");
        }
    }
}