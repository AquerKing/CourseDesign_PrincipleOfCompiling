// Follow set calculator
pub mod fc {
    /**
     * String processing module for handling generative expressions and splitting strings by line breaks.
     */
    pub mod sp {
        /**
         * Get the generative expressions from a string.
         */
        pub fn get_ge_from_str(s: &str) -> Vec<String> {
            let ge = split_string_by_line_break(s);
            ge
        }

        /**
         * Split a string by line breaks and return a vector of non-empty trimmed strings.
         */
        pub(crate) fn split_string_by_line_break(s: &str) -> Vec<String> {
            s.split('\n')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect()
        }
    }

    /**
     * Grammar management module for handling symbols and their unique IDs.
     */
    pub mod gmr {
        use std::hash::Hash;

        /**
         * A trait for types that can be symbolized and printed.
         */
        pub trait Symbolizable {}

        impl Symbolizable for char {}
        impl Symbolizable for String {}

        /**
         * Symbol management module for assigning unique IDs to symbols and allowing querying by ID or symbol.
         */
        pub(super) mod symbol {
            use crate::fc::gmr::Symbolizable;
            use std::{
                collections::{HashMap, HashSet},
                hash::Hash,
            };

            pub type SymbolId = u64;

            /**
             * A manager for symbols that assigns unique IDs to symbols and allows querying by ID or symbol.
             */
            pub struct SymbolTable<T>
            where
                T: Symbolizable + Eq + PartialEq + PartialOrd + Ord + Clone + Hash,
            {
                used_ids: HashSet<SymbolId>,
                next_unused_id: SymbolId,
                id_symbol_map: HashMap<SymbolId, T>,
                symbol_id_map: HashMap<T, SymbolId>,
            }

            impl<T> SymbolTable<T>
            where
                T: Symbolizable + Eq + PartialEq + PartialOrd + Ord + Clone + Hash,
            {
                /**
                 * Query the ID associated with a given symbol.
                 */
                pub fn query_id_by_symbol(&self, symbol: &T) -> Option<SymbolId> {
                    self.symbol_id_map.get(symbol).cloned()
                }

                /**
                 * Query the symbol associated with a given ID.
                 */
                pub fn get_symbol_by_id(&self, id: SymbolId) -> Option<T> {
                    self.id_symbol_map.get(&id).cloned()
                }

                /**
                 * Register a new symbol and return its unique ID.
                 *  If the symbol is already registered, return its existing ID.
                 */
                pub fn register_symbol(&mut self, symbol: &T) -> SymbolId {
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
                    self.id_symbol_map.insert(new_id, symbol.clone());
                    self.symbol_id_map.insert(symbol.clone(), new_id);

                    new_id
                }

                /**
                 * Unregister a symbol and return its ID.
                 *  If the symbol is not registered, return None.
                 */
                pub fn unregister_symbol(&mut self, symbol: &T) -> Option<SymbolId> {
                    if let Some(id) = self.query_id_by_symbol(symbol) {
                        self.used_ids.remove(&id);
                        self.id_symbol_map.remove(&id);
                        return Some(id);
                    }
                    None
                }
            }

            impl SymbolTable<char> {
                fn new() -> Self {
                    let mut used_ids = HashSet::new();
                    used_ids.insert(0);

                    let mut id_symbol_map = HashMap::new();
                    id_symbol_map.insert(0, 'ε');

                    let mut symbol_id_map = HashMap::new();
                    symbol_id_map.insert('ε', 0);

                    Self {
                        used_ids: used_ids,
                        next_unused_id: 1,
                        id_symbol_map: id_symbol_map,
                        symbol_id_map: symbol_id_map,
                    }
                }
            }

            impl SymbolTable<String> {
                fn new() -> Self {
                    let mut used_ids = HashSet::new();
                    used_ids.insert(0);

                    let mut id_symbol_map = HashMap::new();
                    id_symbol_map.insert(0, String::from("ε"));

                    let mut symbol_id_map = HashMap::new();
                    symbol_id_map.insert(String::from("ε"), 0);

                    Self {
                        used_ids: used_ids,
                        next_unused_id: 1,
                        id_symbol_map: id_symbol_map,
                        symbol_id_map: symbol_id_map,
                    }
                }
            }
        }

        /**
         * Generative expressions module for handling grammar context and symbol management.
         */
        #[allow(unused_imports)]
        pub mod ge {
            use super::symbol::*;
            use super::*;

            pub struct GenerativeExpressionManager {}

            pub struct GenerativeExpression {
                pub left_side: SymbolId,
                pub right_side: Vec<SymbolId>,
            }
        }

        pub struct GrammarContext<T>
        where
            T: Symbolizable + Eq + PartialEq + PartialOrd + Ord + Clone + Hash,
        {
            pub symbol_manager: symbol::SymbolTable<T>,
            pub expressions: Vec<ge::GenerativeExpression>,
        }
    }
}

// Unit tests
#[allow(unused_imports)]
mod test {
    use crate::*;

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
