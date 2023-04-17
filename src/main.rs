use rand::Rng;
use std::collections::HashSet;
use std::io;

const DICTIONARY: &str = include_str!("../dictionary.text");

fn main() {
    let answer = "guess";
    let guess = "gusse";
    let guess_result = wordle::check_guess_vec(answer, guess);
    let hash_set = HashSet::<&'static str>::from_iter(
        DICTIONARY
            .lines(),
    );

    let mut game = wordle::Game::new(hash_set);
    game.start();
}
