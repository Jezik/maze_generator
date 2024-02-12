use std::{error, io, process};
use crate::maze::Maze;

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

fn match_user_input(menu: &Menu, maze: &mut Maze) {
    match menu {
        Menu::GenerateMaze => {
            let (width, height) = get_maze_dimensions();
            maze.generate_maze(width, height);
            maze.print_maze();
        },
        Menu::SaveMaze => {
            println!("Please input the name of a file");
            let mut file_name = String::new();
            io::stdin()
                .read_line(&mut file_name)
                .expect("Error proceeding input");
            let file_name= file_name.trim();
            let file_name_extension = format!("{}.maze", file_name);
            if let Err(e) = maze.write_to_file(&file_name_extension) {
                println!("Error has occurred. {}", e.to_string());
            }
        },
        Menu::LoadMaze => {
            println!("Please input the name of a file");
            let mut file_name = String::new();
            io::stdin()
                .read_line(&mut file_name)
                .expect("Error proceeding input");
            let file_name= file_name.trim();
            let file_name_extension = format!("{}.maze", file_name);
            maze.read_from_file(&file_name_extension);
            maze.print_maze();
        },
        Menu::SolveMaze => {
            maze.find_path();
            maze.print_maze();
        },
        Menu::Quit => {
            println!("Thanks for using Rust for this, bye! :)");
            process::exit(0);
        },
    }
}

fn get_usize_from_user() -> usize {
    loop {
        let mut some_str = String::new();
        io::stdin()
            .read_line(&mut some_str)
            .expect("Reading the input is impossible");
        if let Ok(some_usize) = some_str.trim().parse::<usize>() {
            return some_usize;
        } else {
            println!("Consider using a valid integer next time");
        }
    }
}

fn get_maze_dimensions() -> (usize, usize) {
    println!("Please provide to integers for the maze generation. Odd numbers show much better result though :)");
    let x = get_usize_from_user();
    let y = get_usize_from_user();
    (x ,y)
}

pub fn run_application_logic(maze: &mut Maze) {
    loop {
        print_menu();

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Can't read input");

        if let Ok(menu) = Menu::from_str(&user_input) {
            match_user_input(&menu, maze);
        } else {
            println!("Error has occurred. Try one more time.\nAcceptable command is an integer within the range [1..5]\n");
        }
    }
}