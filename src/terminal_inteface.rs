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


pub fn show_player_prompt(player_symbol: &str){ 

    let mut stdout = stdout(); 
    print!("player {}'s turn, pick a spot: ", player_symbol);

    stdout.flush().expect("can't flush stdout: ");

}

pub fn read_input()-> io::Result<String>{

    let mut buffer = String::new(); 

    stdin().read_line(&mut buffer)?; 

    Ok(buffer) 
    
}

pub fn parse_arguments()-> Result<usize, &'static str> {
    
    let user_input = read_input(); 

    match user_input{
        Ok(arg_str)=>{
            let args: Vec<&str> = arg_str.split_whitespace().collect(); 

            if args.len() != 1{
                return Err("please provide only one input number between 1 and 9"); 
            }else{
                let position = args[0].parse::<usize>(); 

                match position{
                    Ok(position) => {
                        if position < 1|| position > 9{
                            return Err("value not in grid bounds... enter number between 1 and 9"); 
                        }else{
                            return Ok(position); 
                        }
                    }, 

                    Err(_integer_parse_error)=>{
                        return Err("The input provided is not a number...");
                    }
                }
            }
        }

        Err(_input_read_error)=>{
            return Err("could not read input... try again"); 
        }
    }
}