use std::{collections::HashSet, hash::Hash, u32};

pub fn generator(input: &str) -> String {
    input.to_string()
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Cell {
    x: i32,
    y: i32,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }
    pub fn neighors(&self) -> [Self; 4] {
        return [
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y - 1),
            Self::new(self.x, self.y + 1),
        ];
    }
}

struct Region {
    cells: HashSet<Cell>,
}

impl Region {
    fn area(&self) -> u32 {
        self.cells.len() as u32
    }
    fn perimter(&self) -> u32 {
        self.cells
            .iter()
            .map(|c| {
                c.neighors()
                    .iter()
                    .map(|n| {
                        if !self.cells.contains(n) {
                            return 1;
                        }
                        return 0;
                    })
                    .sum::<u32>()
            })
            .sum()
    }
    fn price(&self) -> u32 {
        return self.area() * self.perimter();
    }
    fn sides(&self) -> u32 {
        return self
            .cells
            .iter()
            .map(|c| {
                let mut count = 0;

                for x in [-1, 1].iter() {
                    for y in [-1, 1].iter() {
                        let nx = self.cells.get(&Cell { x: c.x + x, y: c.y });
                        let ny = self.cells.get(&Cell { x: c.x, y: c.y + y });
                        if !nx.is_some() && !ny.is_some() {
                            count += 1;
                        }
                        let n = self.cells.get(&Cell {
                            x: x + c.x,
                            y: y + c.y,
                        });
                        if nx.is_some() && ny.is_some() && !n.is_some() {
                            count += 1;
                        }
                    }
                }
                count
            })
            .sum();
    }
}

fn discover_regions(grid: Vec<Vec<char>>) -> Vec<Region> {
    let mut regions: Vec<Region> = vec![];
    let mut visited_cells: HashSet<Cell> = HashSet::new();

    for (x, row) in grid.iter().enumerate() {
        for (y, cell_value) in row.iter().enumerate() {
            let cell = Cell::new(x as i32, y as i32);
            if visited_cells.contains(&cell) {
                continue;
            }
            if let Some(region) = flood_fill(&grid, cell, cell_value, &mut visited_cells) {
                regions.push(region);
            }
        }
    }

    regions
}

/**
 * Iterative flood fill
 */
fn flood_fill(
    grid: &Vec<Vec<char>>,
    start_cell: Cell,
    plot: &char,
    visited_cells: &mut HashSet<Cell>,
) -> Option<Region> {
    let grid_rows = grid.len();
    let grid_width = grid[0].len();
    let mut region_cells: HashSet<Cell> = HashSet::new();

    if visited_cells.contains(&start_cell) {
        return None;
    }

    let mut cell_stack = vec![start_cell];

    while let Some(cell) = cell_stack.pop() {
        if !(cell.x >= 0
            && cell.x < (grid_width as i32)
            && cell.y >= 0
            && cell.y < (grid_rows as i32))
        {
            continue;
        }

        if grid[cell.x as usize][cell.y as usize] != *plot {
            continue;
        }

        if visited_cells.contains(&cell) {
            continue;
        }
        visited_cells.insert(cell);
        region_cells.insert(cell);
        for neighbor in cell.neighors() {
            cell_stack.push(neighbor);
        }
    }

    Some(Region {
        cells: region_cells,
    })
}

pub fn part_one(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let regions = discover_regions(grid);
    regions.iter().map(|r| r.price()).sum::<u32>() as u32
}

pub fn part_two(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let regions = discover_regions(grid);

    regions.iter().map(|r| r.area() * r.sides()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}
}
