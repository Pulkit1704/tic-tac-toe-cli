

pub fn get_game_grid()-> Vec<String> {

    let mut game_grid = Vec::new(); 

    for _ in 0..9{
        game_grid.push("-".to_string()); 
    }

    return game_grid; 
}

// * give out an exit code to tell the main function that game is over. 
pub fn play_turn(position: usize, game_grid: &mut Vec<String>, active_player: &str)-> i32{

    if game_grid[position-1] != "-"{
        println!("position already filled... pick another spot"); 
        return 0; 
    }

    game_grid[position-1] = active_player.to_string(); 

    if player_won(game_grid, active_player) {
        println!("game over, {active_player} won!!!"); 
        return 1;
    }else{
        if is_game_over(game_grid){
            println!("game over, nobody won :(");
            return 1;  
        }
    }

    return 0; 

}

fn player_won(game_grid: &Vec<String>, active_player: &str) -> bool{

    return column_check(game_grid, active_player) || row_check(game_grid, active_player)|| diagonal_check(game_grid, active_player); 

}

fn column_check(game_grid:&Vec<String>, character: &str) -> bool{

    if game_grid[0] == character && game_grid[3] == character && game_grid[6] == character{
        return true; 
    }
    if game_grid[1] == character && game_grid[4] == character && game_grid[7] == character{
        return true; 
    }
    if game_grid[2] == character && game_grid[5] == character && game_grid[8] == character{
        return true; 
    }

    return false; 
}

fn row_check(game_grid: &Vec<String>, character: &str) -> bool{
    if game_grid[0] == character && game_grid[1] == character && game_grid[2] == character{
        return true; 
    }
    if game_grid[3] == character && game_grid[4] == character && game_grid[5] == character{
        return true; 
    }
    if game_grid[6] == character && game_grid[7] == character && game_grid[8] == character{
        return true; 
    }
    return false; 
}

fn diagonal_check(game_grid: &Vec<String>, character: &str)-> bool{

    if game_grid[0] == character && game_grid[4] == character && game_grid[8] == character{
        return true; 
    }

    if game_grid[2] == character && game_grid[4] == character && game_grid[6] == character{
        return true; 
    }


    return false; 
}

fn is_game_over(game_grid: &Vec<String>)-> bool{

    for element in game_grid{
        if element == "-"{
            return false; 
        }
    }; 
    return true; 
}