use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
pub mod card;
pub mod evaluate;
pub mod lookup;

#[wasm_bindgen]
pub fn draw(n: usize) -> Vec<String> {
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

    deck.into_iter().take(n).collect()
}

#[wasm_bindgen]
pub fn draw_starting_hand() -> Vec<String> {
    draw(2)
} 

#[wasm_bindgen]
pub fn draw_flop() -> Vec<String> {
    draw(3)
}

#[wasm_bindgen] 
pub fn draw_card() -> Vec<String> {
    draw(1)
}
