use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};

use crate::{
    MathUtilities::{Position, Vector},
    Objects::Interfaces::{Drawable, Updatable},
};


pub struct MainIntroInnerState {
    FAKE_LOAD_TIME: f32,
    elapsed_time: f32,
    pub progress: f32,

    BAR_SIZE_IN_PERCENT_OF_WINDOW_SIZE: Vector,
    BAR_POSITION_IN_PERCENT_OF_WINDOW_SIZE: Position,
}


impl MainIntroInnerState {
    pub fn new() -> Self {
        Self {
            FAKE_LOAD_TIME: 1.0,
            elapsed_time: 0.0,
            progress: 0.0,
            BAR_SIZE_IN_PERCENT_OF_WINDOW_SIZE: Vector::new(0.6, 0.2),
            BAR_POSITION_IN_PERCENT_OF_WINDOW_SIZE: Vector::new(0.5, 0.5),
        }
    }

    fn create_bar(&self, window_size: Vector) -> RectangleShape {
        let mut bar = RectangleShape::new();
        let bar_size = self.BAR_SIZE_IN_PERCENT_OF_WINDOW_SIZE * window_size;
        let bar_position = self.BAR_POSITION_IN_PERCENT_OF_WINDOW_SIZE * window_size;
        bar.set_size(bar_size);
        bar.set_position(bar_position - bar_size * 0.5);

        bar
    }

    fn create_progress_bar_with_current_state(&self, window_size: Vector) -> RectangleShape {
        let mut progress_bar = self.create_bar(window_size);
        progress_bar.set_fill_color(Color::RED);
        progress_bar.set_scale(Vector::new(self.progress, 1.0));

        progress_bar
    }

    fn create_progress_bar_outline(&self, window_size: Vector) -> RectangleShape {
        let mut progres_bar_outline = self.create_bar(window_size);
        progres_bar_outline.set_fill_color(Color::TRANSPARENT);
        progres_bar_outline.set_outline_color(Color::WHITE);
        progres_bar_outline.set_outline_thickness(30.0);
        progres_bar_outline
    }
}

impl Updatable for MainIntroInnerState {
    fn update(&mut self, delta_time: f32) {
        self.elapsed_time += delta_time;
        self.progress = self.elapsed_time / self.FAKE_LOAD_TIME;
    }
}

impl Drawable for MainIntroInnerState {
    fn draw(&mut self, window: &mut RenderWindow) {
        let window_size = Vector::from_Vector2u(window.size());
        let progress_bar_outline = self.create_progress_bar_outline(window_size);
        let progress_bar = self.create_progress_bar_with_current_state(window_size);

        window.draw(&progress_bar_outline);
        window.draw(&progress_bar);
    }
}