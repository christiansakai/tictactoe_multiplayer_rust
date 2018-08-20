use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    O,
    X,
    Clear,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::O => write!(f, "{}", "O"),
            Player::X => write!(f, "{}", "X"),
            Player::Clear => write!(f, "{}", "-"),
        }
    }
}

#[derive(Debug)]
pub struct TicTacToe {
    board: Vec<Vec<Player>>,
    turn: Player,
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        let mut board: Vec<Vec<Player>> = Vec::new();

        for _r in 0..3 {
            let mut row: Vec<Player> = vec![
                Player::Clear,
                Player::Clear,
                Player::Clear,
            ];

            board.push(row);
        }

        let turn = Player::O;
        
        TicTacToe {
            board, 
            turn,
        }
    }

    pub fn get_turn(&self) -> Player {
        self.turn
    }

    pub fn next_turn(&mut self) -> Result<(), &'static str> {
        if !self.check_game_over() {
            Err("game already over")
        } else {
            if self.turn == Player::X {
                self.turn = Player::O;
            } else if self.turn == Player::O {
                self.turn = Player::X;
            }

            Ok(())
        }
    }

    pub fn fill(&mut self, row: i32 ,col: i32) -> Result<(), &'static str> {
        if (row > 2) ||
           (row < 0) ||
           (col > 2) ||
           (col < 0) {
            return Err("row and col must be between 0 and 2");
        }

        self.board[row as usize][col as usize] = self.turn;
        Ok(())
    }

    pub fn check_game_over(&self) -> bool {
        self.check_rows() ||
        self.check_cols() ||
        self.check_topleft_bottom_right() ||
        self.check_bottomleft_topright()
    }

    fn check_rows(&self) -> bool {
        for r in 0..3 {
            let mut count = 1;
            let slot = self.board[r][0];

            for c in 1..3 {
                let cell = self.board[r][c];
                if cell != Player::Clear && cell == slot {
                    count += 1;
                }
            }


            if count == 3 {
                return true;
            }
        }

        false
    }

    fn check_cols(&self) -> bool {
        for c in 0..3 {
            let mut count = 1;
            let slot = self.board[0][c];

            for r in 1..3 {
                let cell = self.board[r][c];
                if cell != Player::Clear && cell == slot {
                    count += 1;
                }
            }

            if count == 3 {
                return true;
            }
        }

        false
    }

    fn check_topleft_bottom_right(&self) -> bool {
        let mut count = 1;
        let slot = self.board[0][0];

        for x in 1..3 {
            let row = x;
            let col = x;
            let cell = self.board[row][col];

            if cell != Player::Clear && cell == slot {
                count += 1;
            }
        }

        count == 3
    }

    fn check_bottomleft_topright(&self) -> bool {
        let mut count = 1;
        let slot = self.board[0][2];

        for x in 1..3 {
            let row = 0 + x;
            let col = 2 - x;
            let cell = self.board[row][col];

            if cell != Player::Clear && cell == slot {
                count += 1;
            }
        }

        count == 3
    }

    pub fn print_board(&self) {
        println!("Turn: {}", self.turn);

        println!("Board");
        for row in &self.board {
            for cell in row {
                print!("{}", cell);
            }
            println!("");
        }

        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start_with_empty_board() {
        use Player::Clear;

        let game = TicTacToe::new();
        let board = vec![
            vec![Clear, Clear, Clear],
            vec![Clear, Clear, Clear],
            vec![Clear, Clear, Clear],
        ];

        assert_eq!(game.board, board);
    }

    #[test]
    fn start_with_player_o() {
        let game = TicTacToe::new();
        assert_eq!(game.turn, Player::O);
    }

    #[test]
    fn turn_based_player() {
        let mut game = TicTacToe::new();

        let result = game.next_turn();
        if let Ok(_) = result {
            assert_eq!(game.get_turn(), Player::X);
        }
        
        let result = game.next_turn();
        if let Ok(_) = result {
            assert_eq!(game.get_turn(), Player::O);
        }
    }

    #[test]
    fn cannot_fill_outside_range() {
        let mut game = TicTacToe::new();

        let result =  game.fill(-1, 2);
        assert_eq!(result, Err("row and col must be between 0 and 2"));

        let result =  game.fill(3, 2);
        assert_eq!(result, Err("row and col must be between 0 and 2"));

        let result =  game.fill(0, -1);
        assert_eq!(result, Err("row and col must be between 0 and 2"));

        let result =  game.fill(0, 3);
        assert_eq!(result, Err("row and col must be between 0 and 2"));
    }

    #[test]
    fn check_for_row_win() {
        for r in 0..3 {
            let mut game = TicTacToe::new();

            assert_eq!(game.check_rows(), false);
            game.fill(r, 0).unwrap();
            game.fill(r, 1).unwrap();
            game.fill(r, 2).unwrap();
            assert_eq!(game.check_rows(), true);
        }
    }

    #[test]
    fn check_for_col_win() {
        for c in 0..3 {
            let mut game = TicTacToe::new();

            assert_eq!(game.check_cols(), false);
            game.fill(0, c).unwrap();
            game.fill(1, c).unwrap();
            game.fill(2, c).unwrap();
            assert_eq!(game.check_cols(), true);
        }
    }

    #[test]
    fn check_topleft_bottom_right_win() {
        let mut game = TicTacToe::new();

        assert_eq!(game.check_topleft_bottom_right(), false);
        game.fill(0, 0).unwrap();
        game.fill(1, 1).unwrap();
        game.fill(2, 2).unwrap();
        assert_eq!(game.check_topleft_bottom_right(), true);
    }

    #[test]
    fn check_bottomleft_topright_win() {
        let mut game = TicTacToe::new();

        assert_eq!(game.check_bottomleft_topright(), false);
        game.fill(0, 2).unwrap();
        game.fill(1, 1).unwrap();
        game.fill(2, 0).unwrap();
        assert_eq!(game.check_bottomleft_topright(), true);
    }

    #[test]
    fn check_game_over() {
        let mut game = TicTacToe::new();

        assert_eq!(game.check_game_over(), false);
        game.fill(0, 2).unwrap();
        game.fill(1, 1).unwrap();
        game.fill(2, 0).unwrap();
        assert_eq!(game.check_game_over(), true);
    }
}
