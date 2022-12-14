use itertools::Itertools;
use std::cmp::{max, min};
use std::mem;

pub fn part_one(input: &str) -> Option<u32> {
    let (mut points, start, line_length) = get_input_one(input)?;
    let mut y = 0;
    let mut x = start;
    let mut count = 0;
    while let Some(resting) = sand_step(&mut x, &mut y, line_length, start, &mut points) {
        if resting {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut points, start, line_length) = get_input_two(input)?;
    let mut y = 0;
    let mut x = start;
    let mut count = 0;
    while !points[start] {
        if sand_step(&mut x, &mut y, line_length, start, &mut points)? {
            count += 1;
        }
    }
    Some(count)
}

fn sand_step(
    x: &mut usize,
    y: &mut usize,
    line_length: usize,
    start: usize,
    points: &mut [bool],
) -> Option<bool> {
    let down_idx = *x + (*y + 1) * line_length;
    Some(match points.get(down_idx)? {
        true => match points.get(down_idx - 1)? {
            true => match points.get(down_idx + 1)? {
                true => {
                    points[*x + *y * line_length] = true;
                    *x = start;
                    *y = 0;
                    true
                }
                false => {
                    *x += 1;
                    false
                }
            },
            false => {
                if *x == 0 {
                    return None;
                }
                *x -= 1;
                false
            }
        },
        false => {
            *y += 1;
            false
        }
    })
}

fn get_points(input: &str) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = vec![];
    for l in input.lines() {
        for ((mut x1, mut y1), (mut x2, mut y2)) in l
            .split(" -> ")
            .filter_map(|c| c.split_once(','))
            .filter_map(|(x, y)| Some((x.parse::<usize>().ok()?, y.parse::<usize>().ok()?)))
            .tuple_windows()
        {
            if y1 == y2 {
                if x2 < x1 {
                    mem::swap(&mut x1, &mut x2);
                }
                points.extend((x1..=x2).map(|x| (x, y1)));
            } else if x1 == x2 {
                if y2 < y1 {
                    mem::swap(&mut y1, &mut y2);
                }
                points.extend((y1..=y2).map(|y| (x1, y)));
            }
        }
    }
    points
}

fn get_input_one(input: &str) -> Option<(Vec<bool>, usize, usize)> {
    let points = get_points(input);
    let (&x_min, &x_max) = points.iter().map(|(x, _)| x).minmax().into_option()?;
    let depth = *points.iter().map(|(_, y)| y).max()? as usize;
    let line_x = (x_max - x_min + 1) as usize;
    let mut result = vec![false; line_x * (depth + 1)];
    for (x, y) in points {
        let idx = x - x_min + y * line_x;
        result[idx] = true;
    }
    Some((result, (500 - x_min) as usize, line_x))
}

fn get_input_two(input: &str) -> Option<(Vec<bool>, usize, usize)> {
    let points = get_points(input);
    let min_max = points.iter().map(|(x, _)| x).minmax().into_option()?;
    let floor = 2 + *points.iter().map(|(_, y)| y).max()? as usize;
    let min_max = (min(500 - floor, *min_max.0), max(500 + floor, *min_max.1));
    let line_x = (min_max.1 - min_max.0 + 1) as usize;
    let mut result = vec![false; line_x * (floor + 1)];
    for (x, y) in points {
        let idx = x - min_max.0 + y * line_x;
        result[idx] = true;
    }
    for x in result.iter_mut().skip(floor * line_x) {
        *x = true;
    }
    Some((result, (500 - min_max.0) as usize, line_x))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
