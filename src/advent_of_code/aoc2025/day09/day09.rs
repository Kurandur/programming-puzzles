use std::u64;

#[derive(Clone, Copy)]
struct Edge {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Red,
    Other,
    Green,
}

fn sort(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn manhattan(a: (i32, i32), b: (i32, i32)) -> u64 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u64
}

fn rectangle_area(a: (i32, i32), b: (i32, i32)) -> u64 {
    ((a.0 - b.0).abs() as u64 + 1) * ((a.1 - b.1).abs() as u64 + 1)
}

pub struct Floor {
    red_tiles: Vec<(i32, i32)>,
    width: i32,
    height: i32,
}

impl Floor {
    pub fn from_input(input: &str) -> Self {
        let mut red_tiles = Vec::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for line in input.lines() {
            let (x, y) = line.split_once(',').unwrap();
            let x: i32 = x.trim().parse().unwrap();
            let y: i32 = y.trim().parse().unwrap();

            red_tiles.push((x, y));
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        Self {
            red_tiles,
            width: max_x + 1,
            height: max_y + 1,
        }
    }

    pub fn largest_area(&self) -> u64 {
        let mut max_area: u64 = 0;

        for i in 0..self.red_tiles.len() {
            for j in i + 1..self.red_tiles.len() {
                let (x1, y1) = self.red_tiles[i];
                let (x2, y2) = self.red_tiles[j];

                let width = (x1 - x2).abs() as u64 + 1;
                let height = (y1 - y2).abs() as u64 + 1;

                if width > 1 && height > 1 {
                    let area = width * height;
                    max_area = max_area.max(area);
                }
            }
        }

        max_area
    }

    pub fn largest_area_constrained(&self) -> u64 {
        let n = self.red_tiles.len();
        let mut edges = Vec::with_capacity(n);

        for i in 0..n {
            let (x1, y1) = self.red_tiles[i];
            let (x2, y2) = self.red_tiles[(i + 1) % n];
            edges.push(Edge { x1, y1, x2, y2 });
        }

        let intersects = |min_x: i32, min_y: i32, max_x: i32, max_y: i32| -> bool {
            for e in &edges {
                let (ex_min, ex_max) = sort(e.x1, e.x2);
                let (ey_min, ey_max) = sort(e.y1, e.y2);

                if min_x < ex_max && max_x > ex_min && min_y < ey_max && max_y > ey_min {
                    return true;
                }
            }
            false
        };

        let mut result = 0u64;

        for i in 0..n {
            for j in i + 1..n {
                let a = self.red_tiles[i];
                let b = self.red_tiles[j];

                if a.0 == b.0 || a.1 == b.1 {
                    continue;
                }

                let (min_x, max_x) = sort(a.0, b.0);
                let (min_y, max_y) = sort(a.1, b.1);

                let d = manhattan(a, b);
                if d * d <= result {
                    continue;
                }

                if !intersects(min_x, min_y, max_x, max_y) {
                    let area = rectangle_area(a, b);
                    result = result.max(area);
                }
            }
        }

        result
    }
}

pub fn generator(input: &str) -> Floor {
    Floor::from_input(input)
}

pub fn part_one(floor: Floor) -> u64 {
    floor.largest_area()
}

pub fn part_two(floor: Floor) -> u64 {
    floor.largest_area_constrained()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}
}
