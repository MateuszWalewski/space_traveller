use space_traveller::GameController;

fn main() {
    let player1 = String::from("steve12");
    let player2 = String::from("john90");

    let mut gc = GameController::new();

    gc.add_player(player1);
    gc.add_player(player2);

    //TODO: Introduce states to prevents from starting the game before
    // the players are added
    let winner = gc.start_game().expect("Will be good");

    println!("The winner is MAINA {}", winner.name());
}
