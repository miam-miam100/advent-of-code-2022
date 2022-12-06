use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    // Works as input only contains ascii
    input
        .as_bytes()
        .windows(4)
        .position(|slice| slice.iter().all_unique())
        .map(|i| i + 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    // Works as input only contains ascii
    input
        .as_bytes()
        .windows(14)
        .position(|slice| slice.iter().all_unique())
        .map(|i| i + 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(29));
    }
}
