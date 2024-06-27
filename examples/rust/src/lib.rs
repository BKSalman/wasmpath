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
    fn step(state: &State) {
        if !get_went_up() {
            state.move_up();
            set_went_up(true);
        } else {
            state.move_left();
            set_went_up(false);
        }
    }
}

export!(Component);
