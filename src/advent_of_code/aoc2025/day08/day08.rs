use std::{str::FromStr, u32};

struct Circuits {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Circuits {
    fn new(n: usize) -> Self {
        Circuits {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb {
            return false;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];

        true
    }

    fn circuit_sizes(&mut self) -> Vec<usize> {
        let mut result = Vec::new();
        for i in 0..self.parent.len() {
            if self.find(i) == i {
                result.push(self.size[i]);
            }
        }
        result
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct JunctionBox {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug)]
struct Connection {
    a: usize,
    b: usize,
    dist: u64,
}

impl JunctionBox {
    fn distance(&self, other: &Self) -> u64 {
        let dx = self.x as i64 - other.x as i64;
        let dy = self.y as i64 - other.y as i64;
        let dz = self.z as i64 - other.z as i64;

        (dx * dx + dy * dy + dz * dz) as u64
    }
}

impl FromStr for JunctionBox {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let x = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let y = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let z = parts.next().ok_or(())?.parse().map_err(|_| ())?;

        Ok(JunctionBox { x, y, z })
    }
}

pub fn generator(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .map(|l| JunctionBox::from_str(l).unwrap())
        .collect()
}

pub fn solve_part_one(junction_boxes: Vec<JunctionBox>, connections_to_use: usize) -> u32 {
    let junction_count = junction_boxes.len();

    let mut connections = Vec::new();
    for i in 0..junction_count {
        for j in i + 1..junction_count {
            connections.push(Connection {
                a: i,
                b: j,
                dist: junction_boxes[i].distance(&junction_boxes[j]),
            });
        }
    }

    connections.sort_unstable_by(|a, b| a.dist.cmp(&b.dist));

    let mut circuits = Circuits::new(junction_count);

    for c in connections.iter().take(connections_to_use) {
        circuits.union(c.a, c.b);
    }

    let mut sizes = circuits.circuit_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    (sizes[0] * sizes[1] * sizes[2]) as u32
}

pub fn part_one(junction_boxes: Vec<JunctionBox>) -> u32 {
    solve_part_one(junction_boxes, 1000)
}

pub fn part_two(junction_boxes: Vec<JunctionBox>) -> u64 {
    let junction_count = junction_boxes.len();

    let mut connections = Vec::new();
    for i in 0..junction_count {
        for j in i + 1..junction_count {
            connections.push(Connection {
                a: i,
                b: j,
                dist: junction_boxes[i].distance(&junction_boxes[j]),
            });
        }
    }

    connections.sort_unstable_by(|a, b| a.dist.cmp(&b.dist));

    let mut circuits = Circuits::new(junction_count);
    let mut unions = 0;

    for c in connections {
        if circuits.union(c.a, c.b) {
            unions += 1;

            // When everything is connected
            if unions == junction_count - 1 {
                let x1 = junction_boxes[c.a].x as i64;
                let x2 = junction_boxes[c.b].x as i64;
                return (x1 * x2) as u64;
            }
        }
    }

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(solve_part_one(generator(test_input), 10), 40);
    }
}
