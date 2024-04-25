use space_traveller::{constants, GameManager};
mod common;

#[test]
fn addition_of_players_within_specified_limit_works_1() {
    let pred_input = "tom78\njohn99\n";
    let custom_reader = common::CustomInputReader::new(pred_input);
    let test_view = common::TestView;
    let gm = GameManager::new(custom_reader, test_view);
    let gm = gm.add_players();
    assert_eq!(gm.number_of_players(), 2);
}

#[test]
fn addition_of_players_within_specified_limit_works_2() {
    let pred_input = "tom78\njohn99\n";
    let custom_reader = common::CustomInputReader::new(pred_input);
    let test_view = common::TestView;
    let gm = GameManager::new(custom_reader, test_view);
    let gm = gm.add_players();
    let players = gm.get_players();
    assert_eq!(players[0].name(), "tom78");
    assert_eq!(players[1].name(), "john99");
}

#[test]
fn full_game_flow_works() {
    let pred_input = "tom78\njohn99\n1\n1\n1\n5\n";
    let custom_reader = common::CustomInputReader::new(pred_input);
    let test_view = common::TestView;
    let gm = GameManager::new(custom_reader, test_view);
    let gm = gm.add_players();
    let gm = gm.start_game();
    let winner = gm.finish_game();
    assert_eq!(winner.name(), "john99");
    assert_eq!(
        winner.score(),
        constants::TELEPORT_PREMIUM + constants::BOOST_PREMIUM
    );
}
