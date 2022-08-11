use std::{collections::HashMap, hash::Hash, array};

use sfml::window::Key;

use crate::Objects::Interfaces::{Updatable, Initializable};

#[derive(Default)]
struct PreviousCurrentValue<T>{
    pub previous_value: T,
    pub current_value: T,
}

pub trait InputConsumer{
    fn handle_input(&mut self, input: &Input);
}

pub enum Keys{
    Left,
    Right,
    Up,
    Down,
    Enter,
    Esc,

    Size,
}

pub struct Input{
    key_states: Vec<PreviousCurrentValue<bool>>,

    key_value_sources: Vec<Box<dyn Fn()->bool>>,
}

impl Input{
    pub fn new()->Self{
        Self{
            key_states: vec![],
            key_value_sources: vec![],
        }
    }

    pub fn is_pressed(&self, key: Keys)->bool{
        let index = key as usize;

        self.key_states.get(index).unwrap().current_value
    }

    pub fn just_pressed(&self, key: Keys)->bool{
        let index = key as usize;
        let has_been_just_pressed = self.key_states.get(index).unwrap().current_value && !self.key_states.get(index).unwrap().previous_value;

        has_been_just_pressed
    }

    fn update_current_keys_states(&mut self){
        let keys_count = Keys::Size as usize;
        for i in 0..keys_count{
            let get_current_key_state_function = self.key_value_sources.get(i).unwrap();
            let value =  get_current_key_state_function();
            self.key_states.get_mut(i).unwrap().current_value = value;
        }
    }

    fn update_previous_keyStates(&mut self){
        let keys_count = Keys::Size as usize;
        for i in 0..keys_count{
            self.key_states.get_mut(i).unwrap().previous_value = self.key_states.get(i).unwrap().current_value;
        }
    }

    fn get_key(key: &Key){
        return
    }
}

impl Initializable for Input{
    fn initialize(&mut self) {
        let keys_count = Keys::Size as usize;
        for i in 0..keys_count{
            self.key_states.push(PreviousCurrentValue::default());
            self.key_value_sources.push(Box::new(|| Key::ESCAPE.is_pressed())); //ESCAPE just to initialize vector with some random key
        }

        let w_supplier = Box::new(|| Key::W.is_pressed());
        let s_supplier = Box::new(|| Key::S.is_pressed());
        let a_supplier = Box::new(|| Key::A.is_pressed());
        let d_supplier = Box::new(|| Key::D.is_pressed());
        let enter_supplier = Box::new(|| Key::ENTER.is_pressed());
        let esc_supplier = Box::new(|| Key::ESCAPE.is_pressed());



        self.key_value_sources.insert(Keys::Left as usize, a_supplier);
        self.key_value_sources.insert(Keys::Right as usize, d_supplier);
        self.key_value_sources.insert(Keys::Up as usize, w_supplier);
        self.key_value_sources.insert(Keys::Down as usize, s_supplier);
        self.key_value_sources.insert(Keys::Enter as usize, enter_supplier);
        self.key_value_sources.insert(Keys::Esc as usize, esc_supplier);

    }
}

impl Updatable for Input{
    fn update(&mut self, delta_time: f32) {
        self.update_previous_keyStates();
        self.update_current_keys_states();
    }
}