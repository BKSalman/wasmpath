use std::sync::{OnceLock, RwLock};

use wasi::logging::logging::{self, Level};

wit_bindgen::generate!({
    path: "wit",
    additional_derives: [PartialEq, Eq],
});

#[derive(Debug, Clone, Copy)]
struct Cell {
    cost_from_starting_point: u32, // g
    total_cost: u32,               // f
    position: Position,
}

fn open_list() -> &'static RwLock<Vec<Cell>> {
    static OPEN_LIST: OnceLock<RwLock<Vec<Cell>>> = OnceLock::new();
    OPEN_LIST.get_or_init(|| RwLock::new(vec![]))
}

fn closed_list() -> &'static RwLock<Vec<Cell>> {
    static CLOSED_LIST: OnceLock<RwLock<Vec<Cell>>> = OnceLock::new();
    CLOSED_LIST.get_or_init(|| RwLock::new(vec![]))
}

struct Component;

impl Guest for Component {
    fn initialize(state: &State) {
        open_list().write().unwrap().push(Cell {
            cost_from_starting_point: 0,
            total_cost: 0,
            position: state.player_position(),
        });
    }

    fn step(state: &State) {
        loop {
            let open_list_lock = open_list().read().unwrap();
            let min = open_list_lock.iter().copied().min_by_key(|c| c.total_cost);
            drop(open_list_lock);

            if let Some(current) = min {
                open_list()
                    .write()
                    .unwrap()
                    .retain(|c| c.position != current.position);

                for cell in state.adjacent(current.position) {
                    if cell == state.target_position() {
                        open_list().write().unwrap().push(Cell {
                            cost_from_starting_point: 0,
                            total_cost: 0,
                            position: cell,
                        });
                        break;
                    }

                    if open_list()
                        .read()
                        .unwrap()
                        .iter()
                        .any(|c| c.position == cell)
                        || closed_list()
                            .read()
                            .unwrap()
                            .iter()
                            .any(|c| c.position == cell)
                    {
                        continue;
                    }

                    // compute g (cost to reach `cell` from the starting point) for `cell`
                    let g = current.cost_from_starting_point + 1;
                    // compute h (distance from `cell` to target) for `cell`
                    let target_pos = state.target_position();
                    let h =
                        target_pos.row.abs_diff(cell.row) + target_pos.column.abs_diff(cell.column);
                    // compute f (g + h) for `cell`
                    let f = g + h;

                    let new_cell = Cell {
                        cost_from_starting_point: g,
                        total_cost: f,
                        position: cell,
                    };

                    open_list().write().unwrap().push(new_cell);
                }

                if current.position.row < state.player_position().row
                    && current.position.column == state.player_position().column
                {
                    state.move_up();
                    break;
                } else if current.position.row > state.player_position().row
                    && current.position.column == state.player_position().column
                {
                    state.move_down();
                    break;
                } else if current.position.column < state.player_position().column
                    && current.position.row == state.player_position().row
                {
                    state.move_left();
                    break;
                } else if current.position.column > state.player_position().column
                    && current.position.row == state.player_position().row
                {
                    state.move_right();
                    break;
                }
                closed_list().write().unwrap().push(current);
            } else {
                break;
            }
        }
    }
}

export!(Component);
