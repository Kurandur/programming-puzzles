use core::fmt;

use reqwest::header;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Splitter,
    Beam,
    BeamStart,
}

impl Cell {
    pub fn to_char(self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::Splitter => '^',
            Cell::Beam => '|',
            Cell::BeamStart => 'S',
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = &'static str;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '.' => Ok(Cell::Empty),
            '^' => Ok(Cell::Splitter),
            '|' => Ok(Cell::Beam),
            'S' => Ok(Cell::BeamStart),
            _ => Err("Unknown cell character"),
        }
    }
}

pub struct TachyonManifold {
    height: usize,
    width: usize,
    grid: Vec<Cell>,
}

impl TachyonManifold {
    pub fn from_str(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

        let mut grid = Vec::with_capacity(width * height);

        for line in lines {
            for ch in line.chars() {
                let cell = Cell::try_from(ch).unwrap();
                grid.push(cell);
            }
        }

        TachyonManifold {
            height,
            width,
            grid,
        }
    }

    pub fn cell_at(&self, row: usize, col: usize) -> Option<Cell> {
        if row >= self.height || col >= self.width {
            None
        } else {
            Some(self.grid[row * self.width + col])
        }
    }

    fn idx(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.height && col < self.width {
            Some(row * self.width + col)
        } else {
            None
        }
    }

    fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        if let Some(i) = self.idx(row, col) {
            self.grid[i] = cell;
        }
    }

    pub fn propagate_beams(&mut self) -> u32 {
        let mut split_count = 0;

        for row in 0..self.height - 1 {
            let mut next_row_beams = vec![false; self.width];

            for col in 0..self.width {
                let Some(cell) = self.cell_at(row, col) else {
                    continue;
                };

                match cell {
                    Cell::Beam | Cell::BeamStart => {
                        let Some(below) = self.cell_at(row + 1, col) else {
                            continue;
                        };

                        match below {
                            Cell::Splitter => {
                                let mut did_split = false;

                                if col > 0 && !next_row_beams[col - 1] {
                                    next_row_beams[col - 1] = true;
                                    did_split = true;
                                }

                                if col + 1 < self.width && !next_row_beams[col + 1] {
                                    next_row_beams[col + 1] = true;
                                    did_split = true;
                                }

                                if did_split {
                                    split_count += 1;
                                }
                            }

                            Cell::Empty => {
                                next_row_beams[col] = true;
                            }

                            _ => {}
                        }
                    }

                    _ => {}
                }
            }

            for col in 0..self.width {
                if next_row_beams[col] {
                    if let Some(Cell::Empty) = self.cell_at(row + 1, col) {
                        self.set_cell(row + 1, col, Cell::Beam);
                    }
                }
            }
        }

        split_count
    }
    pub fn count_timelines(&self) -> u64 {
        let mut current = vec![0u64; self.width];

        for col in 0..self.width {
            if let Some(Cell::BeamStart) = self.cell_at(0, col) {
                current[col] = 1;
                break;
            }
        }

        for row in 0..self.height - 1 {
            let mut next = vec![0u64; self.width];

            for col in 0..self.width {
                let count = current[col];
                if count == 0 {
                    continue;
                }

                match self.cell_at(row + 1, col) {
                    Some(Cell::Empty) => {
                        next[col] += count;
                    }

                    Some(Cell::Splitter) => {
                        if col > 0 {
                            next[col - 1] += count;
                        }
                        if col + 1 < self.width {
                            next[col + 1] += count;
                        }
                    }

                    _ => {}
                }
            }

            current = next;
        }

        current.iter().sum()
    }
}

impl fmt::Display for TachyonManifold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let ch = self.grid[row * self.width + col].to_char();
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn generator(input: &str) -> TachyonManifold {
    TachyonManifold::from_str(input)
}

pub fn part_one(mut tachyon_manifold: TachyonManifold) -> u32 {
    tachyon_manifold.propagate_beams()
}

pub fn part_two(mut tachyon_manifold: TachyonManifold) -> u64 {
    tachyon_manifold.count_timelines()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        assert_eq!(part_one(generator(test_input)), 21);
    }
    #[test]
    fn test_part_two() {
        let test_input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        assert_eq!(part_two(generator(test_input)), 40);
    }
}
