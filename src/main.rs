mod terminal_inteface;
mod game;

use game::{get_game_grid, play_turn}; 
use terminal_inteface::{print_grid, show_player_prompt, parse_arguments}; 

fn run_prompt(game_grid: &mut Vec<String>, player_1: &str, player_2: &str){

    let mut active_player = player_1; 

    // game will be over when play_turn function sets this variable to 1
    let mut game_over_status = 0; 

    while game_over_status == 0{
        print_grid(game_grid);
        show_player_prompt(active_player); 
        
        // * use the parse_arguments function here 
        let arg_parse_result = parse_arguments(); 

        match arg_parse_result{
            Ok(position) =>{

                if game_grid[position -1] == "X" || game_grid[position-1] == "O"{
                    println!("position alread taken, pick another one..."); 
                    continue; 
                }

                game_over_status = play_turn(position, game_grid, active_player); 
                active_player = if active_player == player_1 {player_2} else {player_1}; 
                continue; 
            }, 

            Err(e) =>{
                println!("GAME ERROR: {e}"); 
                continue; 
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