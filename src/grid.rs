use std::f64::consts::PI;

use rand::Rng;

use crate::Position;

#[derive(Debug)]
pub struct Grid {
    rows: usize,
    columns: usize,
    player: Position,
    target: Position,
    history: Vec<Position>,
}

fn generate_random_position(rows: u32, columns: u32) -> Position {
    let mut rng = rand::thread_rng();
    let column = rng.gen_range(0..columns);
    let row = rng.gen_range(0..rows);
    Position { column, row }
}

fn calculate_displacement(min_distance: f64) -> (i32, i32) {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..2.0 * PI);
    let dx = (min_distance * angle.cos()).round() as i32;
    let dy = (min_distance * angle.sin()).round() as i32;
    (dx, dy)
}

fn apply_displacement(
    player_position: &Position,
    displacement: (i32, i32),
    rows: u32,
    columns: u32,
) -> Position {
    let (dx, dy) = displacement;
    let new_x = (player_position.column as i32 + dx).rem_euclid(columns as i32) as u32;
    let new_y = (player_position.row as i32 + dy).rem_euclid(rows as i32) as u32;
    Position {
        column: new_x,
        row: new_y,
    }
}

impl Grid {
    pub fn new(rows: u32, columns: u32) -> Self {
        let player_pos = generate_random_position(rows, columns);
        let displacement = calculate_displacement((rows / 2).max(columns / 2) as f64);
        let target_pos = apply_displacement(&player_pos, displacement, rows, columns);

        Self {
            rows: rows as usize,
            columns: columns as usize,
            player: player_pos,
            target: target_pos,
            history: vec![player_pos],
        }
    }

    pub fn move_up(&mut self) {
        if self.player.row == 0 {
            return;
        }

        self.history.push(self.player);
        self.player.row -= 1;
    }

    pub fn move_down(&mut self) {
        if self.player.row as usize >= self.rows {
            return;
        }

        self.history.push(self.player);
        self.player.row += 1;
    }

    pub fn move_right(&mut self) {
        if self.player.column as usize >= self.columns {
            return;
        }

        self.history.push(self.player);
        self.player.column += 1;
    }

    pub fn move_left(&mut self) {
        if self.player.column == 0 {
            return;
        }

        self.history.push(self.player);
        self.player.column -= 1;
    }

    pub fn history(&self) -> Vec<Position> {
        self.history.clone()
    }

    pub fn is_in_history(&self, cell: &Position) -> bool {
        self.history.iter().any(|h| h == cell)
    }

    pub fn player_pos(&self) -> Position {
        self.player
    }

    pub fn target_pos(&self) -> Position {
        self.target
    }

    pub fn adjacent(&self, cell: &Position) -> Vec<Position> {
        let mut adjacent = vec![];

        // Up
        if cell.row > 0 {
            adjacent.push(Position {
                row: cell.row - 1,
                column: cell.column,
            });
        }

        // Down
        if cell.row < self.rows as u32 - 1 {
            adjacent.push(Position {
                row: cell.row + 1,
                column: cell.column,
            });
        }

        // Left
        if cell.column > 0 {
            adjacent.push(Position {
                row: cell.row,
                column: cell.column - 1,
            });
        }

        // Right
        if cell.column < self.columns as u32 - 1 {
            adjacent.push(Position {
                row: cell.row,
                column: cell.column + 1,
            });
        }

        adjacent
    }

    /// NOTE: columns and rows start from 0
    pub fn columns(&self) -> usize {
        self.columns
    }

    /// NOTE: columns and rows start from 0
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// one dimensional grid
    pub fn blocks(&self) -> Vec<()> {
        (0..self.rows * self.columns).map(|_| ()).collect()
    }

    /// two dimensional grid representation
    pub fn grid(&self) -> Vec<Vec<()>> {
        (0..self.rows)
            .map(|_| (0..self.columns).map(|_| ()).collect())
            .collect()
    }

    pub fn has_reached(&self) -> bool {
        self.target == self.player
    }
}
