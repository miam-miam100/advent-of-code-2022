pub fn part_one(input: &str) -> Option<i32> {
    let mut current_cycle = 1;
    let mut x = 1;
    Some(
        parse_input(input)
            .filter_map(|(cycles, add)| {
                let mut result = None;
                for _ in 0..cycles {
                    if (current_cycle + 20) % 40 == 0 {
                        result = Some(current_cycle * x);
                    }
                    current_cycle += 1;
                }
                x += add;
                result
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let mut current_cycle = 1;
    let mut x = 1;
    let mut result = String::new();
    for (cycles, add) in parse_input(input) {
        for _ in 0..cycles {
            if (x - 1..=x + 1).contains(&((current_cycle - 1) % 40)) {
                result.push('#');
            } else {
                result.push('.')
            }
            if current_cycle % 40 == 0 && current_cycle < 240 {
                result.push('\n')
            }
            current_cycle += 1;
        }
        x += add;
    }
    Some(result)
}

fn parse_input(input: &str) -> impl Iterator<Item = (i32, i32)> + '_ {
    input.lines().filter_map(|l| {
        Some(if l.starts_with("noop") {
            (1_i32, 0_i32)
        } else {
            let (_, n) = l.split_once(' ')?;
            (2, n.parse().ok()?)
        })
    })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some("##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....".into()));
    }
}
