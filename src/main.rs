mod card;
mod game;
mod strategy;
use std::{sync::atomic::{AtomicU64}, thread, time::Duration};
use game::*;

use crate::card::{Value, Color};

static TOTAL_GAMES: AtomicU64 = AtomicU64::new(0);
static TOTAL_WINS: AtomicU64 = AtomicU64::new(0);

struct Handler;
impl GameHandler for Handler {
    fn on_turn<const PLAYERS: usize>(&mut self, game: &Game<PLAYERS>) -> Action {
        if game.player == PLAYERS-1 {
            strategy::smart_move(game)
        } else {
            strategy::random_move(game)
        }
    }
    fn on_win<const PLAYERS: usize>(&mut self, game: &Game<PLAYERS>, winner: usize) {
        if winner == PLAYERS-1 {
            TOTAL_WINS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        TOTAL_GAMES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

fn main() {

    // println!("Validate Actions: {}", cfg!(feature="validate_actions"));

    const PLAYERS: usize = 10;
    let mut handles = vec![
        thread::spawn(||{
            loop {
                thread::sleep(Duration::from_secs(10));
                let total_games_f64 = TOTAL_GAMES.load(std::sync::atomic::Ordering::Relaxed) as f64;
                let total_wins_f64 = TOTAL_WINS.load(std::sync::atomic::Ordering::Relaxed) as f64;
                let advantage = PLAYERS as f64 * total_wins_f64 / total_games_f64 - 1.0;
                if advantage >= 0.0 {
                    println!("[{}] +{:.2}%", total_games_f64, advantage * 100.0);
                } else {
                    println!("[{}] {:.2}%", total_games_f64, advantage * 100.0);
                }
            }
        })
    ];
    for _ in 0..4 {
        handles.push(thread::spawn(|| {
            loop {
                let mut game = Game::<PLAYERS>::new();
                for i in 0..7 {
                    game.get_player_mut(0).truncate(0);
                    game.get_player_mut(0).push((Value::DrawFour, Color::None));
                }
                game.run(Handler);
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
