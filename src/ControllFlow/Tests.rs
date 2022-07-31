use crate::Objects::Interfaces::Updatable;

use super::FlowState;
use super::State;
use super::StateMachine;

mod TestInnerStates;
use TestInnerStates::{TestState1InnerState,TestState2InnerState};

mod TestStateEnums;
use TestStateEnums::TestStatesEnum;

#[test]
#[should_panic]
fn general_state_switching_test(){
    let s1 = Box::new(TestStatesEnum::TestState1(TestState1InnerState::default()));
    let s2 = Box::new(TestStatesEnum::TestState2(TestState2InnerState::default()));

    let mut state_machine: StateMachine<TestStatesEnum> = StateMachine::new();

    while(true){
        state_machine.update(0.0);
    }
}