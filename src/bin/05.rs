use itertools::Itertools;
use std::collections::VecDeque;
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<String> {
    let mut cranes = starting_crates(input)?;
    for (count, from, to) in instructions(input) {
        for _ in 0..count {
            let from = cranes.get_mut(from - 1)?;
            let elem = from.pop_back()?;
            let to = cranes.get_mut(to - 1)?;
            to.push_back(elem);
        }
    }
    Some(
        cranes
            .into_iter()
            .filter_map(|mut v| v.pop_back())
            .collect(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let mut crates = starting_crates(input)?;
    for (count, from, to) in instructions(input) {
        let from = crates.get_mut(from - 1)?;
        let elems: Vec<_> = from.drain((from.len() - count)..).collect();
        let to = crates.get_mut(to - 1)?;
        to.extend(elems);
    }
    Some(
        crates
            .into_iter()
            .filter_map(|mut v| v.pop_back())
            .collect(),
    )
}

fn instructions(input: &str) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
    input
        .lines()
        .skip_while(|l| !l.starts_with("move"))
        .filter_map(|l| {
            let mut i = l.split_whitespace().skip(1).step_by(2).map(usize::from_str);
            Some((i.next()?.ok()?, i.next()?.ok()?, i.next()?.ok()?))
        })
}

fn starting_crates(input: &str) -> Option<Vec<VecDeque<char>>> {
    let mut result = vec![];
    for l in input.lines() {
        if l.is_empty() {
            break;
        }
        for (i, c) in l
            .chars()
            .chunks(4)
            .into_iter()
            .filter_map(|mut i| i.nth(1))
            .enumerate()
        {
            // Only runs in the first line iteration
            if i == result.len() {
                result.push(VecDeque::new());
            }
            if c.is_ascii_alphabetic() {
                let e: &mut VecDeque<char> = result.get_mut(i)?;
                e.push_front(c);
            }
        }
    }
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
