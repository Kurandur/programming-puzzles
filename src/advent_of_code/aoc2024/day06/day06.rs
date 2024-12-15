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

fn does_guard_loop(
    grid: &Vec<Vec<char>>,
    start_position: [usize; 2],
    start_direction: [isize; 2],
) -> bool {
    let mut guard_position = start_position;
    let mut direction = start_direction;
    let mut visited_states: HashSet<([usize; 2], [isize; 2])> = HashSet::new();

    visited_states.insert((guard_position, direction));

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
        } else {
            guard_position = next_position;
        }

        if !visited_states.insert((guard_position, direction)) {
            return true;
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
    let mut loop_count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '.' && [i, j] != start_position {
                grid[i][j] = '#';
                if does_guard_loop(&grid, guard_position, guard_direction) {
                    loop_count += 1;
                }

                grid[i][j] = '.';
            }
        }
    }
    loop_count
}

#[cfg(test)]
mod tests {}
