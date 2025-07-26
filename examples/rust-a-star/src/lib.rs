use std::cell::RefCell;

use component::wasmpath::game::{Position, State};
use exports::component::wasmpath::solution::{Direction, Guest, GuestSolver};

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

struct Solver {
    open_list: RefCell<Vec<Cell>>,
    closed_list: RefCell<Vec<Cell>>,
}

impl Guest for Solver {
    type Solver = Solver;
}

impl GuestSolver for Solver {
    fn new(state: &State) -> Self {
        Solver {
            open_list: RefCell::new(vec![Cell {
                cost_from_starting_point: 0,
                total_cost: 0,
                position: state.player_position(),
            }]),
            closed_list: RefCell::new(Vec::new()),
        }
    }

    fn step(&self, state: &State) -> Option<Direction> {
        loop {
            let min = self
                .open_list
                .borrow()
                .iter()
                .copied()
                .min_by_key(|c| c.total_cost);

            if let Some(current) = min {
                self.open_list
                    .borrow_mut()
                    .retain(|c| c.position != current.position);

                for cell in state.adjacent_cells(current.position) {
                    if cell == state.target_position() {
                        self.open_list.borrow_mut().push(Cell {
                            cost_from_starting_point: 0,
                            total_cost: 0,
                            position: cell,
                        });
                        break;
                    }

                    if self.open_list.borrow().iter().any(|c| c.position == cell)
                        || self.closed_list.borrow().iter().any(|c| c.position == cell)
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

                    self.open_list.borrow_mut().push(new_cell);
                }

                if current.position.row < state.player_position().row
                    && current.position.column == state.player_position().column
                {
                    break Some(Direction::Up);
                } else if current.position.row > state.player_position().row
                    && current.position.column == state.player_position().column
                {
                    break Some(Direction::Down);
                } else if current.position.column < state.player_position().column
                    && current.position.row == state.player_position().row
                {
                    break Some(Direction::Left);
                } else if current.position.column > state.player_position().column
                    && current.position.row == state.player_position().row
                {
                    break Some(Direction::Right);
                }

                self.closed_list.borrow_mut().push(current);
            } else {
                break Some(Direction::Right);
            }
        }
    }
}

export!(Solver);
