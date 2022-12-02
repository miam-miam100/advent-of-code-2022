pub fn part_one(input: &str) -> Option<u32> {
    Some(create_iter(input).map(|(x, y)| score_one(x, y)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(create_iter(input).map(|(x, y)| score_two(x, y)).sum())
}

fn create_iter(input: &str) -> impl Iterator<Item = (&str, &str)> {
    input.lines().filter_map(|line| {
        let mut i = line.split_ascii_whitespace();
        Some((i.next()?, i.next()?))
    })
}

fn score_one(opponent: &str, choice: &str) -> u32 {
    match (opponent, choice) {
        ("A", "X") => 1 + 3,
        ("A", "Y") => 2 + 6,
        ("A", "Z") => 3,
        ("B", "X") => 1,
        ("B", "Y") => 2 + 3,
        ("B", "Z") => 3 + 6,
        ("C", "X") => 1 + 6,
        ("C", "Y") => 2,
        ("C", "Z") => 3 + 3,
        _ => 0,
    }
}

fn score_two(opponent: &str, choice: &str) -> u32 {
    match (opponent, choice) {
        ("A", "X") => 3,
        ("A", "Y") => 1 + 3,
        ("A", "Z") => 2 + 6,
        ("B", "X") => 1,
        ("B", "Y") => 2 + 3,
        ("B", "Z") => 3 + 6,
        ("C", "X") => 2,
        ("C", "Y") => 3 + 3,
        ("C", "Z") => 1 + 6,
        _ => 0,
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
