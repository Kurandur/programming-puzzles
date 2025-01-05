use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq, PartialEq, Clone)]
struct CellState {
    cost: u32,
    position: (usize, usize),
    direction: usize,
}

impl Ord for CellState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.cost).cmp(&Reverse(other.cost))
    }
}

impl PartialOrd for CellState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn find_lowest_score(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();
    const DIRECTION_COST: u32 = 5;

    heap.push(CellState {
        cost: 0,
        position: start,
        direction: 0,
    });

    while let Some(CellState {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        if position == end {
            return cost;
        }

        if let Some(&prev_cost) = visited.get(&(position, direction)) {
            if cost >= prev_cost {
                continue;
            }
        }
        visited.insert((position, direction), cost);

        for (new_direction, &(dx, dy)) in directions.iter().enumerate() {
            let new_position = (
                (position.0 as isize + dx) as usize,
                (position.1 as isize + dy) as usize,
            );

            if let Some(&cell) = grid
                .get(new_position.0)
                .and_then(|row| row.get(new_position.1))
            {
                if cell == '#' {
                    continue;
                }

                let additional_cost = if new_direction == direction {
                    1
                } else {
                    DIRECTION_COST
                };

                heap.push(CellState {
                    cost: cost + additional_cost,
                    position: new_position,
                    direction: new_direction,
                });
            }
        }
    }
    u32::MAX
}

pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn part_one(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = (i, j);
            } else if cell == 'E' {
                end = (i, j);
            }
        }
    }

    let score = find_lowest_score(&grid, start, end);

    score
}

pub fn part_two(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = (i, j);
            } else if cell == 'E' {
                end = (i, j);
            }
        }
    }

    let empty_cells: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, &cell)| {
                    if cell == '.' || cell == 'S' || cell == 'E' {
                        Some((i, j))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let lowest_score = find_lowest_score(&grid, start, end);
    let result = empty_cells
        .into_iter()
        .filter(|empty_cell| {
            println!("Checking out: {:?}", empty_cell);
            let score = find_lowest_score(&grid, start, *empty_cell);
            if lowest_score < score {
                return false;
            }
            let actual_score = find_lowest_score(&grid, *empty_cell, end);
            return score + actual_score == lowest_score;
        })
        .count();
    result as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}
}
