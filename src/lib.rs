use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::{Read, Write};
use std::ops;

mod algorithms;

pub struct Game {
    dictionary: HashSet<&'static str>,
    num_attempts: usize,
}

impl Game {
    pub fn new(dictionary: HashSet<&'static str>) -> Self {
        Game {
            dictionary: dictionary,
            num_attempts: 0,
        }
    }

    pub fn start(&mut self) {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.dictionary.len());
        let answer = self.dictionary.iter().skip(index).next().unwrap();
        // println!("the answer is {}", answer);
        println!("Please enter a 5 letter word");
        let stdin = io::stdin();
        let mut guess = String::new();
        loop {
            println!("Attempt-{}/5:", self.num_attempts + 1);

            guess.clear();
            stdin.read_line(&mut guess);
            println!("what is the guess: {}", guess.trim());
            if guess.trim().len() != 5 {
                println!("invalid word");
                continue;
            }

            let game_result = check_guess_vec(answer, guess.trim());
            if game_result.is_success() {
                println!("You have correctly guessed: {answer}");
                break;
            } else {
                println!("{}", game_result);
                println!(
                    "{}        {}        {}        {}        {}",
                    guess.chars().nth(0).unwrap(),
                    guess.chars().nth(1).unwrap(),
                    guess.chars().nth(2).unwrap(),
                    guess.chars().nth(3).unwrap(),
                    guess.chars().nth(4).unwrap()
                );
                self.num_attempts += 1;
                if self.num_attempts == 5 {
                    println!("The answer is: {}. Try again next time!", answer);
                    break;
                }
            }
            println!("\n");
            io::stdout().flush();
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub enum LetterState {
    #[default]
    Wrong,
    Correct,
    Misplaced,
}

#[derive(Default, Debug, PartialEq)]
pub struct GuessResult {
    result: [LetterState; 5],
}

impl fmt::Display for GuessResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:?} {:?} {:?} {:?} {:?}]",
            self.result[0], self.result[1], self.result[2], self.result[3], self.result[4]
        )
    }
}

impl GuessResult {
    pub fn is_success(&self) -> bool {
        self.result
            .iter()
            .all(|letter_state| *letter_state == LetterState::Correct)
    }
}

impl PartialEq<[LetterState; 5]> for GuessResult {
    fn eq(&self, other: &[LetterState; 5]) -> bool {
        other
            .iter()
            .enumerate()
            .all(|(i, letter_state)| self[i] == *letter_state)
    }
}

impl PartialEq<GuessResult> for [LetterState; 5] {
    fn eq(&self, other: &GuessResult) -> bool {
        self.iter()
            .enumerate()
            .all(|(i, letter_state)| other[i] == *letter_state)
    }
}

impl ops::Index<usize> for GuessResult {
    type Output = LetterState;

    fn index(&self, index: usize) -> &Self::Output {
        &self.result[index]
    }
}

impl ops::IndexMut<usize> for GuessResult {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.result[index]
    }
}

#[allow(unused)]
/// Algo explanation.
/// Marked guess_result to be [Wrong, Wrong, Wrong, Wrong, Wrong]
/// Iterate through both ans and guess to see if any character is happens to be the same and at the same position.
/// If yes, marked the corresponding position in guess_result to be Correct.
/// If not, try to search for the slot that is not marked in guess. If there is exist a same character, but in different position, mark that as Misplaced.
pub fn check_guess_vec(answer: &str, guess: &str) -> GuessResult {
    assert!(answer.len() == 5);
    assert!(guess.len() == 5);


    // crate::check_guess_vec("abhde", "bbcdf"),
    // [Wrong, Correct, Wrong, Correct, Wrong]
    let mut guess_result = GuessResult::default();
    answer
        .chars()
        .zip(guess.chars())
        .enumerate()
        .for_each(|(i, (a, g))| {
            // println!("hello world: answer:{}-guess:{}", a, g);
            if a == g {
                guess_result[i] = LetterState::Correct;
            } else {
                if let Some(position) = guess
                    .chars()
                    .enumerate()
                    .position(|(i, c)| {
                        // println!("the c is {}     the a is {}", c, a);
                        guess_result[i] == LetterState::Wrong && c == a
                    })
                {
                    guess_result[position] = LetterState::Misplaced;
                }
            }
            println!("the result state: {:?}", guess_result);
        });

    guess_result
}

#[allow(unused)]
fn check_guess_hash(answer: &str, guess: &str) -> GuessResult {
    assert!(answer.len() == 5);
    assert!(guess.len() == 5);

    let mut answer_dict = answer.chars().into_iter().enumerate().fold(
        HashMap::<char, Vec<usize>>::with_capacity(5),
        |mut map, (i, e)| {
            map.entry(e)
                .and_modify(|vec| vec.push(i))
                .or_insert(vec![i]);
            map
        },
    );

    guess
        .chars()
        .into_iter()
        .enumerate()
        .fold(GuessResult::default(), |mut game_result, (i, c)| {
            if let Some(answer_letter_indices) = answer_dict.get_mut(&c) {
                if let Some(position) = answer_letter_indices.iter().position(|x| *x == i) {
                    game_result[i] = LetterState::Correct;
                    answer_letter_indices.remove(position);
                } else {
                    game_result[i] = LetterState::Misplaced;
                }
            } else {
                game_result[i] = LetterState::Wrong;
            }

            game_result
        })
}

#[cfg(test)]
mod tests {
    use crate::LetterState::*;
    #[test]
    fn test_all_correct() {
        assert_eq!(
            crate::check_guess_vec("guess", "guess"),
            [Correct, Correct, Correct, Correct, Correct]
        );
    }

    #[test]
    fn test_all_wrong() {
        assert_eq!(
            crate::check_guess_vec("gamer", "books"),
            [Wrong, Wrong, Wrong, Wrong, Wrong]
        );
    }

    #[test]
    fn test_all_misplaced() {
        assert_eq!(
            crate::check_guess_vec("books", "oncek"),
            [Misplaced, Wrong, Wrong, Wrong, Misplaced]
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(
            crate::check_guess_vec("azzaz", "aaabb"),
            [Correct, Misplaced, Wrong, Wrong, Wrong]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            crate::check_guess_vec("baccc", "aaddd"),
            [Wrong, Correct, Wrong, Wrong, Wrong]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            crate::check_guess_vec("abcde", "aacde"),
            [Correct, Wrong, Correct, Correct, Correct]
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            crate::check_guess_vec("acaca", "hhhch"),
            [Wrong, Wrong, Wrong, Correct, Wrong]
        );
    }
}
