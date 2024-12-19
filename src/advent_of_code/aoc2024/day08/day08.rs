use std::{
    collections::{HashMap, HashSet},
    u32,
};

pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn part_one(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != '.' {
                antennas.entry(cell).or_default().push((x as i32, y as i32));
            }
        }
    }
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut antinodes = HashSet::new();

    for (&_frequency, positions) in &antennas {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                let dx = x2 - x1;
                let dy = y2 - y1;

                let point1 = (x1 - dx, y1 - dy);
                let point2 = (x2 + dx, y2 + dy);

                if check_bounds(point1, rows, cols) {
                    antinodes.insert(point1);
                }
                if check_bounds(point2, rows, cols) {
                    antinodes.insert(point2);
                }
            }
        }
    }

    antinodes.len() as u32
}

fn check_bounds((x, y): (i32, i32), rows: i32, cols: i32) -> bool {
    x >= 0 && x < cols && y >= 0 && y < rows
}

pub fn part_two(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != '.' {
                antennas.entry(cell).or_default().push((x as i32, y as i32));
            }
        }
    }
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut antinodes = HashSet::new();

    for (&_frequency, positions) in &antennas {
        for i in 0..positions.len() {
            let (x1, y1) = positions[i];

            antinodes.insert((x1, y1));

            for j in 0..positions.len() {
                if i == j {
                    continue;
                }

                let (x2, y2) = positions[j];
                let dx = x2 - x1;
                let dy = y2 - y1;

                let mut point = (x1 - dx, y1 - dy);
                while check_bounds(point, rows, cols) {
                    antinodes.insert(point);
                    point = (point.0 - dx, point.1 - dy);
                }

                point = (x2 + dx, y2 + dy);
                while check_bounds(point, rows, cols) {
                    antinodes.insert(point);
                    point = (point.0 + dx, point.1 + dy);
                }
            }
        }
    }
    antinodes.len() as u32
}

#[cfg(test)]
mod tests {}
