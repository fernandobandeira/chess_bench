use chess::Board;
use rayon::prelude::*;

use game::Game;

pub mod engine;
pub mod game;

const N_GAMES: u32 = 100;
const N_THREADS: u32 = 10;

fn main() {
    let mut white_victories: u32 = 0;
    let mut black_victories: u32 = 0;
    let mut draws: u32 = 0;

    let mut total_games = 0;
    while total_games < N_GAMES {
        let remaining_games = N_GAMES - total_games;
        let spawn_threads = if remaining_games > N_THREADS { N_THREADS } else { remaining_games };

        let results: Vec<Board> = (0..spawn_threads)
            .into_par_iter()
            .map(|_| Game::new("engines/chess_engine", "engines/chess_engine").play())
            .collect();

        for result in results {
            total_games += 1;
            match result.status() {
                chess::BoardStatus::Checkmate => {
                    if result.side_to_move() == chess::Color::White {
                        black_victories += 1;
                    } else {
                        white_victories += 1;
                    }
                },
                _ => {
                    draws += 1
                },
            }
        }
    }

    println!("White victories: {}", white_victories);
    println!("Black victories: {}", black_victories);
    println!("Draws: {}", draws);
}
