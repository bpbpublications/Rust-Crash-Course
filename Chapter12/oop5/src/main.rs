trait State {
    fn process(&self);
}

struct Start;
impl State for Start {
    fn process(&self) {
        println!("Started..")
    }
}

struct Running;
impl State for Running {
    fn process(&self) {
        println!("Running..")
    }
}

struct Stop;
impl State for Stop {
    fn process(&self) {
        println!("Stopped..")
    }
}

struct StateContext {
    state: Box<dyn State>
}

impl StateContext {
    fn new() -> StateContext {
        StateContext {
            state: Box::new(Start {}),
        }
    }
    fn set_state(&mut self, s: Box<dyn State>) {
        self.state = s;
    }
    
    fn process(&self) {
        self.state.process();
    }
}

fn main() {
    let mut ctxt = StateContext::new();
    ctxt.process();
    ctxt.set_state(Box::new(Running {}));
    ctxt.process();
    ctxt.process();
    ctxt.process();
    ctxt.set_state(Box::new(Stop {}));
    ctxt.process();
}
