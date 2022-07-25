mod game;
use game::{Game, graphics::textures::GeneralTextureRepository};



fn main() {
    let texture_repository = GeneralTextureRepository::new();
    let mut game = Game::new(&texture_repository);

    game.initialize();
    while game.is_on() {
        game.handle_events();
        game.update();
        game.render();
    }
}
