use rusty_engine::prelude::*;

struct GameState {
  high_score:u32;
  current_score:u32;
  
}

fn main() {
    let mut game = Game::new();

    game.run(());
}
