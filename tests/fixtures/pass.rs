use nah::{boy_scout_fail, dead_code_candidate, kiss_violation, spaghetti_code, too_many_arguments};

#[too_many_arguments("group these parameters into a config struct")]
pub fn render(a: i32, b: i32, c: i32, d: i32) -> i32 {
    a + b + c + d
}

#[dead_code_candidate]
pub struct Demo;

#[boy_scout_fail]
pub enum State {
    Ready,
    Busy,
}

#[spaghetti_code]
pub trait Handler {
    fn handle(&self);
}

#[kiss_violation]
impl Handler for Demo {
    fn handle(&self) {}
}

fn main() {
    let demo = Demo;
    demo.handle();
    let _ = render(1, 2, 3, 4);
    let _ = State::Ready;
}

