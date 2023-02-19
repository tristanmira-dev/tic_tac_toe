use super::*;

fn init_board(rows: i32, columns: i32) -> Board<char> {
    let properties: Properties<char> = Properties {
        rows,
        columns,
        board_value: '_',
        cursor: '*',
    };

    let default_position: Position = Position { x: 0, y: 0 };

    let board: Board<char> =
        Board::new(properties, default_position).unwrap_or_else(|err| panic!("{}", err));

    board
}

#[test]
fn set_piece_success() {
    let pos = Position { x: 1, y: 2 };
    let mut board: Board<char> = init_board(3, 3);
    let result: bool = board.set_player_piece(&pos, 'X');
    assert_eq!(result, true);
}

#[test]
fn set_piece_fail() {
    let pos = Position { x: 1, y: 2 };
    let mut board: Board<char> = init_board(3, 3);
    let result: bool = board.set_player_piece(&pos, 'X');
    assert_eq!(result, false);
}
