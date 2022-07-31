use sfml::{
    graphics::{RenderWindow, RenderTarget, Color},
    system::Clock,
    window::{ContextSettings, Event, Style, VideoMode},
};

mod ProgramControllFlow;
use ProgramControllFlow::States;
use crate::{ControllFlow::StateMachine, Objects::Interfaces::{Updatable, Drawable}};

pub struct Program {
    window: RenderWindow,
    timer: Clock,
    states: StateMachine<States>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            window: Self::create_window(),
            timer: Clock::default(),
            states: StateMachine::new(),
        }
    }

    pub fn main_loop(&mut self) {
        while self.window.is_open() {
            self.handle_events();
            self.update();
            self.draw();
        }
    }
    
    
    pub fn initialize(&mut self) {}
    
    
    fn update(&mut self){
        let delta_time = self.timer.restart().as_seconds();
        self.states.update(delta_time);
    }
    
    fn draw(&mut self){
        self.window.clear(Color::BLACK);
        self.states.draw(&mut self.window);
        self.window.display();
    }

    pub fn finish(&mut self) {}
    
    fn handle_events(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.window.close(),
                _ => {}
            }
        }
    }

    fn create_window() -> RenderWindow {
        let vide_mode = VideoMode {
            width: 1280,
            height: 720,
            ..VideoMode::desktop_mode()
        };

        RenderWindow::new(vide_mode, "", Style::DEFAULT, &ContextSettings::default())
    }
}
