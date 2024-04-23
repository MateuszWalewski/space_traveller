use space_traveller::tools; // make it private! Leave InputReaders in the public module only!
use space_traveller::view; // make it private! Leave InputReaders in the public module only!
use space_traveller::GameManager;

fn main() {
    let std_reader = tools::StdInputReader;
    let console_view = view::ConsoleView;
    let gm = GameManager::new(std_reader, console_view);
    let gm = gm.add_players();
    let gm = gm.start_game();
    gm.finish_game();
}
