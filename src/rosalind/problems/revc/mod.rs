pub fn solve(input: &str) -> String {
    input
        .chars()
        .rev()
        .map(|c| match c {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => c,
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_data() {
        let sample_data = "AAAACCCGGT";
        assert_eq!(solve(sample_data), "ACCGGGTTTT")
    }
}
