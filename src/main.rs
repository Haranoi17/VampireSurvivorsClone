use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture, Transformable},
    system::{Vector2f, Clock},
    window::{ContextSettings, Event, Style, VideoMode},
};

fn create_window() -> RenderWindow {
    let vide_mode = VideoMode {
        width: 1280,
        height: 720,
        ..VideoMode::desktop_mode()
    };

    RenderWindow::new(vide_mode, "", Style::DEFAULT, &ContextSettings::default())
}

fn main() {
    let mut window = create_window();

    let texture = Texture::from_file("resources/character.png").unwrap();
    let mut player = Sprite::new();
    player.set_texture(&texture, true);
    player.set_scale((0.1, 0.1));

    let mut player_pos = Vector2f::new(0.0, 0.0);
    let mut player_dir = Vector2f::new(0.0, 0.0);

    let mut timer = Clock::start();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
        }

        player_dir = Vector2f::new(0.0, 0.0);
        if sfml::window::Key::A.is_pressed() {
            player_dir += Vector2f::new(-1.0, 0.0);
        }
        if sfml::window::Key::D.is_pressed() {
            player_dir += Vector2f::new(1.0, 0.0);
        }
        if sfml::window::Key::W.is_pressed() {
            player_dir += Vector2f::new(0.0, -1.0);
        }
        if sfml::window::Key::S.is_pressed() {
            player_dir += Vector2f::new(0.0, 1.0);
        }

        let dt = timer.elapsed_time().as_seconds();
        timer.restart();
        player_pos += player_dir*dt*100.0;
        player.set_position(player_pos);
        window.clear(Color::BLACK);
        window.draw(&player);
        window.display();
    }
}
