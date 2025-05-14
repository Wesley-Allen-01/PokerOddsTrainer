use once_cell::sync::Lazy;
use std::collections::HashMap;
use itertools::Itertools;
use crate::card::{ prime_prod_from_rank};




pub fn next_bit_permutation(v: u32) -> Option<u32> {
    let t = (v | (v-1)).wrapping_add(1);

    let nt = (!t).wrapping_add(1);
    let nv = (!v).wrapping_add(1);

    let w = t | ((((t & nt) / (v & nv)) >> 1) - 1);

    return Some(w)
}

struct BitSubset {
    current: Option<u32>,
    limit: u32
}

impl BitSubset {
    pub fn new(n_bits: u8, k: u8) -> Self {
        let start = (1u32 << k) - 1;
        let limit = 1u32 << n_bits;
        Self { current: Some(start), limit}
    }
}

impl Iterator for BitSubset {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.current?;

        if x >= self.limit {
            self.current = None;
            return None;
        }
        else {
            self.current = next_bit_permutation(x);
            return Some(x);
        }
    }
}

fn build_flush_table() -> HashMap<u32, u32> {

    // 1287 = binom(13, 5)
    // Equates to the total number of distinct flushes disregarding suit
    let mut map: HashMap<u32, u32> = HashMap::with_capacity(1287);

    const STRAIGHT_FLUSHES: [u32; 10] = [
        0b1_1111_0000_0000, // A-K-Q-J-T
        0b1111_1000_0000,
        0b111_1100_0000,
        0b11_1110_0000,
        0b1_1111_0000, // T-9-8-7-6
        0b1111_1000,
        0b111_1100,
        0b11_1110,
        0b1_1111, // 6-5-4-3-2
        0b1000_0000_01111 // A-5-4-3-2
    ];

    let mut rank = 1;
    for sf in STRAIGHT_FLUSHES {
        let key = prime_prod_from_rank(sf);
        map.insert(key, rank);
        rank += 1;
    }

    let input = 0b1_1111_0000_0000;
    let prime_input = prime_prod_from_rank(input);
    let val = map.get(&prime_input).copied().expect("Royal flush missing early");
    assert_eq!(val, 1);

    rank = 323;

    let all_patterns: Vec<u32> = BitSubset::new(13, 5).collect();

    for bits in all_patterns.into_iter().rev() {
        if STRAIGHT_FLUSHES.contains(&bits) {
            continue;
        } else {
            let key = prime_prod_from_rank(bits);
            map.insert(key, rank);
            rank += 1;
        }
    }
    
    return map;
}

pub static FLUSH_LOOKUP: Lazy<HashMap<u32, u32>> = Lazy::new(build_flush_table);

fn build_unsuited_table() -> HashMap<u32, u32> {
    let mut map: HashMap<u32, u32> = HashMap::with_capacity(6175);

    const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];


    let mut rank = 11;
    for r in (0..13).rev() {
        for s in (0..13).rev().filter(|&k| k != r) {
            let val = PRIMES[r].pow(4) * PRIMES[s];
            map.insert(val, rank);
            rank += 1;
        }
    }

    for r in (0..13).rev() {
        for s in (0..13).rev().filter(|&k| k != r) {
            let val = PRIMES[r].pow(3) * PRIMES[s].pow(2);
            map.insert(val, rank);
            rank += 1;
        }
    }
    
    rank = 1600;

    const STRAIGHTS: [u32; 10] = [
        0b1_1111_0000_0000, // A-K-Q-J-T
        0b1111_1000_0000,
        0b111_1100_0000,
        0b11_1110_0000,
        0b1_1111_0000, // T-9-8-7-6
        0b1111_1000,
        0b111_1100,
        0b11_1110,
        0b1_1111, // 6-5-4-3-2
        0b1000_0000_01111 // A-5-4-3-2
    ];

    for straight in STRAIGHTS {
        let val = prime_prod_from_rank(straight);
        map.insert(val, rank);
        rank += 1;
    }

    for trips in (0..13).rev() {
        let kickers: Vec<usize> = (0..13).rev().filter(|&k| k != trips).collect();

        for (k1, k2) in kickers.iter().copied().tuple_combinations() {
            let key = PRIMES[trips].pow(3) * PRIMES[k1] * PRIMES[k2];
            map.insert(key, rank);
            rank += 1;
        }
    }

    for (p1, p2) in (0..13).rev().tuple_combinations() {
        let kickers: Vec<usize> = (0..13).rev().filter(|&k| k != p1 && k != p2).collect();
        for kick in kickers {
            let key = PRIMES[p1].pow(2) * PRIMES[p2].pow(2) * PRIMES[kick];
            map.insert(key, rank);
            rank += 1;
        }
    }

    for pair in (0..13).rev() {
        let kickers: Vec<usize> = (0..13).rev().filter(|&k| k != pair).collect();
        for (k1, k2, k3) in kickers.iter().copied().tuple_combinations() {
            let key = PRIMES[pair].pow(2) * PRIMES[k1] * PRIMES[k2] * PRIMES[k3];
            map.insert(key, rank);
            rank += 1;
        }
    }

    let straights = [
        [12, 11, 10, 9, 8],
        [11, 10, 9, 8, 7],
        [10, 9, 8, 7, 6],
        [9, 8, 7, 6, 5],
        [8, 7, 6, 5, 4],
        [7, 6, 5, 4, 3],
        [6, 5, 4, 3, 2],
        [5, 4, 3, 2, 1],
        [4, 3, 2, 1, 0],
        [12, 3, 2, 1, 0]
    ];

    let straight_keys: Vec<u32> = straights.iter().map(|row| {
        row.iter().fold(1u32, |acc, &r| acc * PRIMES[r])
    }).collect();

    let mut i = 1;
    for high in (0..13).rev() {
        let kickers: Vec<usize> = (0..13).rev().filter(|&k| k < high).collect();
        for (k1, k2, k3, k4) in kickers.iter().copied().tuple_combinations() {
            let key = PRIMES[high] * PRIMES[k1] * PRIMES[k2] * PRIMES[k3] * PRIMES[k4];
            if straight_keys.contains(&key) {
                println!("{}", i);
                i += 1;
                continue
            } else {
                map.insert(key, rank);
                rank += 1;
            }
        }
    }

    return map;
}

