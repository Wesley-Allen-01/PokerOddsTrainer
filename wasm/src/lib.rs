use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[wasm_bindgen]
pub fn draw_random_hand() -> Vec<String> {
    let suits = ["s", "c", "h", "d"];
    let ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];

    // creates vector containing each card in deck by appending each rank to each suit
    let mut deck: Vec<String> = suits
        .iter()
        .flat_map(|&suit| {
            ranks.iter().map(move |&rank| format!("{}{}", rank, suit))
        })
        .collect();

    deck.shuffle(&mut thread_rng());

    vec![deck[0].clone(), deck[1].clone()]
}

