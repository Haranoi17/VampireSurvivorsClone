
mod game;

fn main() {
    let mut window = create_window();
    let texture_repository = GeneralTextureRepository::new();
    let mut drawables = vec![Entity::new()];
    setup_textures(&mut drawables, &texture_repository);

    while window.is_open() {
        handle_events(&mut window);
        clear_screen(&mut window);

        for drawable in &drawables {
            drawable.draw(&mut window);
        }

        window.display();
    }
}
