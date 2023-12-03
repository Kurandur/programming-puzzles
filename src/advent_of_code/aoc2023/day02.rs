#[derive(Debug)]
struct Set {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug)]
pub struct Game {
    number: u32,
    sets: Vec<Set>,
}

pub fn generator(input: &str) -> Vec<Game> {
    let mut games = Vec::new();
    for line in input.lines() {
        let (game, sets) = line.split_once(":").unwrap();
        let (_, game_number) = game.split_once(" ").unwrap();

        let mut game = Game {
            number: game_number.parse().unwrap(),
            sets: Vec::new(),
        };

        for set_string in sets.split(";") {
            let mut set = Set {
                blue: 0,
                red: 0,
                green: 0,
            };
            for colored_cubes in set_string.split(",") {
                let (count, color) = colored_cubes.trim().split_once(" ").unwrap();
                match color {
                    "red" => set.red = count.parse().unwrap(),
                    "green" => set.green = count.parse().unwrap(),
                    "blue" => set.blue = count.parse().unwrap(),
                    _ => (),
                }
            }
            game.sets.push(set)
        }
        games.push(game);
    }
    games
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn part_one(input: Vec<Game>) -> u32 {
    let sum: u32 = input
        .iter()
        .filter(|game| {
            game.sets
                .iter()
                .all(|set| MAX_RED >= set.red && MAX_GREEN >= set.green && MAX_BLUE >= set.blue)
        })
        .map(|game| game.number)
        .sum();

    sum
}

pub fn part_two(input: Vec<Game>) -> u32 {
    let mut sum: u32 = 0;
    for game in input {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in game.sets {
            if set.red > max_red {
                max_red = set.red
            }
            if set.green > max_green {
                max_green = set.green
            }
            if set.blue > max_blue {
                max_blue = set.blue
            }
        }
        sum += max_red * max_blue * max_green;
    }
    sum
}

#[cfg(test)]
mod tests {}
