const DIRECTIONS: [[i8; 2]; 8] = [
    [0, 1],
    [1, 1],
    [1, 0],
    [1, -1],
    [0, -1],
    [-1, -1],
    [-1, 0],
    [-1, 1],
];

const SEARCH_WORD: &str = "XMAS";

pub fn generator(input: &str) -> String {
    input.to_string()
}

fn is_match(
    cells: &[Vec<char>],
    word: &[char],
    mut x: i32,
    mut y: i32,
    direction: [i8; 2],
    word_length: usize,
) -> bool {
    for k in 0..word_length {
        if x < 0 || x >= cells.len() as i32 || y < 0 || y >= cells[0].len() as i32 {
            return false;
        }

        if cells[x as usize][y as usize] != word[k] {
            return false;
        }

        x += direction[0] as i32;
        y += direction[1] as i32;
    }

    true
}

pub fn part_one(input: &str) -> u32 {
    let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let word_chars: Vec<char> = SEARCH_WORD.chars().collect();
    let mut word_count = 0;
    for i in 0..cells.len() {
        for j in 0..cells[i].len() {
            for direction in DIRECTIONS {
                if is_match(
                    &cells,
                    &word_chars,
                    i as i32,
                    j as i32,
                    direction,
                    SEARCH_WORD.len(),
                ) {
                    word_count = word_count + 1;
                }
            }
        }
    }

    word_count
}

const FORWARD_MAS: &str = "MAS";
const BACKWARD_MAS: &str = "SAM";

const DIAGONAL_DIRECTIONS: [[[i8; 2]; 3]; 2] =
    [[[-1, -1], [0, 0], [1, 1]], [[-1, 1], [0, 0], [1, -1]]];

pub fn part_two(input: &str) -> u32 {
    let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    let height = cells.len();
    let width = cells[0].len();

    for i in 0..height {
        for j in 0..width {
            if check_diagonal(&cells, i as usize, j as usize) {
                count += 1;
            }
        }
    }

    count as u32
}

fn check_diagonal(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    let mut chars = String::new();

    for direction_set in DIAGONAL_DIRECTIONS.iter() {
        for direction in direction_set.iter() {
            let nx = x as isize + direction[0] as isize;
            let ny = y as isize + direction[1] as isize;

            if nx >= 0 && ny >= 0 && (nx as usize) < grid.len() && (ny as usize) < grid[0].len() {
                chars.push(grid[nx as usize][ny as usize]);
            } else {
                return false;
            }
        }
        if chars != FORWARD_MAS && chars != BACKWARD_MAS {
            return false;
        }
        chars = String::new();
    }
    return true;
}
#[cfg(test)]
mod tests {}
