#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Block,
    Reverse,
    DrawTwo,
    DrawFour,
    Wild,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    None,
}

impl Color {
	pub fn from_num(num: usize) -> Color {
		match num {
			0 => Color::Red,
			1 => Color::Blue,
			2 => Color::Green,
			3 => Color::Yellow,
			_ => Color::None,
		}
	}
	pub fn num(self) -> usize {
		match self {
			Color::Red => 0,
			Color::Blue => 1,
			Color::Green => 2,
			Color::Yellow => 3,
			Color::None => 4,
		}	
	}
}

pub type Card = (Value, Color);

pub fn can_follow(old: Card, new: Card) -> bool {
    match (old, new) {
        ((Value::Zero, _), (Value::Zero, _)) => true,
        ((Value::One, _), (Value::One, _)) => true,
        ((Value::Two, _), (Value::Two, _)) => true,
        ((Value::Three, _), (Value::Three, _)) => true,
        ((Value::Four, _), (Value::Four, _)) => true,
        ((Value::Five, _), (Value::Five, _)) => true,
        ((Value::Six, _), (Value::Six, _)) => true,
        ((Value::Seven, _), (Value::Seven, _)) => true,
        ((Value::Eight, _), (Value::Eight, _)) => true,
        ((Value::Nine, _), (Value::Nine, _)) => true,
        ((Value::Block, _), (Value::Block, _)) => true,
        ((Value::Reverse, _), (Value::Reverse, _)) => true,
        ((Value::DrawTwo, _), (Value::DrawTwo, _)) => true,
        ((_, Color::Red), (_, Color::Red)) => true,
        ((_, Color::Blue), (_, Color::Blue)) => true,
        ((_, Color::Green), (_, Color::Green)) => true,
        ((_, Color::Yellow), (_, Color::Yellow)) => true,
        (_, (Value::DrawFour, _)) => true,
        (_, (Value::Wild, _)) => true,
        _ => false,
    }
}
