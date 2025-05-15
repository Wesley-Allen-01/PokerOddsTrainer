use itertools::Itertools;
use wasm_bindgen::prelude::*;
use crate::card::{prime_prod_from_pbits, prime_prod_from_rank, Card};
use crate::lookup::{FLUSH_LOOKUP, UNSUITED_LOOKUP};

#[wasm_bindgen]
pub fn evaluate_hand(cards: Vec<u32>) -> u32 {
    if check_flush(&cards) {
        let hand = cards.iter().fold(0u32, |acc, &x| acc | x) >> 16;
        let prime_val = prime_prod_from_rank(hand);
        FLUSH_LOOKUP[&prime_val]
    } else {
        let key: u32 = prime_prod_from_pbits(cards);
        UNSUITED_LOOKUP[&key]
    }
}

pub fn check_flush(cards: &[Card]) -> bool {
    let card_holder = cards.iter().copied().fold(0xF000, |acc, x| acc & x);

    if card_holder & 0xF000 != 0 {
        return true
    }
    else {
        return false
    }
}

pub fn evaluate_board(cards: Vec<u32>) -> u32 {
    cards.iter()
        .copied()
        .combinations(5)
        .map(|combo| evaluate_hand(combo))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{new_card};

    #[test]
    fn check_real_flush() {
        let a = new_card("As").unwrap();
        let b = new_card("Ks").unwrap();
        let c = new_card("Qs").unwrap();
        let d = new_card("Js").unwrap();
        let e = new_card("Ts").unwrap();

        let card_vector: &[Card] = &[a, b, c, d, e];
        let ret = check_flush(&card_vector);
        assert!(ret)
    }

    #[test]
    fn check_no_flush() {
        let a = new_card("As").unwrap();
        let b = new_card("Ks").unwrap();
        let c = new_card("Qs").unwrap();
        let d = new_card("Js").unwrap();
        let e = new_card("Tc").unwrap();

        let card_vector: &[Card] = &[a, b, c, d, e];
        let ret = check_flush(&card_vector);
        assert!(!ret)
    }

    #[test]
    fn check_flush_with_eval() {
        let a = new_card("As").unwrap();
        let b = new_card("Ks").unwrap();
        let c = new_card("Qs").unwrap();
        let d = new_card("Js").unwrap();
        let e = new_card("8s").unwrap();

        let card_vector: Vec<Card> = vec![a, b, c, d, e];
        let val = evaluate_hand(card_vector);
        assert_eq!(val, 324)
    }

    #[test]
    fn check_random_hand_with_eval() {
        let a = new_card("As").unwrap();
        let b = new_card("Ac").unwrap();
        let c = new_card("Ah").unwrap();
        let d = new_card("Js").unwrap();
        let e = new_card("Jd").unwrap();

        let card_vector: Vec<Card> = vec![a, b, c, d, e];
        let val = evaluate_hand(card_vector);
        assert_eq!(val, 169)
    }
}