use space_traveller::CycleIter;

fn main() {
    let container = vec![1, 4, 5, 7];

    for i in container.cycle_iter().take(9) {
        println!("item: {}", i);
    }
}
