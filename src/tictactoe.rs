pub struct TicTacToe {
    game: u32,
    board: Vec<char>,
    win_states: Vec<u32>,
    draw_state: u32,
    multiplayer: bool,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            game: 0,
            win_states: vec![
                0b100_1001,
                0b1001_0010,
                0b1_0010_0100,
                0b111,
                0b11_1000,
                0b1_1100_0000,
                0b1_0001_0001,
                0b101_0100,
            ],
            draw_state: 0x1ff,
            board: vec![' ';9],
            multiplayer: true,
        }
    }

    fn alpha_beta_pruning(&self, mut game_state: u32, depth: u32, mut alpha: i32, mut beta: i32, is_max: bool) -> (i32, usize){
        let moves = self.gen_moves(&mut game_state, is_max);
        //let player = game_state.0 & 0x200;
        
        if depth == 0 || moves.is_empty() {
            println!("Evaluating state: {:b}", game_state);
            let score = Self::eval(game_state);
            println!("Score: {}", score);
            //println!("Leaf Node Player: {:b}", player);
            return (score , 0)
        }

        //println!("Bitboard: {:b}", game_state.0);
        //println!("Root Node Player: {:b}", player);
        let mut best_move = 0;

        for bit_move in moves {
            //println!("BitBoard Before : {:b}", game_state);
            //println!("Move Bit # : {}", bit_move);
            game_state = game_state ^ 1 << bit_move;
            //println!("BitBoard After  : {:b}", game_state);
            let (score, _) = self.alpha_beta_pruning(game_state, depth - 1, alpha, beta, !is_max);
            //println!("Player: {:b}", player);
            //println!("Game state observing: {:b}", game_state.0);
            if !is_max {
                if beta > score {
                    beta = score;    
                    best_move = bit_move;
                }
            }
            else {
                if alpha < score {
                    alpha = score;
                    best_move = bit_move;
                }
            }
            game_state = game_state ^ 1 << bit_move;

            if alpha >= beta {
                break;
            }
        }
        //println!("Bitboard: {:b}", best_state.0);
        //println!("Bit: {}", best_state.1);

        if !is_max {
            println!("Picked = {}, b = {}", best_move, beta);
            (beta, best_move)
        }
        else {
            println!("Picked = {}, a = {}", best_move, alpha);
            (alpha, best_move)
        }
    }

    fn gen_moves(&self, game_state: &mut u32, is_max: bool) -> Vec<usize>{
        let mut moves: Vec<usize> = vec![];
        //*game_state ^= 0x200;
        //let turn = *game_state & 0x200;
        for bit in 0..9 {
            if Self::check_square(bit, *game_state) {
                if is_max {
                    //println!("P1: {:b}", *game_state ^  1 << bit);
                    moves.push(bit + 16);
                }
                else {
                    //println!("P2: {:b}", *game_state ^ 1 << (bit + 16));
                    moves.push(bit);
                }
            }
        }
        moves
    }

    fn eval(game_state: u32) -> i32 {
        let mut score = 0;
        //let row = 0b111;
        //let col = 0b1001001;
        //let diag1 = 0b1010100;
        //let diag2 = 0b100010001;

        for disp in 0..3 {
            println!("Check row {}", disp);
            score += Self::eval_line(game_state, disp * 3, 1);
            println!("Score: {}", score);
            println!("Check col {}", disp);
            score += Self::eval_line(game_state, disp, 3);
            println!("Score: {}", score);
        }
        println!("Check diagonal 0-4-8");
        score += Self::eval_line(game_state, 0, 4);
        println!("Check diagonal 2-4-6");
        score += Self::eval_line(game_state, 2, 2);
        score
    }
    /*
    012 345 678
    
    036 147 258 
    
    048 246
    */

    fn eval_line(game_state: u32, disp: u32, multiple: u32) -> i32 {
        let mut score = 0;
        println!("Full board {:b}", game_state);
        for i in 0..3 {
            let p1_cell = game_state & 1 << disp + i * multiple;
            let ai_cell = game_state & 1 << disp + i * multiple + 16;
            println!("Cell {} P1 {:b}", i, p1_cell);
            println!("Cell {} AI {:b}", i, ai_cell);
            if i == 0 {
                // o??
                if ai_cell != 0 {
                    score = 1;
                }
                // x??
                else if p1_cell != 0 {
                    score = -1;
                }
                // else score = 0
            }
            else if i == 1 {
                if ai_cell != 0 {
                    // oo?
                    if score == 1 {
                        score = 10;
                    }
                    // xo?
                    else if score == -1 {
                        return 0;
                    }
                    // -o?
                    else {
                        score = 1;
                    }
                }
                else if p1_cell != 0 {
                    // xx? 
                    if score == -1 {
                        score = -10;
                    }
                    // ox?
                    else if score == 1 {
                        return 0;
                    }
                    // -x?
                    else {
                        score = -1;
                    }
                }
                else {
                    // o-?
                    if score == 1 {
                        score = 1;
                    }
                    // x-?
                    else if score == -1 {
                        score = -1;
                    }
                    // --?
                    else {
                        score = 0;
                    }
                }
            }
            else {
                if ai_cell != 0 {
                    // ooo
                    if score == 10 {
                        score *= 10;
                    }
                    // -oo
                    // o-o
                    else if score > 0 {
                        score *= 5;
                    }
                    // --o
                    else {
                        score = 1;
                    }
                }
                else if p1_cell != 0 {
                    // xxx
                    if score == -10 {
                        score *= 10;
                    }
                    // -xx
                    // x-x
                    else if score > 0 {
                        score *= 5;
                    }
                    // --x
                    else {
                        score = -1;
                    }
                }
                // else 
                // ---
                // xx-
                // oo-
                // x--
                // o--
            }
        }
        score 
    }

    fn game_won(&self) -> bool {
        for state in &self.win_states {
            if self.game & *state == *state {
                println!("Player 1 wins!");
                return true
            }
            else if self.game & (*state << 16) == *state << 16 {
                println!("Player 2 wins!");
                return true
            }
        }
        false
    }

    fn game_draw(&self) -> bool {
        if (self.game | (self.game >> 16)) & 0x1ff == self.draw_state {
            println!("This game was a draw!");
            return true
        }
        false
    }

    fn game_start(&self) -> bool {
        (self.game & 1 << 31) == 0
    }

    fn draw(&self) {
        println!("  6 | 7 | 8  \n"); 
        println!("-------------\n"); 
        println!("  3 | 4 | 5  \n"); 
        println!("-------------\n"); 
        println!("  0 | 1 | 2  \n\n"); 
        println!("=============\n"); 
        println!("  {} | {} | {}  \n", self.board[6], self.board[7], self.board[8]); 
        println!("-------------\n"); 
        println!("  {} | {} | {}  \n", self.board[3], self.board[4], self.board[5]); 
        println!("-------------\n"); 
        println!("  {} | {} | {}  \n\n", self.board[0], self.board[1], self.board[2]); 
    }


    fn make_move(&mut self) {
        let turn = self.game & 0x200;
        let mut symbol = 'x';
        let mut disp = 0;

        if turn == 0 {
            println!("Player 1's turn");
        }
        else {
            println!("Player 2's turn");
            symbol = 'o';
            disp = 16;
        }

        if self.multiplayer || turn == 0 {
            let sq_num = self.get_player_input();
            self.game ^= 1 << (sq_num + disp);
            self.board[sq_num] = symbol;
        }
        else {
            let (_, bit) = self.alpha_beta_pruning(self.game, 4, i32::min_value(), i32::max_value(), true);
            self.game = self.game ^ 1 << bit;
            self.board[bit - 16] = symbol;
            println!("{:b}", self.game);
        }
        self.game ^= 0x200;
    }

    fn check_square(sq_num: usize, game_state: u32) -> bool {
        (game_state & 1 << sq_num) == 0 && (game_state & 1 << (sq_num + 16)) == 0 && sq_num < 9
    }

    fn get_player_input(&self) -> usize{
        loop {
            let mut input = String::new();
            if let Ok(_) = std::io::stdin().read_line(&mut input) {
                if let Ok(sq_num) = input.trim().parse::<usize>() {
                    if Self::check_square(sq_num, self.game) {
                        return sq_num;
                    }
                }
            }
            println!("Invalid input. Please input a valid square number: ")
        }
    }

    pub fn start(&mut self) {
        println!("Please pick a mode by entering the corresponding number: ");
        println!("1. Single Player");
        println!("2. Multiplayer");
        let mut input = String::new();
        if let Ok(_) = std::io::stdin().read_line(&mut input) {
            if let Ok(s) = input.trim().parse::<usize>() {
                if s == 1 {
                    self.multiplayer = false;
                }
            }
        }
        self.draw();
        while self.game_start() 
            && !self.game_won() 
                && !self.game_draw() {
                    self.make_move();
                    self.draw();
                }
    }
}

