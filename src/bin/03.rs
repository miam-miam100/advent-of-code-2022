use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let length = line.len() / 2;
                let (first, second) = line.split_at(length);
                first.chars().find(|c| second.contains(*c)).map(priority)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .chunks(3)
            .into_iter()
            .filter_map(|mut it| {
                let first = it.next()?;
                let second = it.next()?;
                it.next()?
                    .chars()
                    .find(|c| first.contains(*c) && second.contains(*c))
                    .map(priority)
            })
            .sum(),
    )
}

fn priority(input: char) -> u32 {
    (match input {
        'a'..='z' => input as u8 - b'a' + 1,
        'A'..='Z' => input as u8 - b'A' + 27,
        _ => 0,
    }) as u32
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
