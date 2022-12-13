<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2022

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2022 Results

| Day | Solution | Part 1 | Part 2 |
| :---: | :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2022/day/1) | [01.rs](src/bin/01.rs) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2022/day/2) | [02.rs](src/bin/02.rs) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2022/day/3) | [03.rs](src/bin/03.rs) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2022/day/4) | [04.rs](src/bin/04.rs) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2022/day/5) | [05.rs](src/bin/05.rs) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2022/day/6) | [06.rs](src/bin/06.rs) | ⭐ | ⭐ |
| [Day 7](https://adventofcode.com/2022/day/7) | [07.rs](src/bin/07.rs) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2022/day/8) | [08.rs](src/bin/08.rs) | ⭐ | ⭐ |
| [Day 9](https://adventofcode.com/2022/day/9) | [09.rs](src/bin/09.rs) | ⭐ | ⭐ |
| [Day 10](https://adventofcode.com/2022/day/10) | [10.rs](src/bin/10.rs) | ⭐ | ⭐ |
| [Day 11](https://adventofcode.com/2022/day/11) | [11.rs](src/bin/11.rs) | ⭐ | ⭐ |
| [Day 12](https://adventofcode.com/2022/day/12) | [12.rs](src/bin/12.rs) | ⭐ | ⭐ |
| [Day 13](https://adventofcode.com/2022/day/13) | [13.rs](src/bin/13.rs) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

---

## Usage

### Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module "src/bin/01.rs"
# Created empty input file "src/inputs/01.txt"
# Created empty example file "src/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

Individual solutions live in the `./src/bin/` directory as separate binaries.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/main/src/bin/scaffold.rs#L11-L41) has _unit tests_ referencing its _example_ file. Use these unit tests to develop and debug your solution against the example input. For some puzzles, it might be easier to forgo the example file and hardcode inputs into the tests.

When editing a solution, `rust-analyzer` will display buttons for running / debugging unit tests above the unit test blocks.

### Download input for a day

> **Note**  
> This command requires [installing the aoc-cli crate](https://github.com/scarvalhojr/aoc-cli/).

```sh
# example: `cargo download 1`
cargo download <day>

# output:
# Downloading input with aoc-cli...
# Loaded session cookie from "/home/felix/.adventofcode.session".
# Downloading input for day 1, 2021...
# Saving puzzle input to "/tmp/tmp.MBdcAdL9Iw/input"...
# Done!
# ---
# 🎄 Successfully wrote input to "src/inputs/01.txt"!
```

To download inputs for previous years, append the `--year/-y` flag. _(example: `cargo download 1 --year 2020`)_

Puzzle inputs are not checked into git. [Reasoning](https://old.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/?context=3).

### Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Running `target/debug/01`
# 🎄 Part 1 🎄
#
# 6 (elapsed: 37.03µs)
#
# 🎄 Part 2 🎄
#
# 9 (elapsed: 33.18µs)
```

`solve` is an alias for `cargo run --bin`. To run an optimized version for benchmarking, append the `--release` flag.

Displayed _timings_ show the raw execution time of your solution without overhead (e.g. file reads).

### Run all solutions

```sh
cargo run

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# 🎄 Part 1 🎄
#
# 0 (elapsed: 170.00µs)
#
# 🎄 Part 2 🎄
#
# 0 (elapsed: 30.00µs)
# <...other days...>
# Total: 0.20ms
```

`all` is an alias for `cargo run`. To run an optimized version for benchmarking, use the `--release` flag.

_Total timing_ is computed from individual solution _timings_ and excludes as much overhead as possible.

### Run all solutions against the example input

```sh
cargo test
```
