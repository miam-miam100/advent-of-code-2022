use pathfinding::prelude::astar;

pub fn part_one(input: &str) -> Option<u32> {
    let (line_length, (s, e), array) = get_input(input);
    let result = astar(
        &s,
        |p| successors(*p, line_length, &array),
        |p| heuristic(*p, line_length, e, &array),
        |p| *p == e,
    )?;
    Some(result.1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (line_length, (s, e), array) = get_input(input);
    let result = astar(
        &s,
        |p| successors_shim(*p, line_length, s, &array),
        |p| heuristic(*p, line_length, e, &array),
        |p| *p == e,
    )?;
    Some(result.1)
}

fn successors(
    position: usize,
    line_length: usize,
    array: &[u8],
) -> impl Iterator<Item = (usize, u32)> + '_ {
    let position = position as isize;
    [-1, 1, -(line_length as isize), line_length as isize]
        .into_iter()
        .filter_map(move |p| {
            let p = (p + position).try_into().ok()?;
            if array.get(p)? <= &(array.get(position as usize)? + 1) {
                Some((p, 1))
            } else {
                None
            }
        })
}

fn successors_shim(
    position: usize,
    line_length: usize,
    start: usize,
    array: &[u8],
) -> impl Iterator<Item = (usize, u32)> + '_ {
    // Need to do this as can't return two different iterators with different types
    successors(position, line_length, array).chain(
        array
            .iter()
            .take(if position == start { usize::MAX } else { 0 })
            .enumerate()
            .filter(|(_, c)| c == &&0)
            .map(|(i, _)| (i, 0)),
    )
}

fn heuristic(position: usize, size: usize, end: usize, array: &[u8]) -> u32 {
    let x = ((end % size) as isize).abs_diff((position % size) as isize);
    let y = ((end / size) as isize).abs_diff((position / size) as isize);
    (x + y) as u32 + (array[position] as u32) * 2
}

fn get_input(input: &str) -> (usize, (usize, usize), Vec<u8>) {
    let mut size = 0;
    let mut result = vec![];
    let mut line_length = 0;
    let mut end = 0;

    for (idx, l) in input.lines().enumerate() {
        size = l.len();
        result.extend(l.chars().enumerate().map(|(i, c)| match c {
            'S' => {
                line_length = idx * size + i;
                0
            }
            'E' => {
                end = idx * size + i;
                b'z' - b'a'
            }
            c => c as u8 - b'a',
        }));
    }
    (size, (line_length, end), result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
