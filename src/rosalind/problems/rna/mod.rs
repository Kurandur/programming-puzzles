pub fn solve(input: &str) -> String {
    input.replace("T", "U")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_data() {
        let sample_data = "GATGGAACTTGACTACGTAAATT";
        assert_eq!(solve(sample_data), "GAUGGAACUUGACUACGUAAAUU")
    }
}
