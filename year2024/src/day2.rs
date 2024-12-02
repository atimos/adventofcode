use itertools::Itertools;
use rayon::prelude::*;
use std::fs::read_to_string;

fn main() {
    dbg!(part1());
    dbg!(part2());
}

fn part1() -> usize {
    let data = data();

    data.iter()
        .filter(|line| {
            let mut ord = None;
            let result = line.iter().tuple_windows().all(|(a, b)| {
                if let Some(ord) = ord {
                    a.cmp(b) == ord && (1..=3).contains(&a.abs_diff(*b))
                } else {
                    ord = Some(a.cmp(b));
                    (1..=3).contains(&a.abs_diff(*b))
                }
            });
            result
        })
        .count()
}

fn part2() -> usize {
    let data = data();

    data.into_iter()
        .filter(|line| {
            let mut ord = None;
            let result = line.iter().tuple_windows().all(|(a, b)| {
                if let some(ord) = ord {
                    a.cmp(b) == ord && (1..=3).contains(&a.abs_diff(*b))
                } else {
                    ord = some(a.cmp(b));
                    (1..=3).contains(&a.abs_diff(*b))
                }
            });

            if result {
                return result;
            }

            (0..line.len()).into_par_iter().any(|idx| {
                let mut ord = None;
                let mut line = line.clone();
                line.remove(idx);

                line.iter().tuple_windows().all(|(a, b)| {
                    if let Some(ord) = ord {
                        a.cmp(b) == ord && (1..=3).contains(&a.abs_diff(*b))
                    } else {
                        ord = Some(a.cmp(b));
                        (1..=3).contains(&a.abs_diff(*b))
                    }
                })
            })
        })
        .count()
}

fn data() -> Vec<Vec<usize>> {
    let input = read_to_string("data/2024-02-live.txt").unwrap();
    input
        .lines()
        .map(|line| line.split_whitespace().map(|entry| entry.parse::<usize>().unwrap()).collect())
        .collect::<Vec<_>>()
}
