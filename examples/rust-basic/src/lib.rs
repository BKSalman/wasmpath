use component::wasmpath::game::State;
use exports::component::wasmpath::solution::{Direction, Guest, GuestSolver};

wit_bindgen::generate!({
    path: "wit"
});

static mut WENT_UP: bool = false;

fn set_went_up(yes: bool) {
    unsafe {
        WENT_UP = yes;
    }
}

fn get_went_up() -> bool {
    unsafe { WENT_UP }
}

struct Component;

impl Guest for Component {
    type Solver = Component;
}

impl GuestSolver for Component {
    fn new(_state: &State) -> Self {
        Self
    }
    fn step(&self, _state: &State) -> Option<Direction> {
        if !get_went_up() {
            set_went_up(true);
            Some(Direction::Up)
        } else {
            set_went_up(false);
            Some(Direction::Down)
        }
    }
}

export!(Component);