pub static UNSUITED_LOOKUP: Lazy<HashMap<u32, u32>> = Lazy::new(build_unsuited_table);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{new_card, prime_prod_from_pbits};

    #[test]
    fn check_ryl_flush_rank() {
        let ryl_flush = 0b1_1111_0000_0000;
        let prime_val = prime_prod_from_rank(ryl_flush);
        let val = FLUSH_LOOKUP.get(&prime_val).copied().expect("Royal flush missing");
        let expected: u32 = 1;
        assert_eq!(val, expected);
    }
    #[test]
    fn check_random_flush() {
        let kng_flsh = 0b111101000000;
        let prime_val = prime_prod_from_rank(kng_flsh);
        let val = FLUSH_LOOKUP.get(&prime_val).copied().expect("king flush missing");
        let expected = 816;
        assert_eq!(val, expected);
    }

    #[test]
    fn check_quad_aces() {
        let a = new_card("As").unwrap();
        let b = new_card("Ac").unwrap();
        let c = new_card("Ad").unwrap();
        let d = new_card("Ah").unwrap();
        let e = new_card("8s").unwrap();

        let hand = vec![a, b, c, d, e];

        let key = prime_prod_from_pbits(hand);

        let predicted = UNSUITED_LOOKUP.get(&key).copied().expect("Quad aces missing");
        let expected = 16;
        assert_eq!(predicted, expected);
    }

    #[test]
    fn check_quad_deuces() {
        let a = new_card("2s").unwrap();
        let b = new_card("2c").unwrap();
        let c = new_card("2d").unwrap();
        let d = new_card("2h").unwrap();
        let e = new_card("8s").unwrap();

        let hand = vec![a, b, c, d, e];

        let key = prime_prod_from_pbits(hand);

        let predicted = UNSUITED_LOOKUP.get(&key).copied().expect("Quad deuces missing");
        let expected = 161;
        assert_eq!(predicted, expected);
    }

    #[test]
    fn check_four_full_over_three() {
        let a = new_card("4s").unwrap();
        let b = new_card("4c").unwrap();
        let c = new_card("4d").unwrap();
        let d = new_card("3h").unwrap();
        let e = new_card("3s").unwrap();

        let hand = vec![a, b, c, d, e];

        let key = prime_prod_from_pbits(hand);

        let predicted = UNSUITED_LOOKUP.get(&key).copied().expect("FH missing");
        let expected = 297;
        assert_eq!(predicted, expected);
    }

    #[test]
    fn check_wheel() {
        let a = new_card("As").unwrap();
        let b = new_card("5c").unwrap();
        let c = new_card("4d").unwrap();
        let d = new_card("3h").unwrap();
        let e = new_card("2s").unwrap();

        let hand = vec![a, b, c, d, e];

        let key = prime_prod_from_pbits(hand);

        let predicted = UNSUITED_LOOKUP.get(&key).copied().expect("FH missing");
        let expected = 1609;
        assert_eq!(predicted, expected);
    }

    #[test]
    fn check_set_of_jacks() {
        let a = new_card("Js").unwrap();
        let b = new_card("Jc").unwrap();
        let c = new_card("Jd").unwrap();
        let d = new_card("3h").unwrap();
        let e = new_card("2s").unwrap();

        let hand = vec![a, b, c, d, e];

        let key = prime_prod_from_pbits(hand);

        let predicted = UNSUITED_LOOKUP.get(&key).copied().expect("FH missing");
        let expected = 1873;
        assert_eq!(predicted, expected);
    }

    #[test]
    fn check_five_and_two() {
        let a = new_card("5s").unwrap();
        let b = new_card("5c").unwrap();
        let c = new_card("2d").unwrap();
        let d = new_card("2h").unwrap();
        let e = new_card("As").unwrap();

        let hand = vec![a, b, c, d, e];

        let key = prime_prod_from_pbits(hand);

        let predicted = UNSUITED_LOOKUP.get(&key).copied().expect("FH missing");
        let expected = 3282;
        assert_eq!(predicted, expected);
    }

    #[test]
    fn check_pair() {
        let a = new_card("2s").unwrap();
        let b = new_card("2c").unwrap();
        let c = new_card("5d").unwrap();
        let d = new_card("4h").unwrap();
        let e = new_card("3s").unwrap();

        let hand = vec![a, b, c, d, e];

        let key = prime_prod_from_pbits(hand);

        let predicted = UNSUITED_LOOKUP.get(&key).copied().expect("FH missing");
        let expected = 6185;
        assert_eq!(predicted, expected);
    }

    #[test]
    fn check_worst_hand() {
        let a = new_card("7s").unwrap();
        let b = new_card("5c").unwrap();
        let c = new_card("4d").unwrap();
        let d = new_card("3h").unwrap();
        let e = new_card("2s").unwrap();

        let hand = vec![a, b, c, d, e];

        let key = prime_prod_from_pbits(hand);

        let predicted = UNSUITED_LOOKUP.get(&key).copied().expect("FH missing");
        let expected = 7462;
        assert_eq!(predicted, expected);
    }
}


