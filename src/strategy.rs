use super::card::*;
use super::game::*;

fn most_common_color(hand: &Vec<Card>) -> Color {
	let counts = hand.iter().fold([0; 4], |mut acc, card| {
		if card.1 != Color::None {
			acc[card.1.num()] += 1;
		}
		acc
	});
	let index = counts.iter().enumerate().max_by_key(|(_, &v)| v).unwrap().0;
	Color::from_num(index)
}

pub fn smart_move<const PLAYERS: usize>(game: &Game<PLAYERS>) -> Action {
    let player = game.current_player();
    let action = if game.player_selected.is_some() {
        let card = game.player_selected.unwrap();
        if card.0 == Value::PlusFour || card.0 == Value::Wild {
            if player.len() == 1 {
                Action::Card((card.0, player[0].1))
            } else if game.card_count(game.player + 1 % PLAYERS) == 1 {
                Action::Card((card.0, most_common_color(player)))
            } else {
				Action::Pass
			}
        } else {
            Action::Card(card)
        }
    } else {
        let mut action = Action::Pass;
        for card in player {
            if game.stack_value == 0 && can_follow(game.top_card(), *card)
                || card.0 == game.top_card().0
            {
                if card.0 == Value::PlusFour || card.0 == Value::Wild {
					// find the color the player has most of

                    action = Action::Card((card.0, most_common_color(player)));
                    break;
                } else {
                    action = Action::Card(*card);
                    break;
                }
            }
        }
        action
    };
    action
}

pub fn random_move<const PLAYERS: usize>(game: &Game<PLAYERS>) -> Action {
    let player = game.current_player();
    let action = if game.player_selected.is_some() {
        let card = game.player_selected.unwrap();
        if card.0 == Value::PlusFour || card.0 == Value::Wild {
            Action::Card((card.0, Color::Red))
        } else {
            Action::Card(card)
        }
    } else {
        let mut action = Action::Pass;
        for card in player {
            if game.stack_value == 0 && can_follow(game.top_card(), *card)
                || card.0 == game.top_card().0
            {
                if card.0 == Value::PlusFour || card.0 == Value::Wild {
                    action = Action::Card((card.0, Color::Red));
                    break;
                } else {
                    action = Action::Card(*card);
                    break;
                }
            }
        }
        action
    };
    action
}
