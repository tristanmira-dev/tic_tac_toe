pub mod app {
    pub mod config;

    #[cfg(test)]
    mod tests;

    use config::{Board, Position, Properties};
    use pancurses::{endwin, initscr, Input, Window};
    use std::fmt::Display;

    //PUT INTO PLAYER STRUCT LATER
    const ALLOWED_INPUTS: [char; 6] = ['x', 'o', 'w', 'a', 's', 'd'];

    fn pause_exit_app(window: &Window) {
        (*window).printw("\nPress any character to exit: ");
        (*window).getch();
        endwin();
    }

    fn handle_invalid_input<T: Copy + Display>(window: &Window, board: &Board<T>) {
        window.clear();
        board.print_board(window);
    }

    fn read_input<F: Fn(char) -> bool, T: Display + Copy>(
        window: &Window,
        board: &Board<T>,
        closure: F,
    ) -> char {
        let input = window.getch();
        match input {
            Some(input) => match input {
                Input::Character(character) => {
                    if !closure(character) {
                        handle_invalid_input(window, board);
                        return read_input(window, board, closure);
                    }
                    return character;
                }
                _ => {
                    handle_invalid_input(window, board);
                    return read_input(window, board, closure);
                }
            },
            None => {
                handle_invalid_input(window, board);
                return read_input(window, board, closure);
            }
        }
    }


    pub fn run() -> () {
        let window: Window = initscr();

        let mut current_pos = Position { x: 0, y: 0 };

        let properties = Properties {
            rows: 3,
            columns: 3,
            board_value: '_',
            cursor: '*',
        };

        let mut board: Board<char> =
            Board::new(properties, Position { x: 0, y: 0 }).unwrap_or_else(|err| panic!("{}", err));

        board.print_board(&window);

        while true {
            let input: char = read_input(&window, &board, |input| {
                return ALLOWED_INPUTS.contains(&input);
            });

            match input {
                'a' => {

                    if ((current_pos.x as i32) - 1) >= 0 {
                        window.clear();
                        current_pos.x -= 1;
                        board.set_cursor(&current_pos);
                        board.print_board(&window);
                    } else {
                        window.clear();
                        board.print_board(&window);
                    }
                    
                }
                'd' => {
                    current_pos.x += 1;
                    board.set_cursor(&current_pos);
                    window.clear();
                    board.print_board(&window);
                }
                'w' => {
                    if ((current_pos.y as i32) - 1) >= 0 {
                        window.clear();
                        current_pos.y -= 1;
                        board.set_cursor(&current_pos);
                        board.print_board(&window);
                    } else {
                        window.clear();
                        board.print_board(&window);
                    }
                }
                's' => {
                    current_pos.y += 1;
                    board.set_cursor(&current_pos);
                    window.clear();
                    board.print_board(&window);
                }
                _ => {}
            }
        }
        pause_exit_app(&window)
    }
}
