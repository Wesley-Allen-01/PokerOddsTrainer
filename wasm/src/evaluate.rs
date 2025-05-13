// use wasm_bindgen::prelude::*;
use crate::card::{Card};

// #[wasm_bindgen]
// pub fn evaluate_hand(cards: &[Card]) -> u32 {

// }

pub fn check_flush(cards: &[Card]) -> bool {
    let card_holder = cards.iter().copied().fold(0xF000, |acc, x| acc & x);

    if card_holder & 0xF000 != 0 {
        return true
    }
    else {
        return false
    }
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
}