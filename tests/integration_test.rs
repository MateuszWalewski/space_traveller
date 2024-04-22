use space_traveller::tools::CustomInputReader;
use space_traveller::GameManager;

    #[test]
    fn addition_of_players_within_specified_limit_works_1() {
        let pred_input = "tom78\njohn99\n";
        let mut custom_input_reader = CustomInputReader::new(pred_input);
        let gm = GameManager::new();
        let gm = gm.add_players(&mut custom_input_reader);
        assert_eq!(gm.number_of_players(), 2);
    }

    #[test]
    fn addition_of_players_within_specified_limit_works_2() {
        let pred_input = "tom78\njohn99\n";
        let mut custom_input_reader = CustomInputReader::new(pred_input);
        let gm = GameManager::new();
        let gm = gm.add_players(&mut custom_input_reader);
        let players = gm.get_players();
        assert_eq!(players[0].name(), "tom78");
        assert_eq!(players[1].name(), "john99");
    }

    #[test]
    fn full_game_flow_works() {
        let pred_input = "tom78\njohn99\n";
        let mut custom_input_reader = CustomInputReader::new(pred_input);
        let gm = GameManager::new();
        let gm = gm.add_players(&mut custom_input_reader);
        let pred_input = "1\n1\n1\n1\n";
        let mut custom_input_reader = CustomInputReader::new(pred_input);
        let gm = gm.start_game(&mut custom_input_reader);
        let winner = gm.get_winner();
        assert_eq!(winner.name(), "tom78");
    }
