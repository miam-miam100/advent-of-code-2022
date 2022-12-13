use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Packet {
    Elem(u32),
    Array(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self, &other) {
            (Packet::Elem(l), Packet::Elem(r)) => l.cmp(r),
            (Packet::Elem(l), Packet::Array(_)) => {
                let left = Packet::Array(vec![Packet::Elem(*l)]);
                left.cmp(other)
            }
            (Packet::Array(_), Packet::Elem(r)) => {
                let right = Packet::Array(vec![Packet::Elem(*r)]);
                self.cmp(&right)
            }
            (Packet::Array(left), Packet::Array(right)) => {
                for (l, r) in left.iter().zip(right.iter()) {
                    let compare = l.cmp(r);
                    if compare.is_ne() {
                        return compare;
                    }
                }
                left.len().cmp(&right.len())
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    pub fn push(&mut self, elem: Packet) {
        if let Packet::Array(v) = self {
            v.push(elem);
        }
    }

    pub fn parse(input: &str) -> Option<Packet> {
        let mut start: Option<usize> = None;
        let mut stack = vec![];
        let mut current_element: Option<Packet> = None;
        for (idx, c) in input.char_indices() {
            if !('0'..='9').contains(&c) && start.is_some() {
                let elem = input.get(start?..idx).and_then(|s| s.parse().ok())?;
                start = None;
                current_element.as_mut()?.push(Packet::Elem(elem));
            }
            match c {
                '0'..='9' => {
                    if start.is_none() {
                        start = Some(idx);
                    }
                }
                '[' => {
                    if let Some(current_element) = current_element {
                        stack.push(current_element);
                    }
                    current_element = Some(Packet::Array(vec![]));
                }
                ']' => {
                    if let Some(mut popped) = stack.pop() {
                        if let Some(current_element) = current_element {
                            popped.push(current_element);
                        }
                        current_element = Some(popped);
                    }
                }
                ' ' | ',' => {}
                _ => {
                    return None;
                }
            }
        }
        current_element
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        get_input(input)
            .enumerate()
            .filter(|(_, (l, r))| l.cmp(r) == Ordering::Less)
            .map(|(i, _)| i + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let insert = [
        Packet::Array(vec![Packet::Array(vec![Packet::Elem(2)])]),
        Packet::Array(vec![Packet::Array(vec![Packet::Elem(6)])]),
    ];
    Some(
        get_input(input)
            .flat_map(|(l, r)| [l, r])
            .chain(insert.clone().into_iter())
            .sorted()
            .enumerate()
            .filter(|(_, e)| insert.contains(e))
            .map(|(i, _)| i + 1)
            .product(),
    )
}

fn get_input(input: &str) -> (impl Iterator<Item = (Packet, Packet)> + '_) {
    input.split("\n\n").filter_map(|s| {
        let (l, r) = s.split_once('\n')?;
        Some((Packet::parse(l)?, Packet::parse(r)?))
    })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
