use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let dirs = get_dirs(input)?;
    Some(dirs.iter().map(|(_, s)| *s).filter(|s| *s <= 100000).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let dirs = get_dirs(input)?;
    let space_needed = 30000000 - (70000000 - dirs.get("/")?);
    dirs.into_iter()
        .map(|(_, s)| s)
        .filter(|s| *s >= space_needed)
        .min()
}

fn get_dirs(input: &str) -> Option<HashMap<String, u32>> {
    let mut current_path = String::from("/");
    let mut dirs = HashMap::new();

    for l in input.lines() {
        if l.starts_with("$ ") {
            if l.starts_with("$ cd ") {
                let cd = l.split_whitespace().nth(2)?;
                match cd {
                    ".." => {
                        if let Some(i) = current_path.strip_suffix('/')?.rfind('/') {
                            current_path.truncate(i + 1)
                        }
                    }
                    "/" => current_path = String::from("/"),
                    x => {
                        current_path.push_str(x);
                        current_path.push('/');
                    }
                }
            }
            if l.starts_with("$ ls") {
                dirs.insert(current_path.clone(), 0);
            }
        } else if !l.starts_with("dir") {
            let mut i = l.split_whitespace();
            if let Some(s) = dirs.get_mut(&current_path) {
                *s += i.next()?.parse::<u32>().ok()?
            }
        }
    }

    let mut result = HashMap::with_capacity(dirs.len());

    for (d, s) in dirs {
        result.entry(d.clone()).and_modify(|v| *v += s).or_insert(s);
        let (mut dir, _) = d.rsplit_once('/')?;
        while let Some((sd, _)) = dir.rsplit_once('/') {
            dir = sd;
            let mut k = String::from(dir);
            k.push('/');
            result.entry(k).and_modify(|v| *v += s).or_insert(s);
        }
    }
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
