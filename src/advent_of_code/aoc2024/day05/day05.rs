use std::collections::HashMap;

pub fn generator(input: &str) -> String {
    input.to_string()
}

fn validate_update(update: &[u16], rules: &HashMap<u16, Vec<u16>>) -> bool {
    for (i, &num) in update.iter().enumerate() {
        if let Some(rule_values) = rules.get(&num) {
            for &rule_val in rule_values {
                if update[..i].contains(&rule_val) {
                    return false;
                }
            }
        }
    }
    true
}

fn reorder_update(update: &[u16], rules: &HashMap<u16, Vec<u16>>) -> Vec<u16> {
    let mut reordered = update.to_vec();
    let mut i = 0;

    while i < reordered.len() {
        let num = reordered[i];

        if let Some(rule_values) = rules.get(&num) {
            for &rule_val in rule_values {
                if let Some(pos) = reordered[..i].iter().position(|&x| x == rule_val) {
                    let offending = reordered.remove(pos);
                    let insert_index = if pos < i { i } else { i + 1 };
                    reordered.insert(insert_index, offending);
                    if pos < i {
                        i -= 1;
                    }
                }
            }
        }

        i += 1;
    }

    reordered
}

pub fn part_one(input: &str) -> u32 {
    let (rules_string, updates_string) = {
        let mut parts = input.split("\n\n");
        let rules_string = parts.next().unwrap_or_default();
        let updates_string = parts.next().unwrap_or_default();
        (rules_string, updates_string)
    };

    let mut rules: HashMap<u16, Vec<u16>> = HashMap::new();

    for line in rules_string.lines() {
        let mut parts = line.split('|');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            let key: u16 = key.parse().unwrap_or_default();
            let value: u16 = value.parse().unwrap_or_default();
            rules.entry(key).or_insert_with(Vec::new).push(value);
        }
    }

    let mut updates: Vec<Vec<u16>> = Vec::new();

    for line in updates_string.lines() {
        let mut update: Vec<u16> = Vec::new();
        for number in line.split(",") {
            update.push(number.parse().unwrap_or_default());
        }
        updates.push(update)
    }

    let mut result: u32 = 0;

    for update in updates {
        if validate_update(&update, &rules) {
            result += update[(update.len() - 1) / 2] as u32;
        }
    }

    result
}

pub fn part_two(input: &str) -> u32 {
    let (rules_string, updates_string) = {
        let mut parts = input.split("\n\n");
        let rules_string = parts.next().unwrap_or_default();
        let updates_string = parts.next().unwrap_or_default();
        (rules_string, updates_string)
    };

    let mut rules: HashMap<u16, Vec<u16>> = HashMap::new();

    for line in rules_string.lines() {
        let mut parts = line.split('|');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            let key: u16 = key.parse().unwrap_or_default();
            let value: u16 = value.parse().unwrap_or_default();
            rules.entry(key).or_insert_with(Vec::new).push(value);
        }
    }

    let mut updates: Vec<Vec<u16>> = Vec::new();

    for line in updates_string.lines() {
        let mut update: Vec<u16> = Vec::new();
        for number in line.split(",") {
            update.push(number.parse().unwrap_or_default());
        }
        updates.push(update)
    }

    let mut result: u32 = 0;

    for update in updates {
        if !validate_update(&update, &rules) {
            let mut reordered_update = update.to_vec();

            while !validate_update(&reordered_update, &rules) {
                reordered_update = reorder_update(&reordered_update, &rules);
            }
            result += reordered_update[(reordered_update.len() - 1) / 2] as u32;
        }
    }

    result
}

#[cfg(test)]
mod tests {}
