pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

pub fn part_one(grid: Vec<Vec<u8>>) -> u32 {
    let mut total_count = 0;

    for (x, row) in grid.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            if cell == 0 {
                // Count the number of 9s reachable from this zero
                total_count += traverse(&grid, x, y);
            }
        }
    }

    total_count as u32
}

fn traverse(grid: &Vec<Vec<u8>>, start_x: usize, start_y: usize) -> u32 {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut count = 0;

    fn dfs(grid: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize, count: &mut u32) {
        if grid[x][y] == 9 {
            *count += 1;
        }
        visited[x][y] = true;

        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0
                && ny >= 0
                && (nx as usize) < grid.len()
                && (ny as usize) < grid[0].len()
                && !visited[nx as usize][ny as usize]
                && grid[nx as usize][ny as usize] == grid[x][y] + 1
            {
                dfs(grid, visited, nx as usize, ny as usize, count);
            }
        }
    }

    if grid[start_x][start_y] == 0 {
        dfs(grid, &mut visited, start_x, start_y, &mut count);
    }

    count
}

pub fn part_two(grid: Vec<Vec<u8>>) -> u32 {
    let mut total_rating = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    // Recursive DFS to count paths from the current cell
    fn dfs(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
        // Base case: if the current cell is 9, it's a valid path
        if grid[x][y] == 9 {
            return 1;
        }

        let mut path_count = 0;
        // Directions: right, down, left, up
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        for (dx, dy) in directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0
                && ny >= 0
                && (nx as usize) < grid.len()
                && (ny as usize) < grid[0].len()
                && grid[nx as usize][ny as usize] == grid[x][y] + 1
            {
                path_count += dfs(grid, nx as usize, ny as usize);
            }
        }

        path_count
    }

    // Find all trailheads and compute their ratings
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 0 {
                total_rating += dfs(&grid, i, j);
            }
        }
    }

    total_rating
}

#[cfg(test)]
mod tests {}
