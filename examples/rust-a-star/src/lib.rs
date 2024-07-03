wit_bindgen::generate!({
    path: "wit",
    additional_derives: [PartialEq, Eq],
});

#[derive(Clone, Copy)]
struct Cell {
    total_cost: u32, // f
    position: Position,
}

static mut OPEN_LIST: Vec<Cell> = vec![];
static mut STARTING_POS: Position = Position { row: 0, column: 0 };
static mut CLOSED_LIST: Vec<Cell> = vec![];

fn push_open_list(item: Cell) {
    unsafe {
        OPEN_LIST.push(item);
    }
}

fn push_closed_list(item: Cell) {
    unsafe {
        OPEN_LIST.push(item);
    }
}

struct Component;

impl Guest for Component {
    fn initialize(state: &State) {
        push_open_list(Cell {
            total_cost: 0,
            position: state.player_position(),
        });
        unsafe {
            STARTING_POS = state.player_position();
        }
    }

    fn step(state: &State) {
        let min = unsafe { OPEN_LIST.iter().min_by_key(|c| c.total_cost) };

        if let Some(current) = min {
            unsafe { OPEN_LIST.retain(|c| c.position != current.position) };

            for cell in state.adjacent(current.position) {
                if cell == state.target_position() {
                    break;
                }

                // TODO: compute g (cost to reach `cell` from the starting point) for `cell`
                // TODO: compute h (distance from `cell` to target) for `cell`
                // TODO: compute f (g + h) for `cell`

                if unsafe {
                    OPEN_LIST.iter().any(|c| c.position == cell)
                        || CLOSED_LIST.iter().any(|c| c.position == cell)
                } {
                    continue;
                }

                push_open_list(Cell {
                    total_cost: todo!(),
                    position: cell,
                });
            }

            push_closed_list(*current);
        }
    }
}

export!(Component);
