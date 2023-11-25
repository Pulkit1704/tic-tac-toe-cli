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
    
    // game will be over when play_turn sets this variable to 1
    let mut game_over_status = 0; 

    while game_over_status == 0{
        print_grid(game_grid);
        show_player_prompt(active_player); 

        let user_input = read_input(); 

        match user_input{
            Ok(arg_str) => {
                let args: Vec<&str> = arg_str.split_whitespace().collect(); 

                if args.len() > 1 {
                    println!("too many arguments provided... only the first one will be used"); 
                }
                let position = args[0].parse::<usize>();  

                match position {
                    Ok(position)=> {

                        if position < 1 || position > 9{
                            println!("value not in grid bounds... enter value between 1 and 9"); 
                            continue; 
                        }else{
                            if game_grid[position -1] != "-"{
                                println!("position already taken... take another turn"); 
                                continue; 
                            }else{
                                game_over_status = play_turn(position, game_grid, active_player); 
                                active_player = if active_player == player_1 {player_2} else {player_1}; 
                            }
                        }
                        
                    }, 

                    Err(_e) => {
                        println!("argument provided not a number try again... "); 
                        continue; 
                    }
                }
            
            }, 

            Err(message) => {
                println!("{}", message); 
            }
        }
    }
}

fn main() {
    let mut game_grid = get_game_grid(); 

    let player_1 = "X"; 
    let player_2 = "O"; 

    run_prompt(&mut game_grid, player_1, player_2); 
}
