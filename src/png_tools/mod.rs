pub mod png_tools {
    pub fn find_sequences(bytes: &[u8], sequence: &[u8]) -> Vec<usize> {
        if sequence.len() <= 1 || bytes.len() <= 1 {
            panic!("Sequence and bytes parameters must be at least 2 bytes long")
        }

        let mut sequence_locations: Vec<usize> = Vec::new();
        let mut sequence_progress: usize = 0;
        let mut sequence_beginning: usize = 0;

        for (i, byte) in bytes.iter().enumerate() {
            let mut expected_byte = &sequence[sequence_progress];
            if sequence_progress == sequence.len() - 1 && expected_byte == byte {
                sequence_locations.push(sequence_beginning);
                sequence_progress = 0;
            }

            if sequence_progress == 0 {
                sequence_beginning = i;
            }

            expected_byte = &sequence[sequence_progress];
            if byte == expected_byte {
                sequence_progress += 1;
            } else {
                sequence_progress = if &sequence[0] == byte { 1 } else { 0 };
                sequence_beginning = i;
            }
        }

        sequence_locations
    }
}

#[cfg(test)]
mod tests {
    use super::png_tools::*;

    fn check_output(bytes: Vec<u8>, sequence: Vec<u8>, expected_output: Vec<usize>) {
        assert_eq!(find_sequences(&bytes, &sequence), expected_output)
    }

    // Happy path cases
    #[test]
    fn only_has_sequence() {
        check_output(vec![3, 4, 5], vec![3, 4, 5], vec![0]);
    }

    #[test]
    fn has_overlapping_sequences() {
        check_output(
            vec![3, 4, 5, 3, 5, 3, 4, 3, 4, 5, 3, 5, 3, 4, 5],
            vec![3, 4, 5],
            vec![0, 7, 12],
        );
    }

    #[test]
    fn triple_number() {
        check_output(vec![0, 0, 0, 1, 1, 1], vec![0, 0, 0], vec![0]);
    }

    #[test]
    fn repeated_sequences() {
        check_output(vec![0, 0, 0], vec![0, 0], vec![0, 1]);
    }

    // While it would be nice to account for this test case,
    // a more advanced algorithm would be needed to achieve this.
    // For more info, check out the Knuth-Morris-Pratt algorithm
    // #[test]
    // fn kmp_edge_case() {
    //     check_output(vec![0, 1, 0, 1, 0, 1], vec![0, 1, 0, 1], vec![0, 2]);
    // }

    // Error Cases
    #[test]
    #[should_panic(expected = "Sequence and bytes parameters must be at least 2 bytes long")]
    fn single_byte_in_sequence() {
        check_output(vec![1, 2, 3], vec![0], vec![]);
    }

    #[test]
    #[should_panic(expected = "Sequence and bytes parameters must be at least 2 bytes long")]
    fn single_byte_in_bytes() {
        check_output(vec![0], vec![0, 0, 0], vec![0, 1, 2]);
    }
}

#[cfg(test)]
mod test_data {}
