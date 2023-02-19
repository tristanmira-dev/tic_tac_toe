use core::fmt::Debug;
use pancurses::Window;
use std::fmt::Display;

#[derive(Debug)]
pub struct Properties<T: Copy + Display> {
    pub rows: i32,
    pub columns: i32,
    pub board_value: T,
    pub cursor: T,
}

pub enum Movement {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Board<T: Copy + Display> {
    dimensions: Vec<Vec<T>>,
    position: Position,
    properties: Properties<T>,
}

impl<T: Copy + Display> Board<T> {
    pub fn new(properties: Properties<T>, position: Position) -> Result<Self, String> {
        if properties.rows == properties.columns && properties.rows > 2 {
            let mut dimensions: Vec<Vec<T>> = Vec::new();

            for _x in 0..properties.rows {
                let mut column: Vec<T> = Vec::new();
                for _y in 0..properties.columns {
                    column.push(properties.board_value);
                }
                dimensions.push(column);
            }

            Ok(Board {
                dimensions,
                position,
                properties,
            })
        } else {
            Err("Invalid Dimensions.".to_string())
        }
    }

    pub fn print_board(self: &Self, current_window: &Window) {
        let mut full_board: String = "".to_string();

        for (row_index, row_value) in self.dimensions.iter().enumerate() {
            let mut row = "".to_string();

            for (column_index, column_value) in row_value.iter().enumerate() {
                let column_count = column_index + 1;
                let casted_column = column_value.to_string();

                let location_tuple = (self.position.y as usize, self.position.x as usize);
                if (row_index, column_index) == location_tuple {
                    if column_count == (self.properties.columns as usize) {
                        let mut current_location: String = self.properties.cursor.to_string();
                        current_location.push('\n');
                        row.push_str(&current_location);
                    } else {
                        let mut current_location: String = self.properties.cursor.to_string();
                        current_location.push('|');
                        row.push_str(&current_location);
                    }
                } else if column_count == (self.properties.columns as usize) {
                    let mut end_row: String = casted_column.clone();
                    end_row.push('\n');
                    row.push_str(&end_row);
                } else {
                    let mut start_row: String = casted_column.clone();
                    start_row.push('|');
                    row.push_str(&start_row);
                }
            }

            full_board.push_str(&row);
        }

        current_window.printw(&full_board);
    }

    pub fn set_player_piece(self: &mut Self, pos: &Position, current_player: T) -> bool {
        match self.dimensions.get_mut(pos.y) {
            Some(val) => match val.get_mut(pos.x) {
                Some(val) => {
                    *val = current_player;
                    return true;
                }
                None => return false,
            },
            None => return false,
        }
    }

    pub fn get_cursor(self: &mut Self) -> &Position {
        return &self.position;
    }

    pub fn re_render_cursor(self: &mut Self, window: &Window, move_state: Movement) {
        window.clear();
        
        self.print_board(window);
    }

    pub fn set_cursor(self: &mut Self, position: &Position) -> bool {
        if (self.properties.columns as usize) < position.y {
            return false;
        }
        if (self.properties.rows as usize) < position.x {
            return false;
        }

        self.position.x = position.x;
        self.position.y = position.y;
        return true;
    }
}
