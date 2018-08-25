use std::fmt;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
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
            Player::Clear => write!(f, "{}", " "),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicTacToe {
    board: Vec<Vec<Player>>,
    turn: Player,
    players_bench: VecDeque<Player>,
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

        let mut players_bench = VecDeque::new();
        players_bench.push_back(Player::O);
        players_bench.push_back(Player::X);
        
        TicTacToe {
            board, 
            turn,
            players_bench,
        }
    }

    pub fn get_turn(&self) -> Player {
        self.turn
    }

    pub fn next_turn(&mut self) -> Result<(), &'static str> {
        if self.check_game_over() {
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

    pub fn fill(&mut self, row: i32 ,col: i32) -> Result<(), String> {
        if (row > 2) ||
           (row < 0) ||
           (col > 2) ||
           (col < 0) {
            let err = String::from("row and col must be between 0 and 2");
            return Err(err);
        }

        let row = row as usize;
        let col = col as usize;

        if self.board[row][col] != Player::Clear {
            let err = format!(
                "row {} and col {} are occupied",
                row,
                col,
            );

            return Err(err);
        }

        self.board[row][col] = self.turn;
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

    pub fn get_board(&self) -> String {
        let mut board_str = String::new();
        board_str.push_str("   _____C_O_L_____ \n");
        board_str.push_str("  | λ | 0 | 1 | 2 |\n");
        board_str.push_str("  |---|---|---|---|\n");

        let row_0 = format!(
            "R | 0 | {} | {} | {} |\n",
            self.board[0][0],
            self.board[0][1],
            self.board[0][2],
        );
        board_str.push_str(&row_0);

        board_str.push_str("O |---|---|---|---|\n");

        let row_1 = format!(
            "W | 1 | {} | {} | {} |\n",
            self.board[1][0],
            self.board[1][1],
            self.board[1][2],
        );
        board_str.push_str(&row_1);

        board_str.push_str("  |---|---|---|---|\n");

        let row_2 = format!(
            "  | 2 | {} | {} | {} |\n",
            self.board[2][0],
            self.board[2][1],
            self.board[2][2],
        );

        board_str.push_str(&row_2);
        board_str.push_str("   --------------- ");

        board_str
    }

    pub fn get_player_from_bench(&mut self) -> Player {
        self.players_bench.pop_front().unwrap()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Input {
    pub row: i32,
    pub col: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MultiplayerStatus {
    ServerAskInputFromClient(TicTacToe),
    ClientGiveInputToServer(Input),
    GameOver { game: TicTacToe, winner: Player },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start_with_empty_board() {
        use self::Player::Clear;

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
        assert_eq!(result, Ok(()));
        assert_eq!(game.get_turn(), Player::X);
        
        let result = game.next_turn();
        assert_eq!(result, Ok(()));
        assert_eq!(game.get_turn(), Player::O);

        game.fill(0, 0).unwrap();
        game.fill(1, 1).unwrap();
        game.fill(2, 2).unwrap();

        let result = game.next_turn();
        assert_eq!(result, Err("game already over"));
    }

    #[test]
    fn print_board() {
        let game = TicTacToe::new();
        let board = "   _____C_O_L_____ 
  | λ | 0 | 1 | 2 |
  |---|---|---|---|
R | 0 |   |   |   |
O |---|---|---|---|
W | 1 |   |   |   |
  |---|---|---|---|
  | 2 |   |   |   |
   --------------- ";

        assert_eq!(game.get_board(), board);
    }

    #[test]
    fn cannot_fill_outside_range() {
        let mut game = TicTacToe::new();

        let result =  game.fill(-1, 2);
        assert_eq!(result, Err(String::from("row and col must be between 0 and 2")));

        let result =  game.fill(3, 2);
        assert_eq!(result, Err(String::from("row and col must be between 0 and 2")));

        let result =  game.fill(0, -1);
        assert_eq!(result, Err(String::from("row and col must be between 0 and 2")));

        let result =  game.fill(0, 3);
        assert_eq!(result, Err(String::from("row and col must be between 0 and 2")));
    }

    #[test]
    fn cannot_fill_occupied_cell() {
        let mut game = TicTacToe::new();

        let _result = game.fill(0, 0).unwrap();
        let result = game.fill(0, 0);

        assert_eq!(result, Err(String::from("row 0 and col 0 are occupied")));
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
