// Follow set calculator
pub mod fc {
    pub mod sp {
        /**
         * Get the generative expressions from a string.
         */
        pub fn get_ge_from_str(s: &str) -> Vec<String> {
            let ge = split_string_by_line_break(s);
            ge
        }

        pub(crate) fn split_string_by_line_break(s: &str) -> Vec<String> {
            s.split('\n')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect()
        }
    }

    pub mod gmr {
        mod symbol {
            use std::collections::{HashMap, HashSet};

            struct SymbomManager {
                used_ids: HashSet<u64>,
                next_unused_id: u64,
                id_char_map: HashMap<u64, char>,
            }

            impl SymbomManager {
                pub fn new() -> SymbomManager {
                    let mut used_ids = HashSet::new();
                    used_ids.insert(0);

                    let mut id_char_map = HashMap::new();
                    id_char_map.insert(0, 'ε');

                    Self {
                        used_ids: used_ids,
                        next_unused_id: 1,
                        id_char_map: id_char_map,
                    }
                }

                pub fn query_id_by_char(ch: char) -> u64 {
                    id_
                }
            }
        }

        pub struct GrammarContext {}
    }
}

// Unit tests

mod test {
    use std::process::Output;

    use super::*;

    #[test]
    fn test_split_string_by_line_break_no_space() {
        let input = "expression1\nexpression2\n\n\nexpression3";
        let expected_output = vec![
            "expression1".to_string(),
            "expression2".to_string(),
            "expression3".to_string(),
        ];

        let output = fc::sp::split_string_by_line_break(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_split_string_by_line_break_with_space() {
        let input = " expression1 \n expression2 \n\n\n expression3 ";
        let expected_output = vec![
            "expression1".to_string(),
            "expression2".to_string(),
            "expression3".to_string(),
        ];

        let output = fc::sp::split_string_by_line_break(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_split_string_by_line_break_with_space_in_expressions() {
        let input = "expre  ssion1 \n ex pre ss ion 2\n\n\nexpression    3";
        let expected_output = vec![
            "expre  ssion1".to_string(),
            "ex pre ss ion 2".to_string(),
            "expression    3".to_string(),
        ];

        let output = fc::sp::split_string_by_line_break(input);
        assert_eq!(output, expected_output);
    }
}
