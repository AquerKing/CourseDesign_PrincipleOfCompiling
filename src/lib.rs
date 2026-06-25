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
        use std::{hash::Hash, vec::Vec};

        use crate::fc::gmr::ge::GenerativeExpression;

        pub fn parse_generative_expressions(
            expressions: &Vec<String>,
            symbol_manager: &mut self::symbol::SymbolTable<char>,
        ) -> Vec<GenerativeExpression> {
            let mut generative_expressions = Vec::new();

            for expr_str in expressions {
                let two_parts: Vec<&str> = expr_str.split("->").collect();

                if two_parts.len() != 2 {
                    panic!("Invalid generative expression: {}", expr_str);
                }

                let trimmed_left_side = two_parts[0].trim().to_string();

                if trimmed_left_side.len() != 1 {
                    panic!(
                        "Invalid left side of generative expression: {}",
                        trimmed_left_side
                    );
                }

                let left_side_id = symbol_manager
                    .register_symbol(&trimmed_left_side.chars().next().unwrap(), false);

                let timed_right_sides: Vec<String> = two_parts[1]
                    .split("|")
                    .map(|s| s.trim().to_string())
                    .collect();

                for right_side in timed_right_sides {
                    let mut right_side_ids = Vec::new();
                    let right_side_symbols: Vec<char> = right_side.chars().collect();

                    for symbol in right_side_symbols {
                        if symbol.is_ascii_uppercase() {
                            right_side_ids.push(symbol_manager.register_symbol(&symbol, false));
                        } else {
                            right_side_ids.push(symbol_manager.register_symbol(&symbol, true));
                        }
                    }

                    generative_expressions.push(GenerativeExpression {
                        left_side: left_side_id,
                        right_side: right_side_ids,
                    });
                }
            }

            generative_expressions
        }

        /**
         * A trait for types that can be symbolized and printed.
         */
        pub trait Symbolizable {}

        impl Symbolizable for char {}
        impl Symbolizable for String {}

        /**
         * Symbol management module for assigning unique IDs to symbols and allowing querying by ID or symbol.
         */
        pub mod symbol {
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
                pub nonterminator_symbol: HashSet<SymbolId>,
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
                pub fn register_symbol(&mut self, symbol: &T, is_terminator: bool) -> SymbolId {
                    if let Some(id) = self.query_id_by_symbol(&symbol) {
                        return id;
                    }

                    let new_id = loop {
                        if self.next_unused_id <= 1 {
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

                    if !is_terminator {
                        self.nonterminator_symbol.insert(new_id);
                    }

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
                pub fn new() -> Self {
                    let mut used_ids = HashSet::new();
                    used_ids.insert(0);
                    used_ids.insert(1);

                    let mut id_symbol_map = HashMap::new();
                    id_symbol_map.insert(0, 'ε');
                    id_symbol_map.insert(1, '#');

                    let mut symbol_id_map = HashMap::new();
                    symbol_id_map.insert('ε', 0);
                    symbol_id_map.insert('#', 1);

                    let next_unused_id = used_ids.len() as u64;

                    Self {
                        used_ids: used_ids,
                        next_unused_id: next_unused_id,
                        id_symbol_map: id_symbol_map,
                        symbol_id_map: symbol_id_map,
                        nonterminator_symbol: HashSet::new(),
                    }
                }
            }

            impl SymbolTable<String> {
                pub fn new() -> Self {
                    let mut used_ids = HashSet::new();
                    used_ids.insert(0);
                    used_ids.insert(1);

                    let mut id_symbol_map = HashMap::new();
                    id_symbol_map.insert(0, String::from("ε"));
                    id_symbol_map.insert(1, String::from("#"));

                    let mut symbol_id_map = HashMap::new();
                    symbol_id_map.insert(String::from("ε"), 0);
                    symbol_id_map.insert(String::from("#"), 1);

                    let next_unused_id = used_ids.len() as u64;

                    Self {
                        used_ids: used_ids,
                        next_unused_id: next_unused_id,
                        id_symbol_map: id_symbol_map,
                        symbol_id_map: symbol_id_map,
                        nonterminator_symbol: HashSet::new(),
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

            #[derive(Clone)]
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

        impl<T> GrammarContext<T>
        where
            T: Symbolizable + Eq + PartialEq + PartialOrd + Ord + Clone + Hash,
        {
            pub fn is_nonterminator(&self, symbol_id: symbol::SymbolId) -> bool {
                self.symbol_manager
                    .nonterminator_symbol
                    .contains(&symbol_id)
            }
        }
    }

    pub mod calc {
        use std::{
            collections::{HashMap, HashSet},
            io::ErrorKind::ConnectionAborted,
        };

        use crate::fc::gmr::symbol::SymbolId;

        pub fn calculate_first_follow_sets(
            context: &super::gmr::GrammarContext<char>,
        ) -> (HashMap<char, HashSet<char>>, HashMap<char, HashSet<char>>) {
            let mut first_id_sets: HashMap<u64, HashSet<u64>> = HashMap::new();
            let mut follow_id_sets: HashMap<u64, HashSet<u64>> = HashMap::new();

            // let nonterminator_ids = context
            //     .symbol_manager
            //     .nonterminator_symbol
            //     .iter()
            //     .filter(|&id| context.is_nonterminator(*id))
            //     .map(|&id| id)
            //     .collect::<Vec<SymbolId>>();

            // Calculate first sets
            loop {
                let mut changed = false;

                for expr in context.expressions.iter() {
                    let left_side_id = expr.left_side;
                    let size_backup = first_id_sets
                        .get(&left_side_id)
                        .map_or(0, |v: &HashSet<u64>| v.len());

                    for &symbol_id in expr.right_side.iter() {
                        // If the symbol is the epsilon symbol (ID 0), add it to the first set and break
                        if symbol_id == 0 {
                            first_id_sets
                                .entry(left_side_id)
                                .or_insert_with(|| HashSet::new())
                                .insert(0);
                            continue;
                        }

                        // If the symbol is a terminator, add it to its own first set and break
                        if !context.is_nonterminator(symbol_id) {
                            first_id_sets
                                .entry(left_side_id)
                                .or_insert_with(|| HashSet::new())
                                .insert(symbol_id);

                            if first_id_sets
                                .get(&left_side_id)
                                .map_or(0, |v: &HashSet<u64>| v.len())
                                > size_backup
                            {
                                changed = true;
                            }
                        } else {
                            // If the symbol is a non-terminator, add it to its own first set and continue
                            let mut this_symbol_first_set = first_id_sets
                                .entry(symbol_id)
                                .or_insert_with(HashSet::new)
                                .clone();

                            let should_continue;

                            if this_symbol_first_set.contains(&0) {
                                this_symbol_first_set.remove(&0);
                                should_continue = true;
                            } else {
                                should_continue = false;
                            }

                            first_id_sets
                                .entry(left_side_id)
                                .or_insert_with(HashSet::new)
                                .extend(this_symbol_first_set);

                            if should_continue {
                                continue;
                            }
                        }

                        break;
                    }

                    if first_id_sets
                        .get(&left_side_id)
                        .map_or(0, |v: &HashSet<u64>| v.len())
                        > size_backup
                    {
                        changed = true;
                    }

                    println!(
                        "First set for {}: {:?}",
                        context
                            .symbol_manager
                            .get_symbol_by_id(expr.left_side)
                            .unwrap(),
                        first_id_sets
                            .get(&expr.left_side)
                            .unwrap()
                            .iter()
                            .map(|&id| context.symbol_manager.get_symbol_by_id(id))
                            .collect::<Vec<_>>()
                    );
                }

                if !changed {
                    break;
                }
            }

            // Calculate follow sets
            loop {
                let mut changed = false;

                follow_id_sets
                    .entry(context.expressions[0].left_side)
                    .or_insert_with(HashSet::new)
                    .insert(1);

                // Calculate follow sets
                for expr in context.expressions.iter() {
                    let left_side_id = expr.left_side;

                    for (i, &symbol_id) in expr.right_side.iter().enumerate() {
                        // If the symbol is a terminator, skip it
                        if !context.is_nonterminator(symbol_id) {
                            continue;
                        }

                        let size_backup = follow_id_sets
                            .get(&symbol_id)
                            .map_or(0, |v: &HashSet<u64>| v.len());

                        // If the symbol is the last one in the right side, add the follow set of the left side to its follow set
                        if i == expr.right_side.len() - 1 {
                            let left_side_follow_set = follow_id_sets
                                .entry(left_side_id)
                                .or_insert_with(HashSet::new)
                                .clone();

                            follow_id_sets
                                .entry(symbol_id)
                                .or_insert_with(HashSet::new)
                                .extend(left_side_follow_set);
                        } else {
                            // If the symbol is not the last one, add the first set of the next symbol to its follow set
                            let next_symbol_id = expr.right_side[i + 1];

                            // If the next symbol is terminator, add it to this symbol's follow set.
                            if !context.is_nonterminator(next_symbol_id) {
                                follow_id_sets
                                    .entry(symbol_id)
                                    .or_insert_with(HashSet::new)
                                    .insert(next_symbol_id);
                            } else {
                                let mut next_symbol_id = next_symbol_id;

                                while context.is_nonterminator(next_symbol_id) {
                                    let next_symbol_first_set = first_id_sets
                                        .entry(next_symbol_id)
                                        .or_insert_with(HashSet::new)
                                        .clone();

                                    follow_id_sets
                                        .entry(symbol_id)
                                        .or_insert_with(HashSet::new)
                                        .extend(
                                            next_symbol_first_set.iter().filter(|&&id| id != 0),
                                        );

                                    if next_symbol_first_set.contains(&0) {
                                        next_symbol_id += 1;
                                        continue;
                                    } else {
                                        break;
                                    }
                                }

                                if !context.is_nonterminator(next_symbol_id) {
                                    follow_id_sets
                                        .entry(symbol_id)
                                        .or_insert_with(HashSet::new)
                                        .insert(next_symbol_id);
                                }
                            }
                        }

                        if follow_id_sets
                            .get(&symbol_id)
                            .map_or(0, |v: &HashSet<u64>| v.len())
                            > size_backup
                        {
                            changed = true;
                        }
                    }
                }

                if !changed {
                    break;
                }
            }

            let first_sets: HashMap<char, HashSet<char>> = first_id_sets
                .iter()
                .map(|(&id, set)| {
                    let symbol = context.symbol_manager.get_symbol_by_id(id).unwrap();
                    let symbol_set: HashSet<char> = set
                        .iter()
                        .filter_map(|&id| context.symbol_manager.get_symbol_by_id(id))
                        .collect();
                    (symbol, symbol_set)
                })
                .collect();

            let follow_sets: HashMap<char, HashSet<char>> = follow_id_sets
                .iter()
                .map(|(&id, set)| {
                    let symbol = context.symbol_manager.get_symbol_by_id(id).unwrap();
                    let symbol_set: HashSet<char> = set
                        .iter()
                        .filter_map(|&id| context.symbol_manager.get_symbol_by_id(id))
                        .collect();
                    (symbol, symbol_set)
                })
                .collect();

            (first_sets, follow_sets)
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
