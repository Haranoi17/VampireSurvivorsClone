use sfml::{graphics::{Font, Text, Color, Transformable, RenderTarget}, SfBox, window::Key};

use crate::{Objects::Interfaces::{Updatable, Drawable}, ControllFlow::StateMachine, MathUtilities::Vector, InputSystem::{InputConsumer, Keys}};

pub struct MainMenuInnerState {
    font: SfBox<Font>,

    selections: Vec<String>,
    current_selection: i32,

    start_position: Vector,
    spacing: f32,
}

impl MainMenuInnerState {
    pub fn new() -> Self {
        Self {
            font: Font::from_file("resources/Fonts/gomarice_no_continue.ttf").unwrap(),
            selections: vec![String::from("Play"), String::from("Credits"), String::from("Exit")],
            current_selection: 0,
            start_position: Vector::new(100.0,100.0),
            spacing: 50.0,
        }
    }

    fn handle_input(&mut self){
        if Key::W.is_pressed(){
            self.move_up();
        }

        if Key::S.is_pressed(){
            self.move_down();
        }
    }

    fn move_up(&mut self){
        if self.current_selection > 0{
            self.current_selection -= 1;
        }
    }

    fn move_down(&mut self){
        if self.current_selection < (self.selections.len()-1) as i32{
            self.current_selection += 1;
        }
    }
}


impl Updatable for MainMenuInnerState{
    fn update(&mut self, delta_time: f32) {
    }
}

impl InputConsumer for MainMenuInnerState{
    fn handle_input(&mut self, input: &crate::InputSystem::Input) {
        if input.just_pressed(Keys::Down){
            self.move_down();
        }
        if input.just_pressed(Keys::Up){
            self.move_up();
        }
    }
}

impl Drawable for MainMenuInnerState{
    fn draw(&mut self, window: &mut sfml::graphics::RenderWindow) {
        
        for i in 0..self.selections.len(){
            let mut text = Text::new(&self.selections[i], &self.font, 20u32);
            if i == self.current_selection as usize{
                text.set_fill_color(Color::RED);
            }
            else{
                text.set_fill_color(Color::WHITE);
            }
            
            let text_position = self.start_position + Vector::new(0.0, i as f32 * self.spacing);
            text.set_position(text_position);


            window.draw(&text);
        }
    }
}
