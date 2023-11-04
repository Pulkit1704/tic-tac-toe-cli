
mod terminal_inteface;

use terminal_inteface::{print_grid, show_player_prompt}; 

fn run_prompt(test_vec: &Vec<String>){
    // show the play grid, 
    // wait for user input, 
    // update the specified position with the player's character 
    // show the play grid, 
    //repeat 

    loop{
        print_grid(test_vec); 

        show_player_prompt(); 
    }
}

fn main() {
    let mut test_vec = Vec::new(); 

    for i in 1..10{
        test_vec.push(i.to_string()); 
    }

    run_prompt(&test_vec); 
}
