use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::card::prime_prod_from_rank;


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

fn build_flush_table() -> HashMap<u32, u16> {

    // 1287 = binom(13, 5)
    // Equates to the total number of distinct flushes disregarding suit
    let mut map: HashMap<u32, u16> = HashMap::with_capacity(1287);

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

pub static FLUSH_LOOKUP: Lazy<HashMap<u32, u16>> = Lazy::new(build_flush_table);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ryl_flush_rank() {
        let ryl_flush = 0b1_1111_0000_0000;
        let prime_val = prime_prod_from_rank(ryl_flush);
        let val = FLUSH_LOOKUP.get(&prime_val).copied().expect("Royal flush missing");
        let expected: u16 = 1;
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
}


