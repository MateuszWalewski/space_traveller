use space_traveller::CircleIter;

fn main() {
    let container = vec![1, 4, 5, 7];
    for i in container.circle_iter().take(9) {
        println!("item: {}", i);
    }
}
