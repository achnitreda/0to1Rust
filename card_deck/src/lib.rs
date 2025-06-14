use rand::Rng;
use rand::thread_rng;

#[derive(Debug,Clone, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug,Clone, PartialEq)]
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
        let variants = [Suit::Heart, Suit::Diamond, Suit::Spade, Suit::Club];
        let index = rng.gen_range(0..variants.len());
        variants[index].clone()
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            4 => Suit::Club,
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => todo!(),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1..=13);
       match n {
            0 => Rank::Ace,
            1 => Rank::King,
            2 => Rank::Queen,
            3 => Rank::Jack,
            n => Rank::Number((n - 3) as u8), 
            _ => todo!(),
       }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            13 => Rank::King,
            12 => Rank::Queen,
            11 => Rank::Jack,
            n => Rank::Number(n), 
       }
    }
}

#[derive(Debug,PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.rank == Rank::Ace && card.suit == Suit::Spade
}