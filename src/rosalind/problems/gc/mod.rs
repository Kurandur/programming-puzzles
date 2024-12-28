use std::collections::HashMap;

fn calculate_gc_content(dna: &str) -> f64 {
    let gc_count = dna.chars().filter(|&c| c == 'C' || c == 'G').count();
    let total_count = dna.len();
    (gc_count as f64 / total_count as f64) * 100.0
}

pub fn solve(input: &str) -> String {
    let mut fasta_records: HashMap<String, String> = HashMap::new();
    let mut current_id = String::new();

    for line in input.lines() {
        if line.starts_with('>') {
            // New ID found, extract it
            current_id = line[1..].to_string();
        } else {
            // Append DNA sequence to the current ID
            fasta_records
                .entry(current_id.clone())
                .and_modify(|seq| seq.push_str(line))
                .or_insert_with(|| line.to_string());
        }
    }
    let (id, percentage) = fasta_records
        .iter()
        .map(|(id, dna)| (id.clone(), calculate_gc_content(dna)))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();
    format!("{}\n{:.6}", id, percentage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_data() {
        let sample_data = ">Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT";
        assert_eq!(
            solve(sample_data),
            "Rosalind_0808
60.919540"
        )
    }
}
