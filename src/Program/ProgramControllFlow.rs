use sfml::graphics::RenderWindow;

use crate::{ControllFlow::{Interfaces::State, FlowState}, Objects::Interfaces::{Drawable, Updatable}};

use self::InnerStates::{MainIntroInnerState, MainMenuInnerState};

mod InnerStates;

pub enum States {
    Intro(MainIntroInnerState),
    Menu(MainMenuInnerState),
}

impl State<States> for States {
    fn onEnter(&mut self) {}

    fn onUpdate(&mut self, delta_time: f32) -> FlowState {
        match self{
            States::Intro(intro) => {
                intro.update(delta_time);
                
                if intro.progress >= 1.0{
                    return FlowState::Exit
                }

                FlowState::Update
            }
            States::Menu(menu) => {
                menu.update(delta_time);
                FlowState::Update
            }
        }
    }

    fn onExit(&mut self) -> Option<States> {
        match self{
            States::Intro(_) =>{
                Some(States::Menu(MainMenuInnerState::new()))
            },
            States::Menu(_) => {
                Some(States::Intro(MainIntroInnerState::new()))
            }
        }
    }
}

impl Default for States{
    fn default() -> Self {
        Self::Intro(MainIntroInnerState::new())
    }
}

impl Drawable for States{
    fn draw(&mut self, window: &mut RenderWindow) {
        match self {
            States::Intro(intro) => intro.draw(window),
            States::Menu(menu) => {menu.draw(window)},
        }
    }
}