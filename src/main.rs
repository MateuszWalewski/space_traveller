use space_traveller::tools;
use space_traveller::GameController;
use std::io;
fn main() {
    let player1 = String::from("steve12");
    let player2 = String::from("john90");

    let mut gc = GameController::new();

    gc.add_player(player1);
    gc.add_player(player2);

    //let mut std_input_reader = tools::StdInputReader;
    let pred_input = "1\n1\n1\n1\n";
    let mut custom_input_reader = tools::CustomInputReader::new(pred_input);
    //TODO: Introduce states to prevents from starting the game before
    // the players are added
    let winner = gc
        .start_game(&mut custom_input_reader)
        .expect("Will be good");

    println!("The winner is MAINA {}", winner.name());
}
