Just play

```rust
use space_traveller::{view, GameManager, StdInputReader};

fn main() {
    let std_reader = StdInputReader;
    let console_view = view::ConsoleView;
    let gm = GameManager::new(std_reader, console_view);
    let gm = gm.add_players();
    let gm = gm.start_game();
    gm.finish_game();
}

```
