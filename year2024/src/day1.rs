use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    dbg!(part1());
    dbg!(part2());
}

fn part1() -> usize {
    let (list1, list2) = data();

    list1.iter().zip(list2).map(|(first, second)| first.abs_diff(second)).sum()
}

fn part2() -> usize {
    let (list1, list2) = data();

    let map2 = list2.into_iter().chunk_by(|i| *i).into_iter().map(|(k, v)| (k, v.count())).collect::<HashMap<_, _>>();

    list1.iter().map(|i| i * map2.get(i).copied().unwrap_or_default()).sum()
}

fn data() -> (Vec<usize>, Vec<usize>) {
    let input = read_to_string("data/2024-01-test.txt").unwrap();
    let (mut list1, mut list2) = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            (split.next().unwrap().parse::<usize>().unwrap(), split.next().unwrap().parse::<usize>().unwrap())
        })
        .collect::<(Vec<_>, Vec<_>)>();

    list1.sort();
    list2.sort();

    (list1, list2)
}
