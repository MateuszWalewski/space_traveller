# Space Traveller

A simple command-line game engine based on the MVC architecture, centered around cosmic space.
The "reader" and "viewer" components are fully customizable as public trait objects that are injected to the main part.
It may serve as a starting point for crafting epic, old-school RPGs, enriching the knowledge of the universe.


## Just play

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
