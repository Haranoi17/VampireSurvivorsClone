use crate::{ControllFlow::{FlowState, State}, Objects::Interfaces::Drawable};

use super::TestInnerStates::{TestState1InnerState, TestState2InnerState};

pub enum TestStatesEnum {
    TestState1(TestState1InnerState),
    TestState2(TestState2InnerState),
}

impl State<TestStatesEnum> for TestStatesEnum {
    fn onEnter(&mut self) {
        println!("Entering")
    }

    fn onUpdate(&mut self, delta_time: f32) -> FlowState {
        println!("Updating");
        match self {
            TestStatesEnum::TestState1(inner_state) => {
                let update_count = 10;
                if inner_state.update_iteration < update_count {
                    inner_state.update_ten_times();
                    return FlowState::Update;
                }

                return FlowState::Exit;
            }
            TestStatesEnum::TestState2(inner_state) => {
                println!("entered second state");
                return FlowState::Exit;
            }
        }
    }

    fn onExit(&mut self) -> Option<TestStatesEnum> {
        println!("Exiting");
        match self {
            TestStatesEnum::TestState1(_) => {
                Some(TestStatesEnum::TestState2(TestState2InnerState::default()))
            }
            TestStatesEnum::TestState2(_) => {
                None
            }
        }
    }
}

impl Default for TestStatesEnum {
    fn default() -> Self {
        Self::TestState1(TestState1InnerState::default())
    }
}

impl Drawable for TestStatesEnum{
    fn draw(&mut self, window: &mut sfml::graphics::RenderWindow) {
        
    }
}