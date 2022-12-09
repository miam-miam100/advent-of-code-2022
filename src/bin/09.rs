use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    let movements = get_movements(input);
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut visited = HashSet::from([(0, 0)]);
    for ((dx, dy), step) in movements {
        for _ in 0..step {
            head_pos.0 += dx;
            head_pos.1 += dy;
            if step_tail(head_pos, &mut tail_pos) {
                visited.insert(tail_pos);
            }
        }
    }
    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let movements = get_movements(input);
    let mut heads_pos = [(0, 0); 10];
    let mut visited = HashSet::from([(0, 0)]);
    for ((dx, dy), step) in movements {
        for _ in 0..step {
            heads_pos[0].0 += dx;
            heads_pos[0].1 += dy;
            for idx in 1..10 {
                if step_tail(heads_pos[idx - 1], &mut heads_pos[idx]) && idx == 9 {
                    visited.insert(heads_pos[idx]);
                }
            }
        }
    }
    Some(visited.len())
}

fn get_movements(input: &str) -> impl Iterator<Item = ((i32, i32), u32)> + '_ {
    input.lines().filter_map(|l| {
        let (dir, steps) = l.split_once(' ')?;
        let delta = match dir {
            "U" => (0, 1),
            "D" => (0, -1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => {
                return None;
            }
        };
        Some((delta, steps.parse().ok()?))
    })
}

fn step_tail(head: (i32, i32), tail: &mut (i32, i32)) -> bool {
    match ((head.0 - tail.0), (head.1 - tail.1)) {
        (0, 0) | (0, 1) | (0, -1) | (1, 0) | (-1, 0) | (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => {
            false
        }
        (0, y) if y < 0 => {
            tail.1 += -1;
            true
        }
        (0, y) if y > 0 => {
            tail.1 += 1;
            true
        }
        (x, 0) if x < 0 => {
            tail.0 += -1;
            true
        }
        (x, 0) if x > 0 => {
            tail.0 += 1;
            true
        }
        (x, y) => {
            tail.0 += if x > 0 { 1 } else { -1 };
            tail.1 += if y > 0 { 1 } else { -1 };
            true
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
