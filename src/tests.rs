use crate::Position;

use super::Grid;

#[test]
fn test_get_adjacent_cells() {
    let grid = Grid::new(20, 20);

    let test_cases = vec![
        (
            Position { row: 0, column: 0 },
            vec![
                Position { row: 1, column: 0 },
                Position { row: 0, column: 1 },
            ],
        ), // Top-left corner
        (
            Position { row: 0, column: 19 },
            vec![
                Position { row: 1, column: 19 },
                Position { row: 0, column: 18 },
            ],
        ), // Top-right corner
        (
            Position { row: 19, column: 0 },
            vec![
                Position { row: 18, column: 0 },
                Position { row: 19, column: 1 },
            ],
        ), // Bottom-left corner
        (
            Position {
                row: 19,
                column: 19,
            },
            vec![
                Position {
                    row: 18,
                    column: 19,
                },
                Position {
                    row: 19,
                    column: 18,
                },
            ],
        ), // Bottom-right corner
        (
            Position {
                row: 10,
                column: 10,
            },
            vec![
                Position { row: 9, column: 10 },
                Position {
                    row: 11,
                    column: 10,
                },
                Position { row: 10, column: 9 },
                Position {
                    row: 10,
                    column: 11,
                },
            ],
        ), // Middle of grid
    ];

    for (position, expected) in test_cases {
        let result = grid.adjacent(&position);
        assert_eq!(
            result, expected,
            "Test case failed for position {:?}",
            position
        );
    }
}
