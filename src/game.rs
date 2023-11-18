

pub fn get_game_grid()-> Vec<String> {

    let mut game_grid = Vec::new(); 

    for _ in 0..9{
        game_grid.push("-".to_string()); 
    }

    return game_grid; 
}


pub fn play_turn(position: usize, game_grid: &mut Vec<String>, active_player_character: &str){

    // todo: check if the position is already filled with the character 
    // todo: check if the position provied is within the bounds 

    game_grid[position] = active_player_character.to_string(); 

}