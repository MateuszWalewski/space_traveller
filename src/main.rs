use space_traveller::tools; // make it private! Leave InputReaders in the public module only!
use space_traveller::GameManager;

fn main() {
    let gm = GameManager::new();
    let pred_input = "tom78\njohn99\n";
    let mut custom_input_reader = tools::CustomInputReader::new(pred_input);
    let gm = gm.add_players(&mut custom_input_reader);

  //  let pred_input_2 = "1\n1\n1\n1\n";

  //  let mut custom_input_reader = tools::CustomInputReader::new(pred_input_2);

    let mut std_input_reader = tools::StdInputReader;
    let gm = gm.start_game(&mut std_input_reader);

    let winner = gm.get_winner();

    println!("The winner is {} !!! :)", winner.name());
}
