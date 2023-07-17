use std::str::FromStr;

use chess::{Board, ChessMove, Color, BoardStatus};

use crate::engine::Engine;

pub struct Game {
    board: Board,
    white_engine: Engine,
    black_engine: Engine,
}

impl Game {
    pub fn new(white_engine: &str, black_engine: &str) -> Game {
        return Game {
            board: Board::default(),
            white_engine: Engine::new(white_engine),
            black_engine: Engine::new(black_engine),
        }
    }

    pub fn play(&mut self) -> Board {
        let mut len_moves = 0;
        while self.board.status() == BoardStatus::Ongoing {
            let next_move: String;
            if self.board.side_to_move() == Color::Black {
                next_move = self.black_engine.calculate_move(self.board.to_string().as_str());
            } else {
                next_move = self.white_engine.calculate_move(self.board.to_string().as_str());
            }

            let chess_move = ChessMove::from_str(next_move.as_str()).unwrap();
            self.board = self.board.make_move_new(chess_move);
            len_moves += 1;

            if len_moves >= 50 {
                break;
            }
        }

        self.white_engine.stop();
        self.black_engine.stop();

        return self.board;
    }
}
