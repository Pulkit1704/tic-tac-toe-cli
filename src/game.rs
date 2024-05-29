pub fn get_game_grid()-> Vec<String> {

    let mut game_grid = Vec::new(); 

    for i in 0..9{
        game_grid.push((i+1).to_string()); 
    }

    return game_grid; 
}

// * give out a boolean value to the main function about the status of the game. 
pub fn play_turn(position: usize, game_grid: &mut Vec<String>, active_player: &str)-> bool{

    game_grid[position-1] = active_player.to_string(); 

    if player_won(game_grid, active_player) {
        println!("game over, {active_player} won!!!"); 
        return true;
    }else{
        if is_game_over(game_grid){
            println!("game over, nobody won :(");
            return true;  
        }
    }

    return false; 

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
        if let Ok(_value) = element.trim().parse::<i32>(){
            return false; 
        }else{
            continue; 
        }
    }; 
    return true; 
}