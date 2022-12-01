pub fn part_one(input: &str) -> Option<u32> {
    let mut elves = vec![0];
    for line in input.lines() {
        if line.is_empty() {
            elves.push(0);
        } else {
            let length = elves.len() - 1;
            elves[length] += str::parse::<u32>(line).unwrap();
        }
    }
    elves.into_iter().max()
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut elves = vec![0];
    for line in input.lines() {
        if line.is_empty() {
            elves.push(0);
        } else {
            let length = elves.len() - 1;
            elves[length] += str::parse::<u32>(line).unwrap();
        }
    }
    elves.sort_by(|i, z| z.cmp(i));
    Some(elves[0] + elves[1] + elves[2])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
