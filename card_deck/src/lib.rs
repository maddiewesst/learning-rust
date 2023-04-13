use rand::Rng;

#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        // let x = rng.gen();
        // let rand_suit = rng.gen::<>();
        match rng.gen_range(1, 4) {
            1 => return Suit::Heart,
            2 => return Suit::Diamond,
            3 => return Suit::Spade,
            4 => return Suit::Club,
            i32::MIN..=0_i32 | 5_i32..=i32::MAX => todo!(),
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            0_u8 | 5_u8..=u8::MAX => todo!(),
            
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        match rng.gen_range(1, 5) {
            1 => return Rank::Ace,
            2 => return Rank::King,
            3 => return Rank::Queen,
            4 => return Rank::Jack,
            5 => return Rank::Number(rng.gen_range(2, 10)),
            i32::MIN..=0_i32 | 6_i32..=i32::MAX => todo!(),
        }
    }
}

pub fn translate(value: u8) -> Rank {
    match value {
        1 => return Rank::Ace,
        13 => return Rank::King,
        12 => return Rank::Queen,
        11 => return Rank::Jack,
        other => return Rank::Number(other),
    }
}
#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    match &card.suit {
        Suit::Spade => match card.rank {
            Rank::Ace => return true,
            _ => false,
        },
        _other => return false,
    }
}
