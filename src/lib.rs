use chrono::{Datelike, FixedOffset, TimeZone, Utc};
use std::env;
use std::fs;

pub mod helpers;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

const RELEASE_TIMEZONE_OFFSET: i32 = -5 * 3600;

#[macro_export]
macro_rules! solve {
    ($part:expr, $solver:ident, $( $input: expr ),+) => {{
        use advent_of_code::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
        use std::fmt::Display;
        use std::time::Instant;

        let timer = Instant::now();
        let result = $solver($($input),+);
        let elapsed = timer.elapsed();
        println!("🎄 {}Part {}{} 🎄", ANSI_BOLD, $part, ANSI_RESET);
        match result {
            Some(result) => {
                println!(
                    "{} {}(elapsed: {:.2?}){}",
                    result, ANSI_ITALIC, elapsed, ANSI_RESET
                );
            }
            None => {
                println!("not solved.")
            }
        }
    }};
}

pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd.join("src").join(folder).join(format!("{:02}.txt", day));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

fn parse_time(val: &str, postfix: &str) -> f64 {
    val.split(postfix).next().unwrap().parse().unwrap()
}

pub fn parse_exec_time(output: &str) -> f64 {
    output.lines().fold(0_f64, |acc, l| {
        if !l.contains("elapsed:") {
            acc
        } else {
            let timing = l.split("(elapsed: ").last().unwrap();
            // use `contains` istd. of `ends_with`: string may contain ANSI escape sequences.
            // for possible time formats, see: https://github.com/rust-lang/rust/blob/1.64.0/library/core/src/time.rs#L1176-L1200
            if timing.contains("ns)") {
                acc // range below rounding precision.
            } else if timing.contains("µs)") {
                acc + parse_time(timing, "µs") / 1000_f64
            } else if timing.contains("ms)") {
                acc + parse_time(timing, "ms")
            } else if timing.contains("s)") {
                acc + parse_time(timing, "s") * 1000_f64
            } else {
                acc
            }
        }
    })
}

pub fn current_event_day() -> Option<u8> {
    let now =
        FixedOffset::east_opt(RELEASE_TIMEZONE_OFFSET)?.from_utc_datetime(&Utc::now().naive_utc());
    let day = now.day();
    if now.month() == 12 && (1..=25).contains(&day) {
        Some(day as u8)
    } else {
        None
    }
}

/// copied from: https://github.com/rust-lang/rust/blob/1.64.0/library/std/src/macros.rs#L328-L333
#[cfg(test)]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let (a, b) = (&$a, &$b);
        assert!(
            (*a - *b).abs() < 1.0e-6,
            "{} is not approximately equal to {}",
            *a,
            *b
        );
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_exec_time() {
        assert_approx_eq!(
            parse_exec_time(&format!(
                "🎄 Part 1 🎄\n0 (elapsed: 74.13ns){}\n🎄 Part 2 🎄\n0 (elapsed: 50.00ns){}",
                ANSI_RESET, ANSI_RESET
            )),
            0_f64
        );

        assert_approx_eq!(
            parse_exec_time("🎄 Part 1 🎄\n0 (elapsed: 755µs)\n🎄 Part 2 🎄\n0 (elapsed: 700µs)"),
            1.455_f64
        );

        assert_approx_eq!(
            parse_exec_time("🎄 Part 1 🎄\n0 (elapsed: 70µs)\n🎄 Part 2 🎄\n0 (elapsed: 1.45ms)"),
            1.52_f64
        );

        assert_approx_eq!(
            parse_exec_time(
                "🎄 Part 1 🎄\n0 (elapsed: 10.3s)\n🎄 Part 2 🎄\n0 (elapsed: 100.50ms)"
            ),
            10400.50_f64
        );
    }
}
