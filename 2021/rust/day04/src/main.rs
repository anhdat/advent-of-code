use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
    vec,
};

use libs::*;

type Ticket = Vec<Vec<i32>>;
#[derive(Debug)]
struct BingoGame {
    draws: Vec<i32>,
    tickets: Vec<Ticket>,
}

fn parse_game(paragraphs: &Vec<String>) -> BingoGame {
    let draws: Vec<i32> = paragraphs
        .first()
        .unwrap()
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let tickets: Vec<Ticket> = paragraphs
        .iter()
        .skip(1)
        .map(|p| {
            p.trim()
                .split("\n")
                .map(|l| {
                    l.split_ascii_whitespace()
                        .map(|c| c.parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    BingoGame { draws, tickets }
}

fn play(game: BingoGame) -> i32 {
    let mut all_sets: Vec<(HashSet<i32>, usize)> = vec![];

    game.tickets
        .iter()
        .enumerate()
        .for_each(|(player_index, ticket)| {
            all_sets.extend(
                ticket
                    .iter()
                    .map(|row| (HashSet::from_iter(row.iter().cloned()), player_index))
                    .collect::<Vec<(HashSet<i32>, usize)>>(),
            );
            all_sets.extend(
                (0..game.tickets.first().unwrap().len())
                    .map(|col| {
                        (
                            ticket.iter().map(|row| row[col]).collect::<HashSet<i32>>(),
                            player_index,
                        )
                    })
                    .collect::<Vec<(HashSet<i32>, usize)>>(),
            );
        });

    let mut current_draws: HashSet<i32> = HashSet::new();
    let mut draws: VecDeque<i32> = game.draws.clone().iter().copied().collect();
    while draws.len() > 1 {
        let last_draw = draws.pop_front().unwrap();
        current_draws.insert(last_draw);
        for (s, i) in all_sets.iter() {
            if s.intersection(&current_draws).count() == 5 {
                let all_nums: HashSet<i32> = HashSet::from_iter(
                    game.tickets[*i]
                        .iter()
                        .flatten()
                        .copied()
                        .collect::<Vec<i32>>(),
                );

                let remainings = all_nums.difference(&current_draws);

                return remainings
                    .copied()
                    .collect::<Vec<i32>>()
                    .into_iter()
                    .sum::<i32>()
                    * last_draw;
            }
        }
    }

    0
}

fn main() {
    // let lines = data_strings("04");
    // let r1 = part_1(&lines);
    // println!("{:?}", r1);
    // let r2 = part_2(&lines);
    // println!("{:?}", r2);
}

#[test]
fn test_part_1() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7"
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();
    let game = parse_game(&input);
    // println!("{:#?}", game);
    let result = play(game);
    println!("{}", result);
    assert_eq!(result, 4512);
    // let lines: Vec<String> = data_paragraphs("04");
    // let game = parse_game(&lines);
    // let r1 = part_1(&lines);
    // assert_eq!(r1, 4001724)
}
