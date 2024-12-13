use std::collections::HashSet;

pub fn generator(input: &str) -> String {
    input.to_string()
}

const LEFT_POSITIONS: [[i8; 2]; 3] = [[-1, -1], [0, -1], [1, -1]];

const RIGHT_POSITIONS: [[i8; 2]; 3] = [[-1, 1], [0, 1], [1, 1]];

pub fn is_part_number(
    cells: &Vec<Vec<char>>,
    number_length: usize,
    number_start: [usize; 2],
) -> bool {
    for position in LEFT_POSITIONS {
        let row = number_start[0].wrapping_add(position[0] as usize);
        let col = number_start[1].wrapping_add(position[1] as usize);
        if row < cells.len() && col < cells[0].len() {
            let value = cells[row][col];
            if value != '.' && !value.is_digit(10) {
                return true;
            }
        }
    }

    for position in RIGHT_POSITIONS {
        let row = number_start[0].wrapping_add(position[0] as usize);
        let col = number_start[1]
            .wrapping_add(position[1] as usize)
            .wrapping_add(number_length - 1);

        if row < cells.len() && col < cells[0].len() {
            let value = cells[row][col];
            if value != '.' && !value.is_digit(10) {
                return true;
            }
        }
    }

    for x in number_start[1]..=(number_start[1] + number_length - 1) {
        let row = number_start[0].wrapping_sub(1);
        // row above
        if row < cells.len() && x < cells[0].len() {
            let value = cells[row][x];
            if value != '.' && !value.is_digit(10) {
                return true;
            }
        }
        // row below
        let row = number_start[0].wrapping_add(1);
        if row < cells.len() && x < cells[0].len() {
            let value = cells[row][x];
            if value != '.' && !value.is_digit(10) {
                return true;
            }
        }
    }

    return false;
}

pub fn part_one(input: &str) -> u32 {
    let mut sum = 0;
    let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut i = 0;
    let mut j = 0;
    while i < cells.len() {
        while j < cells[i].len() {
            if cells[i][j].is_digit(10) {
                let mut number = String::new();
                while j < cells[i].len() && cells[i][j].is_digit(10) {
                    number.push(cells[i][j]);
                    j += 1;
                }
                if is_part_number(&cells, number.len(), [i, j.wrapping_sub(number.len())]) {
                    sum += number.parse::<u32>().unwrap()
                } else {
                }
            } else {
                j += 1;
            }
        }
        j = 0;
        i += 1;
    }

    sum
}

pub fn gear_ratio(cells: &Vec<Vec<char>>, position: [usize; 2]) -> Option<u32> {
    let mut neighbours: HashSet<u32> = HashSet::new();
    for i in 0..=2 {
        for j in 0..=2 {
            let row = i + position[0] - 1;
            let col = j + position[1] - 1;
            if cells[row][col].is_digit(10) {
                let mut number = String::new();
                let mut current_col = col;
                while current_col < cells[row].len() && cells[row][current_col].is_digit(10) {
                    current_col = current_col.wrapping_sub(1)
                }
                current_col = current_col.wrapping_add(1);
                while current_col < cells[row].len() && cells[row][current_col].is_digit(10) {
                    number.push(cells[row][current_col]);
                    current_col = current_col.wrapping_add(1);
                }
                neighbours.insert(number.parse::<u32>().unwrap());
            }
        }
    }
    if neighbours.len() == 2 {
        return Some(neighbours.iter().cloned().product());
    }
    return None;
}

pub fn part_two(input: &str) -> u32 {
    let mut sum = 0;
    let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut i = 0;
    let mut j = 0;

    while i < cells.len() {
        while j < cells[i].len() {
            if cells[i][j] == '*' {
                if let Some(ratio) = gear_ratio(&cells, [i, j]) {
                    sum += ratio;
                }
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}
}
