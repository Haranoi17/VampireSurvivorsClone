mod Tests;

use sfml::graphics::RenderWindow;

use crate::Objects::Interfaces::{Updatable, Drawable};

pub mod Interfaces;
use self::Interfaces::State;

#[derive(PartialEq)]
pub enum FlowState{
    Enter,
    Update,
    Exit,
}

pub struct StateMachine<StatesEnum: Default + State<StatesEnum> + Drawable> {
    current_state: StatesEnum,
    flow_state: FlowState,
}

impl<StatesEnum: Default + State<StatesEnum> + Drawable> StateMachine<StatesEnum> {
    pub fn new() -> Self {
        Self {
            current_state: StatesEnum::default(),
            flow_state: FlowState::Enter,
        }
    }
}

impl<StatesEnum: Default + State<StatesEnum> + Drawable> StateMachine<StatesEnum> {
    pub fn swich_state(&mut self, state: StatesEnum) {
        self.current_state = state;
    }
}

impl<StatesEnum: Default + State<StatesEnum> + Drawable> Updatable for StateMachine<StatesEnum> {
    fn update(&mut self, delta_time: f32) {
       self.flow_state = match self.flow_state{
            FlowState::Enter => {
                self.current_state.onEnter();
                FlowState::Update
            }
            FlowState::Update => {
                self.current_state.onUpdate(delta_time)
            },
            FlowState::Exit => {
                let new_state = self.current_state.onExit();
                
                match new_state {
                    Some(state) => {
                        self.current_state = state;    
                        FlowState::Enter
                    }
                    None => panic!(),
                }
            },
        }
    }
}

impl<StatesEnum: Default + State<StatesEnum> + Drawable> Drawable for StateMachine<StatesEnum>{
    fn draw(&mut self, window: &mut RenderWindow) {
        self.current_state.draw(window);
    }
}