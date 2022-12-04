use std::cmp::{max, min};
use std::str::FromStr;

fn parse(input: &str) -> impl Iterator<Item = (u32, u32, u32, u32)> + '_ {
    input.lines().filter_map(|s| {
        let (pair1, pair2) = s.split_once(',')?;
        let mut pair1 = pair1.split('-').map(u32::from_str);
        let mut pair2 = pair2.split('-').map(u32::from_str);
        Some((
            pair1.next()?.ok()?,
            pair1.next()?.ok()?,
            pair2.next()?.ok()?,
            pair2.next()?.ok()?,
        ))
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .filter(|(spair1, epair1, spair2, epair2)| {
                (max(spair1, spair2) + min(epair1 - spair1, epair2 - spair2))
                    <= min(*epair1, *epair2)
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .filter(|(spair1, epair1, spair2, epair2)| !(spair1 > epair2 || spair2 > epair1))
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
