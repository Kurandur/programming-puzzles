# Programming Puzzles

Various solutions to different programming-puzzles

## Puzzles

- [⭐Advent of Code🎄](src/advent_of_code)

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

This will create a file with this template:

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

#### Running solutions

Before being able to run soltions you have to download your puzzle inputs.

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
