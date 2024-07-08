pub fn hamming_distance(string_a: &str, string_b: &str) -> usize {
    if string_a.len() != string_b.len() {
        panic!("Strings must be of equal length");
    }

    string_a
        .chars()
        .zip(string_b.chars())
        .filter(|(a, b)| a != b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_case {
        ($($fn_name:ident: $input:expr,)*) => {
            $(
                #[test]
                fn $fn_name() {
                    let (string_a, string_b, expected) = $input;
                    assert_eq!(hamming_distance(string_a, string_b), expected);
                }
            )*
        };
        ($($fn_name:ident, $input:expr, $expected:expr,)*) => {
            $(
                #[test]
                fn $fn_name() {
                    let (string_a, string_b) = $input;
                    assert_eq!(hamming_distance(string_a, string_b), $expected);
                }
            )*
        };
    }

    test_case! {
        empty_inputs: ("", "", 0),
        length_1_inputs: ("a", "a", 0),
        same_strings: ("rust", "rust", 0),
        regular_input_0: ("karolin", "kathrin", 3),
        regular_input_1: ("kathrin", "kerstin", 4),
        regular_input_2: ("00000", "11111", 5),
        different_case: ("x", "X", 1),
    }

    test_case! {
        test_hamming_distance_1, ("1011101", "1001001"), 2,
        test_hamming_distance_2, ("1011101", "1001001"), 2,
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance("1011101", "1001001"), 2);
        assert_eq!(hamming_distance("1011101", "1011101"), 0);
        assert_eq!(hamming_distance("1011101", "1011100"), 1);
    }

    #[test]
    #[should_panic]
    fn test_hamming_distance_panic() {
        hamming_distance("1011101", "101110");
    }
}
