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

// Slightly smarter implementation that takes in context and valuable cards.
pub fn smart_move<const PLAYERS: usize>(game: &Game<PLAYERS>) -> Action {
    let player = game.current_player();
    let action = if game.player_selected.is_some() {
        let card = game.player_selected.unwrap();
        if card.0 == Value::DrawFour || card.0 == Value::Wild {
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
        let mut possible_actions = vec![Action::Pass];
        for card in player {
            if game.stack_value == 0 && can_follow(game.top_card(), *card)
                || card.0 == game.top_card().0
            {
                if card.0 == Value::DrawFour || card.0 == Value::Wild {                    
                    possible_actions.push(Action::Card((card.0, most_common_color(player))));
                } else {
                    possible_actions.push(Action::Card(*card));
                }
            }
        }
        let next_player_card_count = game.card_count(game.player + 1 % PLAYERS);
        let action = *possible_actions.iter().min_by_key(|action| match (action, next_player_card_count) {
            (Action::Pass, _) => 10,
            // avoid using DrawFour if the next player already has a lot of cards
            // (Action::Card((Value::DrawFour, _)), n) if n > 20 => 5,
            (Action::Card((Value::DrawFour, _)), 1) => 1,
            (Action::Card((Value::DrawTwo, _)), 1) => 1,
            (Action::Card((Value::Wild, _)), _) => 4,
            (Action::Card(_), _) => 3,
        }).unwrap();
        action  
    };
    action
}


// Picks the first valid move
// Technically spamming pass is also a valid move
// But that ends up panicking because the game runs out of cards to draw
pub fn random_move<const PLAYERS: usize>(game: &Game<PLAYERS>) -> Action {
    let player = game.current_player();
    let action = if game.player_selected.is_some() {
        let card = game.player_selected.unwrap();
        if card.0 == Value::DrawFour || card.0 == Value::Wild {
            Action::Card((card.0, most_common_color()))
        } else {
            Action::Card(card)
        }
    } else {
        let mut action = Action::Pass;
        for card in player {
            if game.stack_value == 0 && can_follow(game.top_card(), *card)
                || card.0 == game.top_card().0
            {
                if card.0 == Value::DrawFour || card.0 == Value::Wild {
                    action = Action::Card((card.0, most_common_color()));
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
