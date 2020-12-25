use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::collections::HashSet;

const DAY: &str = "day22";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut player_1: Vec<usize> = Vec::new();
    let mut player_2: Vec<usize> = Vec::new();

    let mut input = input.into_iter().skip(1);

    for line in &mut input {
        if line == "" {
            break;
        }
        player_1.push(line.parse()?)
    }

    for line in input.skip(1) {
        if line == "" {
            break;
        }
        player_2.push(line.parse()?)
    }

    let score = normal_game(player_1.clone(), player_2.clone())?;

    let winner = recursive_game(player_1, player_2).1;

    Ok([Some(score), Some(compute_score(&winner).to_string())])
}

fn normal_game(mut player_1: Vec<usize>, mut player_2: Vec<usize>) -> Result<String, String> {
    while player_1.len() != 0 && player_2.len() != 0 {
        let card_1 = player_1.remove(0);
        let card_2 = player_2.remove(0);

        println!("{} vs {}...", card_1, card_2);

        if card_1 > card_2 {
            player_1.push(card_1);
            player_1.push(card_2);
        } else if card_2 > card_1 {
            player_2.push(card_2);
            player_2.push(card_1);
        } else {
            return Err(format!("Cards should all be unique but got double {} card", card_1).into());
        }
    }

    let winner = if player_2.len() > 0 {
        player_2
    } else {
        player_1
    };

    println!("Winner deck: {:?}", &winner);
    Ok(compute_score(&winner).to_string())
}

fn recursive_game(mut player_1: Vec<usize>, mut player_2: Vec<usize>) -> (bool, Vec<usize>) {
    let mut visited: HashSet<[Vec<usize>; 2]> = HashSet::new();

    while player_1.len() != 0 && player_2.len() != 0 {
        let card_1 = player_1.remove(0);
        let card_2 = player_2.remove(0);

        // println!("{} vs {}...", card_1, card_2);

        let player_1_wins = if player_1.len() >= card_1 && player_2.len() >= card_2 {
            // println!("Playing subgame");
            recursive_game(Vec::from(&player_1[0..card_1]), Vec::from(&player_2[0..card_2])).0
        } else {
            card_1 > card_2
        };

        if player_1_wins {
            player_1.push(card_1);
            player_1.push(card_2);
        } else {
            player_2.push(card_2);
            player_2.push(card_1);
        }

        let key = decks_as_key(&player_1, &player_2);
        if visited.contains(&key) {
            break;
        } else {
            visited.insert(key);
        }
    }

    // println!("Finishing game");
    if player_1.len() != 0 {
        (true, player_1)
    } else {
        (false, player_2)
    }
}

fn decks_as_key(player_1: & Vec<usize>, player_2: & Vec<usize>) -> [Vec<usize>; 2] {
    [player_1.to_owned(), player_2.to_owned()]
}

fn compute_score(winner :&Vec<usize>) -> usize {
    let len = winner.len();
    winner.iter().zip(0..).map(|(c, i)| (*c) * (len-i)).sum()
}

#[test]
pub fn test_day22() {
    assert!(common::run_test(DAY, &run))
}