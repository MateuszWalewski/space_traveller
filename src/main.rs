use space_traveller::{CycleIter, GameController};


fn main() {
    let mut gc = GameController::new();
    gc.add_players();

    gc.add_players();

    let container = vec![1, 4, 5, 7];

    for i in container.cycle_iter().take(9) {
        println!("item: {}", i);
    }
}
