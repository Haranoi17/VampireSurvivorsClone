use sfml::graphics::RenderWindow;

use crate::{ControllFlow::{Interfaces::State, FlowState}, Objects::Interfaces::{Drawable, Updatable}, InputSystem::{InputConsumer, Input}};

use self::InnerStates::{MainIntroInnerState, MainMenuInnerState, GamePlayInnerState};

mod InnerStates;

pub enum States {
    Intro(MainIntroInnerState),
    Menu(MainMenuInnerState),
    GamePlay(GamePlayInnerState),
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
                menu.get_flow_state()
            }

            States::GamePlay(game_play) => {
                game_play.update(delta_time);
                FlowState::Update
            },
        }
    }

    fn onExit(&mut self) -> Option<States> {
        match self{
            States::Intro(_) =>{
                Some(States::Menu(MainMenuInnerState::new()))
            },
            States::Menu(menu) => {
                let chosen_state = menu.get_chosen_state();
                menu.reset();

                chosen_state
            }
            States::GamePlay(game_play) => todo!(),
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
            States::Menu(menu) => menu.draw(window),
            States::GamePlay(game_play) => game_play.draw(window),
        }
    }
}

impl InputConsumer for States{
    fn handle_input(&mut self, input: &Input) {
        match self {
            States::Intro(intro) => {},
            States::Menu(menu) => menu.handle_input(input),
            States::GamePlay(game_play) => game_play.handle_input(input),
        }
    }
}
