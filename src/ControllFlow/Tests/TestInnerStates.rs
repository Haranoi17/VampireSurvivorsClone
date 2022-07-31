#[derive(Default)]
pub struct TestState1InnerState{
    pub update_iteration: i32
}

impl TestState1InnerState{
    pub fn update_ten_times(&mut self){
        println!("iteration: {}", self.update_iteration);
        self.update_iteration += 1;
    }
}


#[derive(Default)]
pub struct TestState2InnerState{
    is_entering: bool,
    should_exit: bool,
}