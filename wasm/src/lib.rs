use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::card::{Card, new_card};
use crate::evaluate::{evaluate_board};
pub mod card;
pub mod evaluate;
pub mod lookup;

fn new_deck() -> Vec<Card> {
    let suits = ['s', 'c', 'h', 'd'];
    let ranks = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

    let mut deck = Vec::with_capacity(52);

    for &s in &suits {
        for &r in &ranks {
            let card = new_card(&format!("{r}{s}")).unwrap();
            deck.push(card);
        }
    }
    deck
}

#[wasm_bindgen]
pub struct Deck {
    cards: Vec<Card>,
}

#[wasm_bindgen]
impl Deck {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Deck {
        let mut cards = new_deck();
        cards.shuffle(&mut thread_rng());
        Deck { cards }
    }

    pub fn new_from_cards(cards: Vec<Card>) -> Deck {
        let mut deck = new_deck();
        for &card in &cards {
            // remove card from deck
            if let Some(idx) = deck.iter().position(|&c| c == card) {
                deck.remove(idx);
            }
        }
        // add cards back to deck to be in front
        deck.extend(cards.iter().copied());
        Deck { cards: deck }
    }

    pub fn draw(&mut self, n: usize) -> Vec<Card> {
        let take = n.min(self.cards.len());
        self.cards.split_off(self.cards.len() - take)
    }

    pub fn remove(&mut self, card: Card) -> bool {
        if let Some(idx) = self.cards.iter().position(|&c| c == card) {
            self.cards.remove(idx);
            true
        }
        else {
            false
        }
    }
}

pub fn simulate_hand(starting_hand: Vec<Card>) -> i32 {
    let mut deck = Deck::new();
    simulate_hand_with_deck(starting_hand, &mut deck)
}

pub fn simulate_hand_with_deck(starting_hand: Vec<Card>, deck: &mut Deck) -> i32 {
    for &card in &starting_hand {
        deck.remove(card);
    }

    let opponent_start = deck.draw(2);
    let flop = deck.draw(3);
    let turn = deck.draw(1);
    let river = deck.draw(1);

    let mut hero_board = starting_hand.clone();
    hero_board.extend(flop.iter().copied());
    hero_board.extend(turn.iter().copied());
    hero_board.extend(river.iter().copied());

    let mut villain_board = opponent_start.clone();
    villain_board.extend(flop.iter().copied());
    villain_board.extend(turn.iter().copied());
    villain_board.extend(river.iter().copied());

    let hero_score = evaluate_board(hero_board);
    let villain_score = evaluate_board(villain_board);

    if hero_score > villain_score {
        1
    } else if villain_score > hero_score {
        -1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{new_card};
    use crate::{Deck, simulate_hand_with_deck};
    fn rigged_deck() -> Deck {
        let order = vec![
            new_card("As").unwrap(), new_card("Ah").unwrap(), // hero
            new_card("7c").unwrap(), new_card("7d").unwrap(), // villain
            new_card("2s").unwrap(), new_card("3s").unwrap(), new_card("4s").unwrap(), // flop
            new_card("9s").unwrap(), // turn
            new_card("6s").unwrap(), // river
            // remaining 45 cards don't matter for this test
        ];

        Deck {cards: order}
    }

    #[test]
    fn check_aces_win() {
        let mut deck = rigged_deck();
        let starting_hand = vec![new_card("As").unwrap(), new_card("Ah").unwrap()];
        let result = simulate_hand_with_deck(starting_hand, &mut deck);
        assert_eq!(result, 1);
    }
}