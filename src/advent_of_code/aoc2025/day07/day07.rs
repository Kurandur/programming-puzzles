use core::fmt;

#[derive(Debug, Clone, Copy)]
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
    }
}
