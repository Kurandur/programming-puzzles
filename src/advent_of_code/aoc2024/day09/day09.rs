pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn part_one(input: &str) -> u64 {
    let mut block_chars = Vec::new();

    for (i, number_char) in input.chars().enumerate() {
        let number = number_char.to_digit(10).unwrap();
        if i % 2 == 0 {
            block_chars.extend(std::iter::repeat((i / 2).to_string()).take(number as usize));
        } else {
            block_chars.extend(std::iter::repeat('.'.to_string()).take(number as usize));
        }
    }

    let mut end = block_chars.len() - 1;
    for i in 0..block_chars.len() {
        if block_chars[i] == "." {
            while end > i && block_chars[end] == "." {
                end -= 1;
            }
            if end > i {
                block_chars[i] = block_chars[end].clone();
                block_chars[end] = ".".to_string();
                end -= 1;
            }
        }
    }

    let mut checksum: u64 = 0;

    for (i, number_char) in block_chars.iter().enumerate() {
        if number_char.as_str() != "." {
            let number = number_char.parse::<u64>().unwrap();
            let x = i as u64 * number;
            checksum += x;
        }
    }

    checksum
}

pub fn part_two(input: &str) -> u64 {
    let mut block_chars = Vec::new();

    for (i, number_char) in input.chars().enumerate() {
        let number = number_char.to_digit(10).unwrap();
        if i % 2 == 0 {
            block_chars.extend(std::iter::repeat((i / 2).to_string()).take(number as usize));
        } else {
            block_chars.extend(std::iter::repeat('.'.to_string()).take(number as usize));
        }
    }

    let mut index = block_chars.len();

    while index > 0 {
        index -= 1;
        let mut block_start = index;

        if block_chars[index] != "." {
            let number = &block_chars[index];
            while block_start > 0 && block_chars[block_start - 1] == *number {
                block_start -= 1;
            }
            let block_length = index - block_start;
            let potential_space = find_free_space(&block_chars, block_length, block_start);
            if let Some(potential_space) = potential_space {
                move_block(
                    &mut block_chars,
                    potential_space,
                    block_start,
                    block_length + 1,
                );
            }

            index -= block_length;
        }
    }

    let mut checksum: u64 = 0;

    for (i, number_char) in block_chars.iter().enumerate() {
        if number_char.as_str() != "." {
            let number = number_char.parse::<u64>().unwrap();
            let x = i as u64 * number;
            checksum += x;
        }
    }

    checksum
}

fn find_free_space(
    block_chars: &Vec<String>,
    search_length: usize,
    search_start: usize,
) -> Option<usize> {
    let mut search_index = 0;
    while search_index < search_start {
        if block_chars[search_index] == "." {
            let potential_block_start = search_index;
            let mut potential_block_length = 1;
            while (potential_block_start + potential_block_length + 1) <= block_chars.len()
                && block_chars[potential_block_start + potential_block_length] == "."
            {
                potential_block_length += 1;
            }
            if potential_block_length >= search_length + 1 {
                return Some(potential_block_start);
            }
        }
        search_index += 1;
    }
    None
}

fn move_block(
    block_chars: &mut Vec<String>,
    target_index: usize,
    source_index: usize,
    source_length: usize,
) {
    for index in 0..source_length {
        block_chars[target_index + index] = block_chars[source_index + index].clone();
        block_chars[source_index + index] = ".".to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}
}
