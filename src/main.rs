use space_traveller::tools; // make it private! Leave InputReaders in the public module only!
use space_traveller::GameManager;

fn main() {
    let gm = GameManager::new();
    let mut std_input_reader = tools::StdInputReader;
    let gm = gm.add_players(&mut std_input_reader);
    let mut std_input_reader = tools::StdInputReader;
    let gm = gm.start_game(&mut std_input_reader);

    let winner = gm.get_winner();

    println!("The winner is {} !!! :)", winner.name());
}
