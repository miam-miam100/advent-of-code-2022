pub fn part_one(input: &str) -> Option<u32> {
    let (size, array) = parse_input(input)?;
    let mut visibility = vec![false; size * size];

    for h in 0..size {
        check_visible(
            array.iter().enumerate().skip(h * size).take(size),
            &mut visibility,
        );
        check_visible(
            array.iter().enumerate().skip(h * size).take(size).rev(),
            &mut visibility,
        );
    }

    for v in 0..size {
        check_visible(
            array.iter().enumerate().skip(v).step_by(size).take(size),
            &mut visibility,
        );
        check_visible(
            array
                .iter()
                .enumerate()
                .skip(v)
                .step_by(size)
                .take(size)
                .rev(),
            &mut visibility,
        );
    }

    Some(visibility.iter().filter(|b| **b).count() as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (size, array) = parse_input(input)?;
    (0..(size * size))
        .into_iter()
        .filter_map(|idx| {
            let right = check_sightline(array.iter().skip(idx).take(size - (idx % size)))?;
            let left = check_sightline(
                array
                    .iter()
                    .skip(idx - (idx % size))
                    .take(idx % size + 1)
                    .rev(),
            )?;
            let down = check_sightline(
                array
                    .iter()
                    .skip(idx)
                    .step_by(size)
                    .take(size - (idx / size)),
            )?;
            let up = check_sightline(
                array
                    .iter()
                    .skip(idx % size)
                    .step_by(size)
                    .take(idx / size + 1)
                    .rev(),
            )?;
            Some(right * left * down * up)
        })
        .max()
}

fn check_visible<'a>(iter: impl Iterator<Item = (usize, &'a u8)>, visibility: &mut [bool]) {
    let mut max_tree = None;
    for (idx, t) in iter {
        if match max_tree {
            None => true,
            Some(max_tree) => t > max_tree,
        } {
            max_tree = Some(t);
            visibility[idx] = true;
        }
    }
}

fn check_sightline<'a>(mut iter: impl Iterator<Item = &'a u8>) -> Option<usize> {
    let tree = iter.next()?;
    let mut count = 0;
    for t in iter {
        count += 1;
        if t >= tree {
            break;
        }
    }
    Some(count)
}

fn parse_input(input: &str) -> Option<(usize, Vec<u8>)> {
    let mut size = 0;
    let result: Vec<u8> = input
        .lines()
        .flat_map(|l| {
            size = l.len();
            l.chars().flat_map(|c| c.to_digit(10).map(|d| d as u8))
        })
        .collect();
    Some((size, result))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
