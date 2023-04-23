mod card;
mod game;
mod strategy;
use std::sync::atomic::{AtomicU64};
use game::*;

static TOTAL_GAMES: AtomicU64 = AtomicU64::new(0);
static TOTAL_WINS: AtomicU64 = AtomicU64::new(0);

struct Handler;
impl GameHandler for Handler {
    fn on_turn<const PLAYERS: usize>(&mut self, game: &Game<PLAYERS>) -> Action {
        if game.player == 0 {
            strategy::smart_move(game)
        } else {
            strategy::random_move(game)
        }
    }
    fn on_win<const PLAYERS: usize>(&mut self, game: &Game<PLAYERS>, winner: usize) {
        if winner == 0 {
            TOTAL_WINS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        TOTAL_GAMES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

fn main() {
    for i in 1.. {
        const PLAYERS: usize = 3;
        let mut game = Game::<PLAYERS>::new();
        game.run(Handler);
        if i % 100000 == 0 {
            // wins / games - 1/3 = advantage
            let advantage = ((TOTAL_WINS.load(std::sync::atomic::Ordering::Relaxed) as f64)
                / (TOTAL_GAMES.load(std::sync::atomic::Ordering::Relaxed) as f64)
                - 1.0 / PLAYERS as f64) * PLAYERS as f64;
            println!(
                "[{}] +{:.2}%",
                TOTAL_GAMES.load(std::sync::atomic::Ordering::Relaxed),
                advantage * 100.0
            )
        }
    }
}
