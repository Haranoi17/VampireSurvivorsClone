use super::FlowState;

pub trait State<StatesEnum> {
    fn onEnter(&mut self);
    fn onUpdate(&mut self, delta_time: f32)->FlowState;
    fn onExit(&mut self) -> Option<StatesEnum>;
}
