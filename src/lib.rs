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
        pub trait Symbolizable {}

        impl Symbolizable for char {}
        impl Symbolizable for Vec<char> {}
        impl Symbolizable for String {}

        mod symbol {
            use crate::fc::gmr::Symbolizable;
            use std::collections::{HashMap, HashSet};

            struct SymbolManager<T>
            where
                T: Symbolizable + Eq + PartialEq + PartialOrd + Ord + Clone,
            {
                used_ids: HashSet<u64>,
                next_unused_id: u64,
                id_symbol_map: HashMap<u64, T>,
            }

            impl<T> SymbolManager<T>
            where
                T: Symbolizable + Eq + PartialEq + PartialOrd + Ord + Clone,
            {
                pub fn query_id_by_symbol(&self, symbol: &T) -> Option<u64> {
                    self.id_symbol_map
                        .iter()
                        .find(|&(_, c)| c.eq(symbol))
                        .map(|(&id, _)| id)
                }

                pub fn get_symbol_by_id(&self, id: u64) -> Option<T> {
                    self.id_symbol_map.get(&id).cloned()
                }

                pub fn register_symbol(&mut self, symbol: T) -> u64 {
                    if let Some(id) = self.query_id_by_symbol(&symbol) {
                        return id;
                    }

                    let new_id = loop {
                        if self.next_unused_id == 0 {
                            panic!("No more unused IDs available.");
                        }

                        let id = self.next_unused_id;

                        if !self.used_ids.contains(&id) {
                            break id;
                        }

                        self.next_unused_id += 1;
                    };

                    self.used_ids.insert(new_id);
                    self.id_symbol_map.insert(new_id, symbol);

                    new_id
                }

                pub fn unregister_symbol(&mut self, symbol: &T) -> Option<u64> {
                    if let Some(id) = self.query_id_by_symbol(symbol) {
                        self.used_ids.remove(&id);
                        self.id_symbol_map.remove(&id);
                        return Some(id);
                    }
                    None
                }
            }

            impl SymbolManager<char> {
                fn new() -> Self {
                    let mut used_ids = HashSet::new();
                    used_ids.insert(0);

                    let mut id_symbol_map = HashMap::new();
                    id_symbol_map.insert(0, 'ε');

                    Self {
                        used_ids: used_ids,
                        next_unused_id: 1,
                        id_symbol_map: id_symbol_map,
                    }
                }
            }

            impl SymbolManager<Vec<char>> {
                fn new() -> Self {
                    let mut used_ids = HashSet::new();
                    used_ids.insert(0);

                    let mut id_symbol_map = HashMap::new();
                    id_symbol_map.insert(0, vec!['ε']);

                    Self {
                        used_ids: used_ids,
                        next_unused_id: 1,
                        id_symbol_map: id_symbol_map,
                    }
                }
            }

            impl SymbolManager<String> {
                fn new() -> Self {
                    let mut used_ids = HashSet::new();
                    used_ids.insert(0);

                    let mut id_symbol_map = HashMap::new();
                    id_symbol_map.insert(0, String::from("ε"));

                    Self {
                        used_ids: used_ids,
                        next_unused_id: 1,
                        id_symbol_map: id_symbol_map,
                    }
                }
            }
        }

        pub struct GrammarContext {}
    }
}

// Unit tests

mod test {
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
