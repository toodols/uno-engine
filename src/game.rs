use rand::seq::SliceRandom;

use super::card::*;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Card(Card),
    Pass,
}

lazy_static::lazy_static! {
    static ref DEFAULT_POOL: Vec<Card> = {
        let mut pool = Vec::with_capacity(108);
        for color in [Color::Red, Color::Blue, Color::Green, Color::Yellow].into_iter() {
            pool.push((Value::Zero, color));
            for _ in 0..2 {
                pool.push((Value::One, color));
                pool.push((Value::Two, color));
                pool.push((Value::Three, color));
                pool.push((Value::Four, color));
                pool.push((Value::Five, color));
                pool.push((Value::Six, color));
                pool.push((Value::Seven, color));
                pool.push((Value::Eight, color));
                pool.push((Value::Nine, color));
                pool.push((Value::Block, color));
                pool.push((Value::Reverse, color));
                pool.push((Value::PlusTwo, color));
            }
        }
        for _ in 0..4 {
            pool.push((Value::PlusFour, Color::None));
            pool.push((Value::Wild, Color::None));
        }
        pool
    };
}

pub struct Game<const PLAYERS: usize> {
    pub player: usize,
    pool: Vec<Card>,
    trash: Vec<Card>,
    players: [Vec<Card>; PLAYERS],
    pub player_selected: Option<Card>,
    pub stack_value: usize,
    rng: rand::rngs::ThreadRng,
    pub direction: Direction,
}

pub enum Direction {
    Forward,
    Backward,
}

pub trait GameHandler {
    fn on_turn<const PLAYERS: usize>(&mut self, state: &Game<PLAYERS>) -> Action;
    fn on_win<const PLAYERS: usize>(&mut self, state: &Game<PLAYERS>, winner: usize);
}

impl<const PLAYERS: usize> Game<PLAYERS> {
    pub fn current_player(&self) -> &Vec<Card> {
        &self.players[self.player]
    }
    pub fn top_card(&self) -> Card {
        *self.trash.last().unwrap()
    }
    fn advance(&mut self, turns: usize) {
        match self.direction {
            Direction::Forward => self.player = (self.player + turns) % PLAYERS,
            Direction::Backward => self.player = (self.player + PLAYERS - turns) % PLAYERS,
        }
    }
    pub fn run(&mut self, mut handler: impl GameHandler) {
        loop {
            let player = self.player;
            if self.stack_value > 0
                && !self.players[player]
                    .iter()
                    .any(|e| e.0 == self.top_card().0)
            {
                self.draw_into(player, self.stack_value);
                self.stack_value = 0;
                self.advance(1);
                continue;
            }

            let action = handler.on_turn(&self);
            match action {
                Action::Card(card) => {
                    if cfg!(validate_actions) {
                        if self.stack_value > 0 && card.0 != self.top_card().0 {
                            panic!("Must stack or draw");
                        }
                        if !can_follow(self.top_card(), card) {
                            panic!("{:?} cannot follow {:?}", card, self.top_card());
                        }
                        if card.1 == Color::None {
                            panic!("Must specify color");
                        }
                    }
                    if self.player_selected.is_some() {
                        let selected = *self.player_selected.as_ref().unwrap();
                        if cfg!(validate_actions) {
                            if selected != card && selected.1 != Color::None {
                                panic!("Must use card ");
                            }
                        }
                        self.player_selected = None;
                    } else {
                        self.players[player].remove(
                            self.players[player]
                                .iter()
                                .position(|c| c == &card || c.1 == Color::None)
                                .expect(
                                    format!(
                                        "Card {:?} not found in {:?}",
                                        card, self.players[player]
                                    )
                                    .as_str(),
                                ),
                        );
                    }

                    self.trash.push(card);

                    match card {
                        (Value::Block, _) => {
                            self.advance(1);
                        }
                        (Value::Reverse, _) => {
                            self.direction = match self.direction {
                                Direction::Forward => Direction::Backward,
                                Direction::Backward => Direction::Forward,
                            };
                        }
                        (Value::PlusTwo, _) => {
                            self.stack_value += 2;
                        }
                        (Value::PlusFour, _) => {
                            self.stack_value += 4;
                        }
                        _ => {}
                    }
                    self.advance(1);
                }
                Action::Pass => {
                    if self.player_selected.is_some() {
                        self.players[player].push(self.player_selected.take().unwrap());
                        self.advance(1);
                    } else {
                        let card = self.draw();
                        if can_follow(self.top_card(), card) {
                            self.player_selected = Some(card);
                        } else {
                            self.players[player].push(card);
                            self.advance(1);
                        }
                    }
                }
            }
            // player wins when they have 0 cards
            if self.players[player].len() == 0 {
                handler.on_win(&self, player);
                break;
            }
        }
    }
    fn draw(&mut self) -> Card {
        let card = match self.pool.pop() {
            Some(card) => card,
            None => {
                // move trash into pool if pool is empty
                self.pool = self
                    .trash
                    .drain(..self.trash.len() - 1)
                    .map(|card| {
                        if card.0 == Value::Wild || card.0 == Value::PlusFour {
                            (card.0, Color::None)
                        } else {
                            card
                        }
                    })
                    .collect();
                self.pool.shuffle(&mut self.rng);
                self.pool.pop().unwrap()
            }
        };
        card
    }
    fn draw_into(&mut self, player: usize, amount: usize) {
        for _ in 0..amount {
            let card = self.draw();
            self.players[player].push(card);
        }
    }
    pub fn card_count(&self, player: usize) -> usize {
        self.players[player].len()
    }
    pub fn new() -> Self {
        let mut pool = DEFAULT_POOL.clone();
        let mut rng = rand::thread_rng();
        pool.shuffle(&mut rng);
        let mut game = Self {
            player: 0,
            trash: vec![pool.pop().unwrap()],
            pool,
            players: vec![vec![]; PLAYERS].try_into().unwrap(),
            rng,
            player_selected: None,
            direction: Direction::Forward,
            stack_value: 0,
        };
        for player_num in 0..game.players.len() {
            game.draw_into(player_num, 7);
        }
        game
    }
}
