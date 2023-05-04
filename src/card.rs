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
    return old.0 == new.0 || old.1 == new.1 || new.0 == Value::Wild || new.0 == Value::DrawFour;
}
