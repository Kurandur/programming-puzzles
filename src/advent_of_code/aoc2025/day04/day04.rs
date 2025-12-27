use std::{collections::VecDeque, ffi::os_str::Display, fmt};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Empty,
    Paper,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Cell::Empty => '.',
            Cell::Paper => '@',
        };
        write!(f, "{ch}")
    }
}

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            let start = row * self.width;
            let end = start + self.width;

            for cell in &self.cells[start..end] {
                write!(f, "{cell}")?;
            }

            if row + 1 < self.height {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Grid {
    pub fn from_str(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines[0].len();

        let mut cells = Vec::with_capacity(width * height);

        for line in &lines {
            for ch in line.chars() {
                cells.push(match ch {
                    '@' => Cell::Paper,
                    '.' => Cell::Empty,
                    _ => unreachable!(),
                });
            }
        }

        let grid = Self {
            width,
            height,
            cells,
        };
        grid
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Cell> {
        if row < self.height && col < self.width {
            Some(self.cells[self.idx(row, col)])
        } else {
            None
        }
    }

    pub fn accessable_cells_count(&self) -> u32 {
        const DIRS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut count = 0;

        for row in 0..self.height {
            for col in 0..self.width {
                if self.get(row, col) != Some(Cell::Paper) {
                    continue;
                }

                let mut neighbors = 0;

                for (dr, dc) in DIRS {
                    let nr = row as isize + dr;
                    let nc = col as isize + dc;

                    if self.get(nr as usize, nc as usize) == Some(Cell::Paper) {
                        neighbors += 1;

                        if neighbors >= 4 {
                            break;
                        }
                    }
                }

                if neighbors < 4 {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn remove_accessible_papers(&mut self) -> usize {
        const DIRS_8: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut removed_count = 0;
        let mut queue = VecDeque::new();
        let mut queued = vec![false; self.cells.len()];

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = row * self.width + col;

                if self.cells[idx] == Cell::Paper && self.paper_neighbors(row, col) < 4 {
                    queue.push_back(idx);
                    queued[idx] = true;
                }
            }
        }

        while let Some(idx) = queue.pop_front() {
            if self.cells[idx] != Cell::Paper {
                continue;
            }

            self.cells[idx] = Cell::Empty;
            removed_count += 1;

            let row = idx / self.width;
            let col = idx % self.width;

            for (dr, dc) in DIRS_8 {
                let nr = row as isize + dr;
                let nc = col as isize + dc;

                if nr < 0 || nc < 0 {
                    continue;
                }

                let nr = nr as usize;
                let nc = nc as usize;

                if nr >= self.height || nc >= self.width {
                    continue;
                }

                let nidx = nr * self.width + nc;

                if self.cells[nidx] == Cell::Paper
                    && !queued[nidx]
                    && self.paper_neighbors(nr, nc) < 4
                {
                    queue.push_back(nidx);
                    queued[nidx] = true;
                }
            }
        }

        removed_count
    }

    fn paper_neighbors(&self, row: usize, col: usize) -> usize {
        const DIRS_8: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        DIRS_8
            .iter()
            .filter_map(|(dr, dc)| {
                let nr = row as isize + dr;
                let nc = col as isize + dc;
                if nr >= 0 && nc >= 0 {
                    self.get(nr as usize, nc as usize)
                } else {
                    None
                }
            })
            .filter(|&cell| cell == Cell::Paper)
            .count()
    }
}

pub fn generator(input: &str) -> Grid {
    Grid::from_str(input)
}

pub fn part_one(grid: Grid) -> u32 {
    grid.accessable_cells_count()
}

pub fn part_two(mut grid: Grid) -> u32 {
    grid.remove_accessible_papers() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(part_one(generator(test_input)), 13);
    }
}
