use itertools::Itertools;
use num_integer::Integer;
use pest::Parser;
use std::collections::VecDeque;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "monkey.pest"]
pub struct MonkeyParser;

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: u64,
    true_condition: usize,
    false_condition: usize,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut monkeys, div) = get_input(input)?;
    let mut inspections = vec![0_u32; monkeys.len()];
    for _ in 1..=20 {
        for idx in 0..monkeys.len() {
            let monkey = monkeys.get_mut(idx)?;
            let mut items = vec![];
            while let Some(item) = monkey.items.pop_front() {
                let worry = ((monkey.operation)(item) / 3) % div;
                *inspections.get_mut(idx)? += 1;
                items.push((
                    if worry % monkey.test == 0 {
                        monkey.true_condition
                    } else {
                        monkey.false_condition
                    },
                    worry,
                ));
            }
            for (monkey, worry) in items {
                monkeys.get_mut(monkey)?.items.push_back(worry);
            }
        }
    }
    Some(inspections.into_iter().sorted().rev().take(2).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut monkeys, div) = get_input(input)?;
    let mut inspections = vec![0_u64; monkeys.len()];
    for _ in 1..=10000 {
        for idx in 0..monkeys.len() {
            let monkey = monkeys.get_mut(idx)?;
            let mut items = vec![];
            while let Some(item) = monkey.items.pop_front() {
                let worry: u64 = (monkey.operation)(item) % div;
                *inspections.get_mut(idx)? += 1;
                items.push((
                    if worry % monkey.test == 0 {
                        monkey.true_condition
                    } else {
                        monkey.false_condition
                    },
                    worry,
                ));
            }
            for (monkey, worry) in items {
                monkeys.get_mut(monkey)?.items.push_back(worry);
            }
        }
    }
    Some(inspections.into_iter().sorted().rev().take(2).product())
}

fn get_input(input: &str) -> Option<(Vec<Monkey>, u64)> {
    let main = MonkeyParser::parse(Rule::main, input).ok()?.next()?;
    let mut divisibility = vec![];
    Some((
        main.into_inner()
            .filter(|m| m.as_rule() == Rule::monkey)
            .filter_map(|m| {
                let (mut items, mut operation, mut test, mut true_condition, mut false_condition) =
                    (None, None, None, None, None);
                for properties in m.into_inner() {
                    match properties.as_rule() {
                        Rule::name => {}
                        Rule::items => {
                            let i = properties
                                .into_inner()
                                .filter_map(|i| i.as_str().parse().ok());
                            items = Some::<VecDeque<u64>>(i.collect());
                        }
                        Rule::operation => {
                            operation = get_operations(properties.into_inner().map(|o| o.as_str()));
                        }
                        Rule::test => {
                            let mut tests = properties
                                .into_inner()
                                .filter_map(|i| i.as_str().parse().ok());
                            test = tests.next();
                            divisibility.push(test?);
                            true_condition = tests.next();
                            false_condition = tests.next();
                        }
                        _ => unreachable!(),
                    }
                }
                Some(Monkey {
                    items: items?,
                    operation: operation?,
                    test: test?,
                    true_condition: true_condition? as usize,
                    false_condition: false_condition? as usize,
                })
            })
            .collect(),
        divisibility.iter().fold(1, |lhs, rhs| lhs.lcm(rhs)),
    ))
}

fn get_operations<'a>(
    mut operations: impl Iterator<Item = &'a str>,
) -> Option<Box<dyn Fn(u64) -> u64>> {
    let term1 = operations.next()?;
    let op = operations.next()?;
    let term2 = operations.next()?;
    Some(match (op, term1, term2) {
        ("+", "old", "old") => Box::new(|old| old + old),
        ("+", "old", x) | ("+", x, "old") => {
            let x = x.parse::<u64>().ok()?;
            Box::new(move |old| old + x)
        }
        ("+", x, y) => {
            let x = x.parse::<u64>().ok()?;
            let y = y.parse::<u64>().ok()?;
            Box::new(move |_| x + y)
        }
        ("*", "old", "old") => Box::new(|old| old * old),
        ("*", "old", x) | ("*", x, "old") => {
            let x = x.parse::<u64>().ok()?;
            Box::new(move |old| old * x)
        }
        ("*", x, y) => {
            let x = x.parse::<u64>().ok()?;
            let y = y.parse::<u64>().ok()?;
            Box::new(move |_| x * y)
        }
        _ => unreachable!(),
    })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
