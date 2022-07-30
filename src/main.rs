#[allow(non_snake_case)]
mod Animations;

#[allow(non_snake_case)]

mod CollisionSystem;

#[allow(non_snake_case)]
mod MathUtilities;

#[allow(non_snake_case)]
mod Objects;
use CollisionSystem::{Collider, Collidable};
use Objects::{
    Interfaces::{Drawable, Initializable, Updatable},
    Player::Player, Enemy::Enemy,
};

use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    system::Clock,
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
    let mut player = Player::new();
    let mut enemy = Enemy::new();

    player.initialize();

    let mut window = create_window();

    let mut timer = Clock::start();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
        }

        let delta_time = timer.elapsed_time().as_seconds();
        timer.restart();

        player.update(delta_time);

        let result = Collider::collide(player.get_collider(), enemy.get_collider());
        println!("{}",result);

        window.clear(Color::BLACK);


        player.draw(&mut window);
        enemy.draw(&mut window);



        window.display();
    }
}
