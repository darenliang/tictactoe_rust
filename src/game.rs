use std::io;

pub struct Game {
    board: [isize; 9],
    winner: isize,
    last_moves: Vec<usize>,
    win_positions: [[usize; 3]; 8],
    human_marker: isize,
    ai_marker: isize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [0; 9],
            winner: 0,
            last_moves: Vec::new(),
            win_positions: [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6], [1, 4, 7], [2, 5, 8], [0, 4, 8], [2, 4, 6]],
            human_marker: 1,
            ai_marker: -1,
        }
    }

    fn print_board(&self) {
        for i in 0..9 {
            if i % 3 == 0 && i != 0 {
                println!();
            }
            print!("{}", format!("{: >2}", self.board[i]));
        }
        println!();
    }

    fn get_free_positions(&self) -> Vec<usize> {
        let mut free_moves: Vec<usize> = Vec::new();
        for i in 0..9 {
            if self.board[i] == 0 {
                free_moves.push(i);
            }
        }
        free_moves
    }

    fn mark(&mut self, marker: isize, pos: usize) {
        self.board[pos] = marker;
        self.last_moves.push(pos);
    }

    fn revert_last_move(&mut self) {
        let pop = self.last_moves.pop();
        let mut index: usize = 0;
        match pop {
            None => {}
            Some(num) => index = num,
        }
        self.board[index] = 0;
        self.winner = 0;
    }

    fn game_over(&mut self) -> bool {
        for i in 0..8 {
            if self.board[self.win_positions[i][0]] != 0 && self.board[self.win_positions[i][0]] == self.board[self.win_positions[i][1]] && self.board[self.win_positions[i][1]] == self.board[self.win_positions[i][2]] {
                self.winner = self.board[self.win_positions[i][0]];
                return true;
            }
        }
        if !self.board.contains(&0) {
            self.winner = 0;
            return true;
        }
        false
    }

    fn move_human(&mut self) {
        let move_human: usize;
        loop {
            println!("Input position:");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {}
                Err(_) => {
                    println!("Failed to read line");
                    continue;
                }
            }
            let parsed_input = input.trim().parse();
            let input: usize;
            match parsed_input {
                Ok(num) => input = num,
                Err(_) => {
                    println!("Enter a valid integer");
                    continue;
                }
            }
            if self.get_free_positions().contains(&input) {
                move_human = input;
                break;
            } else {
                println!("{} is an invalid move. Please try again.", input);
            }
        }
        self.mark(self.human_marker, move_human);
    }

    fn move_ai(&mut self) {
        let (move_position, _) = self.maximized_move();
        self.mark(self.ai_marker, move_position)
    }

    fn maximized_move(&mut self) -> (usize, isize) {
        let mut curr_score: isize;
        let (mut best_move, mut best_score) = (0usize, -1isize);
        for &v in self.get_free_positions().iter() {
            self.mark(self.ai_marker, v);
            if self.game_over() {
                curr_score = self.get_score();
            } else {
                curr_score = self.minimized_move().1;
            }
            self.revert_last_move();
            if best_score == -1 || curr_score > best_score {
                best_score = curr_score;
                best_move = v;
            }
        }
        (best_move, best_score)
    }

    fn minimized_move(&mut self) -> (usize, isize) {
        let mut curr_score: isize;
        let mut best_move: usize = 0;
        let mut best_score: isize = 1;
        for &v in self.get_free_positions().iter() {
            self.mark(self.human_marker, v);
            if self.game_over() {
                curr_score = self.get_score();
            } else {
                curr_score = self.maximized_move().1;
            }
            self.revert_last_move();
            if best_score == 1 || curr_score < best_score {
                best_score = curr_score;
                best_move = v;
            }
        }
        (best_move, best_score)
    }

    fn get_score(&mut self) -> isize {
        if self.game_over() {
            if self.winner == self.ai_marker {
                return 1;
            } else if self.winner == self.human_marker {
                return -1;
            }
        }
        0
    }

    pub fn play(&mut self, f: isize) {
        for i in 0..9 {
            self.print_board();
            if (i % 2 == 0 && f == 1) || (i % 2 != 0 && f == -2) {
                println!("Player move");
                self.move_human();
            } else {
                println!("Computer move");
                self.move_ai();
            }
            if self.game_over() {
                self.print_board();
                println!("Winner: {}\n", self.winner);
                break;
            }
        }
    }
}
