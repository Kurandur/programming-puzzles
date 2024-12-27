use std::collections::HashMap;

pub fn solve(input: &str) -> String {
    let mut count = HashMap::new();
    for symbol in input.trim().chars() {
        *count.entry(symbol).or_insert(0) += 1;
    }
    format!(
        "{} {} {} {}",
        count[&'A'], count[&'C'], count[&'G'], count[&'T']
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_data() {
        let sample_data = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
        assert_eq!(solve(sample_data), "20 12 17 21")
    }
}
