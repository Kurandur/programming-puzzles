use std::collections::HashSet;

pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn part_one(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut guard_position = [0, 0];
    let mut guard_direction = [0, 0];

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if ['^', '>', '<', 'v'].contains(&cell) {
                guard_position = [i, j];
                guard_direction = match cell {
                    '^' => [-1, 0],
                    '>' => [0, 1],
                    '<' => [0, -1],
                    'v' => [1, 0],
                    _ => unreachable!(),
                };
            }
        }
    }

    let mut visited_cells: HashSet<[usize; 2]> = HashSet::new();
    visited_cells.insert(guard_position);

    loop {
        let next_position = [
            (guard_position[0] as isize + guard_direction[0]) as usize,
            (guard_position[1] as isize + guard_direction[1]) as usize,
        ];

        if next_position[0] >= grid.len() || next_position[1] >= grid[next_position[0]].len() {
            break;
        }

        if grid[next_position[0]][next_position[1]] == '#' {
            guard_direction = match guard_direction {
                [-1, 0] => [0, 1],
                [0, 1] => [1, 0],
                [1, 0] => [0, -1],
                [0, -1] => [-1, 0],
                _ => unreachable!(),
            };
        } else {
            guard_position = next_position;
            visited_cells.insert(guard_position);
        }
    }

    visited_cells.len() as u32
}

fn get_index(direction: [isize; 2], position: [usize; 2], grid_width: usize) -> usize {
    let dir_index = match direction {
        [-1, 0] => 0,
        [0, 1] => 1,
        [1, 0] => 2,
        [0, -1] => 3,
        _ => unreachable!(),
    };
    position[0] * grid_width * 4 + position[1] * 4 + dir_index
}

fn does_guard_loop(
    grid: &Vec<Vec<char>>,
    start_position: [usize; 2],
    start_direction: [isize; 2],
) -> bool {
    let mut guard_position = start_position;
    let mut direction = start_direction;

    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let mut visited_states = vec![false; grid_height * grid_width * 4];

    visited_states[get_index(direction, guard_position, grid_width)] = true;

    loop {
        let next_position = [
            (guard_position[0] as isize + direction[0]) as usize,
            (guard_position[1] as isize + direction[1]) as usize,
        ];

        if next_position[0] >= grid.len() || next_position[1] >= grid[0].len() {
            return false;
        }

        if grid[next_position[0]][next_position[1]] == '#' {
            direction = match direction {
                [-1, 0] => [0, 1],
                [0, 1] => [1, 0],
                [1, 0] => [0, -1],
                [0, -1] => [-1, 0],
                _ => unreachable!(),
            };
            let index = get_index(direction, guard_position, grid_width);
            if visited_states[index] {
                return true;
            }
            visited_states[index] = true;
        } else {
            guard_position = next_position;
        }
    }
}

pub fn part_two(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut guard_position = [0, 0];
    let mut guard_direction = [0, 0];

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if ['^', '>', '<', 'v'].contains(&cell) {
                guard_position = [i, j];
                guard_direction = match cell {
                    '^' => [-1, 0],
                    '>' => [0, 1],
                    '<' => [0, -1],
                    'v' => [1, 0],
                    _ => unreachable!(),
                };
            }
        }
    }

    let start_position = guard_position;
    let start_direction = guard_direction;

    let mut visited_cells: HashSet<[usize; 2]> = HashSet::new();
    visited_cells.insert(guard_position);

    loop {
        let next_position = [
            (guard_position[0] as isize + guard_direction[0]) as usize,
            (guard_position[1] as isize + guard_direction[1]) as usize,
        ];

        if next_position[0] >= grid.len() || next_position[1] >= grid[next_position[0]].len() {
            break;
        }

        if grid[next_position[0]][next_position[1]] == '#' {
            guard_direction = match guard_direction {
                [-1, 0] => [0, 1],
                [0, 1] => [1, 0],
                [1, 0] => [0, -1],
                [0, -1] => [-1, 0],
                _ => unreachable!(),
            };
        } else {
            guard_position = next_position;
            visited_cells.insert(guard_position);
        }
    }

    guard_position = start_position;
    guard_direction = start_direction;

    let mut loop_count = 0;

    for &[i, j] in visited_cells.iter() {
        if grid[i][j] == '.' && [i, j] != start_position {
            grid[i][j] = '#';

            if does_guard_loop(&grid, guard_position, guard_direction) {
                loop_count += 1;
            }

            grid[i][j] = '.';
        }
    }

    loop_count
}

#[cfg(test)]
mod tests {}
