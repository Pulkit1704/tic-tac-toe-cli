
mod terminal_inteface;
mod game;

use game::{get_game_grid, play_turn}; 
use terminal_inteface::{print_grid, show_player_prompt, read_input}; 

fn run_prompt(game_grid: &mut Vec<String>, player_1: &str, player_2: &str){
    // show the play grid, 
    // wait for user input, 
    // update the specified position with the player's character 
    // show the play grid, 
    //repeat 

    let mut active_player = player_1; 

    loop{
        print_grid(game_grid);
        show_player_prompt(); 

        let user_input = read_input(); 

        match user_input{
            Ok(arg_str) => {
                let args: Vec<&str> = arg_str.split_whitespace().collect(); 

                if args.len() > 1 {
                    println!("too many arguments provided... only the first one will be used"); 
                }

                // todo: dont make the thread panic if the player enters the wrong value for number.eat up their turn insted.
                let position = args[0].parse::<usize>().unwrap();  

                play_turn(position, game_grid, active_player); 
            
            }, 

            Err(message) => {
                println!("{}", message); 
            }
        }


        active_player = if active_player == player_1 {player_2} else {player_1}; 

    }
}

fn main() {
    let mut game_grid = get_game_grid(); 

    let player_1 = "X"; 
    let player_2 = "O"; 

    run_prompt(&mut game_grid, player_1, player_2); 
}
