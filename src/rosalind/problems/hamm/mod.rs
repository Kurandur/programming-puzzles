pub fn solve(input: &str) -> String {
    let dna_strings = input.lines().collect::<Vec<&str>>();
    dna_strings[0]
        .chars()
        .zip(dna_strings[1].chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_data() {
        let sample_data = "GAGCCTACTAACGGGAT
CATCGTAATGACGGCCT";
        assert_eq!(solve(sample_data), "7")
    }
}
