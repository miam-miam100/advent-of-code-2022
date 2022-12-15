use prse::try_parse;
use std::cmp::{max, min};

pub fn part_one(input: &str, row: i32) -> Option<usize> {
    let sensors: Vec<_> = get_input(input)
        .map(|(x, y, bx, by)| (x, y, bx, by, bx.abs_diff(x) + by.abs_diff(y)))
        .collect();
    let min_x = sensors.iter().map(|(x, _, _, _, d)| x - *d as i32).min()?;
    let max_x = sensors.iter().map(|(x, _, _, _, d)| x + *d as i32).max()?;
    Some(
        (min_x..=max_x)
            .filter(|x_pos| {
                sensors.iter().any(|(x, y, bx, by, d)| {
                    !(x_pos == bx && row == *by) && x_pos.abs_diff(*x) + row.abs_diff(*y) <= *d
                })
            })
            .count(),
    )
}

fn peripheral(
    start_bound: i32,
    end_bound: i32,
    d: i32,
    (x_s, x_e): (i32, i32),
    (y_s, y_e): (i32, i32),
) -> impl Iterator<Item = (i32, i32)> {
    let mut start = 0;
    let mut end = d + 1;
    if x_s < start_bound {
        start = max(start, start_bound - x_s);
    } else if x_e > end_bound {
        end = min(end, d + 1 - (x_e - end_bound));
    }
    if y_s < start_bound {
        start = max(start, start_bound - y_s);
    } else if y_e > end_bound {
        end = min(end, d + 1 - (y_e - end_bound));
    }
    (start..=end).map(move |i| (x_s + i, y_s + i))
}
// x_i = x - d - 1
// y_i = y
pub fn part_two(input: &str, max_size: u32) -> Option<u64> {
    let sensors: Vec<_> = get_input(input)
        .map(|(x, y, bx, by)| (x, y, (bx.abs_diff(x) + by.abs_diff(y)) as i32))
        .collect();

    let mut i = sensors.iter().flat_map(|&(x, y, d)| {
        let ul = peripheral(0, max_size as i32, d, (x - d - 1, x), (y, y + d + 1));
        let ur = peripheral(0, max_size as i32, d, (x, x + d + 1), (y, y + d + 1));
        let dl = peripheral(0, max_size as i32, d, (x - d - 1, x), (y - d - 1, y));
        let dr = peripheral(0, max_size as i32, d, (x, x + d + 1), (y - d - 1, y));
        ul.chain(ur).chain(dl).chain(dr)
    });

    i.find(|(x_pos, y_pos)| {
        !sensors
            .iter()
            .any(|(x, y, d)| x_pos.abs_diff(*x) + y_pos.abs_diff(*y) <= *d as u32)
    })
    .map(|(x, y)| (x as u64 * 4000000 + y as u64))

    // let mut x_pos = 0;
    // let mut y_pos = 0;
    //
    // loop {
    //     if x_pos > max_size {
    //         y_pos += 1;
    //         x_pos = 0;
    //     } else if y_pos > max_size {
    //         break;
    //     }
    //
    //     let x_skip = sensors
    //         .iter()
    //         .filter_map(|(x, y, d)| Some((d * 2 + 1).checked_sub(2 * (y.abs_diff(y_pos as i32)))?))
    //         .min();
    //
    //     match x_skip {
    //         None => {
    //             x_pos = 0;
    //             y_pos += 1;
    //         }
    //         Some(s) if x_pos + s > max_size => {
    //             x_pos = 0;
    //             y_pos += 1;
    //         }
    //         Some(s) => {
    //             x_pos += s;
    //         }
    //     }
    // }
}

fn get_input(input: &str) -> impl Iterator<Item = (i32, i32, i32, i32)> + '_ {
    input
        .lines()
        .filter_map(|l| try_parse!(l, "Sensor at x={}, y={}: closest beacon is at x={}, y={}").ok())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input, 2000000);
    advent_of_code::solve!(2, part_two, input, 4000000);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input, 10), Some(26));
        assert_eq!(part_one(&input, 11), Some(28));
        assert_eq!(part_one(&input, 9), Some(25));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input, 20), Some(56000011));
    }
}
