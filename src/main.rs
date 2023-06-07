use std::error::Error;
use invaders::game::Game;

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new();
    game.play()
}
