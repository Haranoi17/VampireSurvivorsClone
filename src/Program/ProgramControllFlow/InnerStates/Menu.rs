use sfml::{
    graphics::{Color, Font, RenderTarget, Text, Transformable},
    window::Key,
    SfBox,
};

use crate::{
    ControllFlow::{FlowState},
    InputSystem::{InputConsumer, Keys},
    MathUtilities::Vector,
    Objects::Interfaces::{Drawable, Updatable},
    Program::ProgramControllFlow::States,
};

use super::GamePlayInnerState;

pub struct MainMenuInnerState {
    font: SfBox<Font>,

    selections: Vec<String>,

    current_selection: i32,
    should_enter_currently_selected: bool,

    start_position: Vector,
    spacing: f32,

    force_close_program: bool,
}

impl MainMenuInnerState {
    pub fn new() -> Self {
        Self {
            font: Font::from_file("resources/Fonts/gomarice_no_continue.ttf").unwrap(),
            selections: vec![
                String::from("Play"),
                String::from("Credits"),
                String::from("Exit"),
            ],
            current_selection: 0,
            should_enter_currently_selected: false,

            start_position: Vector::new(100.0, 100.0),
            spacing: 50.0,
            force_close_program: false,
        }
    }

    pub fn get_chosen_state(&mut self) -> Option<States> {
        if self.current_selection == 0 {
            return Some(States::GamePlay(GamePlayInnerState::new()));
        }
        None
    }

    fn enter_selected_state(&mut self) {
        self.should_enter_currently_selected = true;
    }

    pub fn get_flow_state(&mut self) -> FlowState {
        if self.should_enter_currently_selected {
            return FlowState::Exit;
        }
        FlowState::Update
    }

    pub fn reset(&mut self) {
        self.current_selection = 0;
        self.should_enter_currently_selected = false;
    }

    fn handle_input(&mut self) {
        if Key::W.is_pressed() {
            self.move_up();
        }

        if Key::S.is_pressed() {
            self.move_down();
        }
    }

    fn move_up(&mut self) {
        if self.current_selection > 0 {
            self.current_selection -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.current_selection < (self.selections.len() - 1) as i32 {
            self.current_selection += 1;
        }
    }

    fn draw_menu_buttons(&mut self, window: &mut sfml::graphics::RenderWindow) {
        for i in 0..self.selections.len() {
            let mut text = Text::new(&self.selections[i], &self.font, 20u32);
            if i == self.current_selection as usize {
                text.set_fill_color(Color::RED);
            } else {
                text.set_fill_color(Color::WHITE);
            }

            let text_position = self.start_position + Vector::new(0.0, i as f32 * self.spacing);
            text.set_position(text_position);

            window.draw(&text);
        }
    }

    fn close_window_if_forced(&mut self, window: &mut sfml::graphics::RenderWindow) {
        if self.force_close_program {
            window.close();
        }
    }
}

impl Updatable for MainMenuInnerState {
    fn update(&mut self, delta_time: f32) {}
}

impl InputConsumer for MainMenuInnerState {
    fn handle_input(&mut self, input: &crate::InputSystem::Input) {
        if input.just_pressed(Keys::Down) {
            self.move_down();
        }
        if input.just_pressed(Keys::Up) {
            self.move_up();
        }

        if input.just_pressed(Keys::Enter) {
            self.enter_selected_state()
        }

        if input.just_pressed(Keys::Esc) {
            self.force_close_program = true;
        }
    }
}

impl Drawable for MainMenuInnerState {
    fn draw(&mut self, window: &mut sfml::graphics::RenderWindow) {
        self.close_window_if_forced(window);
        self.draw_menu_buttons(window);
    }
}
