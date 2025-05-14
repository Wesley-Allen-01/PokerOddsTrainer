use wasm_bindgen::prelude::*;

pub type Card = u32;

fn char_to_rank(c: char) -> Option<u32> {
    match c {
        '2' => Some(0),
        '3' => Some(1),
        '4' => Some(2),
        '5' => Some(3),
        '6' => Some(4),
        '7' => Some(5),
        '8' => Some(6),
        '9' => Some(7),
        'T' => Some(8),
        'J' => Some(9),
        'Q' => Some(10),
        'K' => Some(11),
        'A' => Some(12),
        _ => None,
    }
}

fn char_to_bit(c: char) -> Option<u32> {
    let base: u32 = 2;
    match c {
        '2' => Some(base.pow(0)),
        '3' => Some(base.pow(1)),
        '4' => Some(base.pow(2)),
        '5' => Some(base.pow(3)),
        '6' => Some(base.pow(4)),
        '7' => Some(base.pow(5)),
        '8' => Some(base.pow(6)),
        '9' => Some(base.pow(7)),
        'T' => Some(base.pow(8)),
        'J' => Some(base.pow(9)),
        'Q' => Some(base.pow(10)),
        'K' => Some(base.pow(11)),
        'A' => Some(base.pow(12)),
        _ => None,
    }
}

fn rank_to_prime(c: char) -> Option<u32> {
    match c {
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(5),
        '5' => Some(7),
        '6' => Some(11),
        '7' => Some(13),
        '8' => Some(17),
        '9' => Some(19),
        'T' => Some(23),
        'J' => Some(29),
        'Q' => Some(31),
        'K' => Some(37),
        'A' => Some(41),
        _ => None,
    }
}

fn suit_to_bit(s: char) -> Option<u32> {
    match s {
        's' => Some(1),
        'h' => Some(2),
        'd' => Some(4),
        'c' => Some(8),
        _ => None,
    }
}

pub fn prime_prod_from_rank(cards: u32) -> u32 {
    let primes: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
    let mut prod: u32 = 1;

    for i in 0..13 {
        if cards & (1 << i) != 0 {
            prod *= primes[i];
        }
    }
    prod
}

pub fn prime_prod_from_pbits(cards: Vec<Card>) -> u32 {
    cards.iter().fold(1, |acc, &x| acc * (x & 0x3F))
}

#[wasm_bindgen]
pub fn new_card(card_str: &str) -> Option<Card> {
    if card_str.len() != 2 {
        return None;
    }

    // "As"
    let mut chars = card_str.chars();
    let rank_char = chars.next()?;
    let suit_char = chars.next()?;

    let rank_b = char_to_bit(rank_char)? << 16;
    let suit = suit_to_bit(suit_char)? << 12;
    let rank_n = char_to_rank(rank_char)? << 8;
    let rank_p = rank_to_prime(rank_char)?;


    let new_card: Card = rank_b | suit | rank_n | rank_p;

    Some(new_card)

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ace_of_king_diamonds() {
        let card = new_card("Kd").unwrap();

        assert_eq!(card, 134236965);
    }

    #[test]
    fn test_five_of_spades() {
        let card: u32 = new_card("5s").unwrap();

        assert_eq!(card, 529159)
    }

    #[test]
    fn test_prime_from_rank_bits() {
        let a = new_card("As").unwrap();
        let b = new_card("Ks").unwrap();
        let c = new_card("Qs").unwrap();
        let d = new_card("Js").unwrap();
        let e = new_card("Ts").unwrap();

        let cards = &[a, b, c, d, e];

        let combined_cards = cards.iter().copied().fold(0b0, |acc, x| acc | x);
        
        let clean_cards = combined_cards >> 16;

        let predicted = prime_prod_from_rank(clean_cards);

        let expected = 41 * 37 * 31 * 29 * 23;

        assert_eq!(predicted, expected)
    }
}

