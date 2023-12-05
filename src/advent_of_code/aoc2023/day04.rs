use std::collections::HashSet;

pub struct Card {
    winning_numbers: HashSet<u16>,
    picked_numbers: HashSet<u16>,
}

pub fn generator(input: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    for line in input.lines() {
        let (_card_part, numbers_part) = line.split_once(":").unwrap();
        let (left_numbers, right_numbers) = numbers_part.split_once("|").unwrap();
        let winning_numbers: HashSet<u16> = left_numbers
            .trim()
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();
        let picked_numbers: HashSet<u16> = right_numbers
            .trim()
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();

        cards.push(Card {
            winning_numbers: winning_numbers,
            picked_numbers: picked_numbers,
        })
    }

    cards
}

pub fn part_one(cards: Vec<Card>) -> u32 {
    let mut sum = 0;
    for card in cards {
        let numbers: HashSet<_> = card
            .winning_numbers
            .intersection(&card.picked_numbers)
            .cloned()
            .collect();
        if numbers.len() != 0 {
            sum += (2 as u32).pow(numbers.len().wrapping_sub(1) as u32)
        }
    }
    sum
}

pub fn part_two(cards: &Vec<Card>) -> u32 {
    let mut card_counts = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let matching_numbers: HashSet<_> = card
            .winning_numbers
            .intersection(&card.picked_numbers)
            .cloned()
            .collect();
        let guard: u32 = if matching_numbers.len() > cards.len() - 1 {
            (cards.len() + 1).try_into().unwrap()
        } else {
            (matching_numbers.len() + i).try_into().unwrap()
        };
        for x in (i + 1)..(guard + 1) as usize {
            println!("x: {}", x);
            card_counts[x] += card_counts[i];
        }
    }
    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}
}
