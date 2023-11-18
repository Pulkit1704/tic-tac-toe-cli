use std::io;
use std::io::Write;
use std::io::stdin; 
use std::io::stdout; 

pub fn print_grid(grid_values: &Vec<String>){

    if grid_values.len() != 9{
        println!("no enough values to make the grid, {}", grid_values.len()); 
        return; 
    }

    let mut grid_output_string = String::new(); 

    for (i, value) in grid_values.iter().enumerate(){

        grid_output_string.push_str(&format!("|{:^3}", value)); 

        if i%3 == 2{
            grid_output_string.push_str(&format!("|\n-------------\n"));
        }

    }

    println!("{}", grid_output_string); 
}


pub fn show_player_prompt(){

    let player_symbol = "x";

    let mut stdout = stdout(); 
    print!("player {}'s turn, pick a spot: ", player_symbol);

    stdout.flush().expect("can't flush stdout: ");

}

pub fn read_input()-> io::Result<String>{

    let mut buffer = String::new(); 

    stdin().read_line(&mut buffer)?; 

    Ok(buffer) 
    
}