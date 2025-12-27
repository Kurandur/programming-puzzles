use std::{collections::HashSet, ops::RangeInclusive};

pub struct IngredientDatabase {
    ranges: Vec<RangeInclusive<u64>>,
    ids: Vec<u64>,
}

impl IngredientDatabase {
    pub fn from_str(input: &str) -> Self {
        let (ranges_part, ids_part) = input.split_once("\n\n").expect("No blank line");

        let ranges = ranges_part
            .lines()
            .map(|line| {
                let (start, end) = line.split_once("-").expect("No dash found");
                let start = start.parse::<u64>().unwrap();
                let end = end.parse::<u64>().unwrap();
                start..=end
            })
            .collect();

        let ids = ids_part
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect();

        IngredientDatabase { ranges, ids }
    }

    pub fn fresh_ingredients(&self) -> u32 {
        self.ids
            .iter()
            .filter(|id| {
                for id_range in &self.ranges {
                    if id_range.contains(id) {
                        return true;
                    }
                }
                return false;
            })
            .count() as u32
    }

    pub fn fresh_ingredient_range_count(&self) -> u64 {
        if self.ranges.is_empty() {
            return 0;
        }

        let mut ranges: Vec<_> = self.ranges.clone();
        ranges.sort_by_key(|r| *r.start());
        let mut total: u64 = 0;

        let mut current_start = *ranges[0].start();
        let mut current_end = *ranges[0].end();

        for range in ranges.iter().skip(1) {
            let start = *range.start();
            let end = *range.end();

            if start <= current_end + 1 {
                current_end = current_end.max(end);
            } else {
                total += current_end - current_start + 1;
                current_start = start;
                current_end = end;
            }
        }

        total += current_end - current_start + 1;

        total as u64
    }
}

pub fn generator(input: &str) -> IngredientDatabase {
    IngredientDatabase::from_str(input)
}

pub fn part_one(ingredient_database: &IngredientDatabase) -> u32 {
    ingredient_database.fresh_ingredients()
}

pub fn part_two(ingredient_database: &IngredientDatabase) -> u64 {
    ingredient_database.fresh_ingredient_range_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(part_one(&generator(test_input)), 3);
    }

    #[test]
    fn test_part_two() {
        let test_input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(part_two(&generator(test_input)), 14);
    }
}
