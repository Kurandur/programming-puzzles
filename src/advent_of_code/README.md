# ⭐Advent of Code🎄

|                 Year                 |  Stars   |
| :----------------------------------: | :------: |
| [2024](/src/advent_of_code/aoc2024/) | 28/50 ⭐ |
|                 2023                 | 0/50 ⭐  |
|                 2022                 | 0/50 ⭐  |
|                 2021                 | 0/50 ⭐  |
|                 2020                 | 0/50 ⭐  |

## Usage

This repo includes a small cli utility for running the puzzle solutions:

### Advent of Code

The cli has helper commands for scaffolding, downloading and running advent of code puzzles.

#### Scaffold

To scaffold a new aoc year folder run:

```
cargo run aoc scaffold year <YEAR>
```

Afterwards you can scaffold a day with:

```
cargo run aoc scaffold day <YEAR> <DAY>
```

This will create a subfolder for the day, download the puzzle, convert it to markdown and
create a day file with this template.

```rust
pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn part_one(input: &str) -> u32 {
    u32::MAX
}

pub fn part_two(input: &str) -> u32 {
    u32::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}
}
```

The only thing that needs to be done by hand is adding the new day to the match pattern in the `run_solution` function like this:

```rust
...
(2024, 1, 1) => aoc2024::day01::part_one(&aoc2024::day01::generator(input)),
(2024, 1, 2) => aoc2024::day01::part_two(&aoc2024::day01::generator(input)),
...
```

I _hopefully_ will add a macro for this.

#### Running solutions

Before being able to run solutions you have to download your puzzle inputs.

This can be automated, but before we have to save our auth token (which can be grabbed from your browsers localstorage)

```
cargo run aoc session set <TOKEN>
```

The token will be saved under `~/.aoc_session`

To download a puzzle input use the following command:

```
cargo run aoc download <YEAR> <DAY>
```

Finally solutions can be run with:

```
cargo run aoc run <YEAR> <DAY> <PART>
```
