#[allow(non_snake_case)]
mod CollisionSystem;

#[allow(non_snake_case)]
mod MathUtilities;

#[allow(non_snake_case)]
mod Objects;

#[allow(non_snake_case)]
mod ControllFlow;

#[allow(non_snake_case)]
mod Program;
use Program::Program as MainProgram;


fn main() {
    let mut program = MainProgram::new();

    program.initialize();
    program.main_loop();
    program.finish();   
}
