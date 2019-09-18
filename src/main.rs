use std::io;

mod game;

fn main() {
    println!(
        "Welcome to TicTacToe!\n\n\
        First input (1: Player, -1: Computer) indicates who moves first\n\
        Later inputs (0 - 8) indicate positions\n\
        0 1 2\n\
        3 4 5\n\
        6 7 8\n\
        Output (1: Player, -1: Computer, 0: Tie)\n");
    let mut game = game::Game::new();
    loop {
        println!("Enter who go first:");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to read line");
                continue;
            }
        }
        let parsed_input = input.trim().parse();
        let input: isize;
        match parsed_input {
            Ok(num) => input = num,
            Err(_) => {
                println!("Enter a valid integer");
                continue;
            }
        }
        match input {
            1 | -1 => {
                game.play(input);
                break;
            }
            _ => println!("The input is out of range"),
        }
    }
}