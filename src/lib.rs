mod core;
use std::collections::HashMap;

use crate::core::{file::Lang, prng::PRNG};

/// Resrap is the main access point for single-threaded uses.
/// It's a collection of grammars which can be generated using parsing grammar.
pub struct Resrap {
    language_graph: HashMap<String, Lang>,
}

impl Resrap {
    /// Creates and returns a new Resrap instance.
    /// The returned instance starts with no loaded grammars.
    pub fn new() -> Self {
        Resrap {
            language_graph: HashMap::new(),
        }
    }

    /// Parses a grammar string and stores it under the given name.
    ///
    /// # Arguments
    /// * `name` - A unique identifier for this grammar (e.g., "C"), should be in ABNF format
    ///            (Check osdc/resrap for more info on that).
    /// * `grammar` - The grammar string to parse
    ///
    /// # Returns
    /// Returns error generated while parsing
    pub fn parse_grammar(&mut self, name: String, grammar: String) -> Result<(), String> {
        let mut lang = Lang::new();
        let err = lang.parse_string(grammar);

        self.language_graph.insert(name.clone(), lang);
        err
    }

    /// Parses a grammar from a file and stores it under the given name.
    ///
    /// # Arguments
    /// * `name` - A unique identifier for this grammar (e.g., "C"), should be in ABNF format
    ///            (Check osdc/resrap for more info on that).
    /// * `location` - Path to the grammar file
    ///
    /// # Returns
    /// Returns error generated while parsing
    pub fn parse_grammar_file(&mut self, name: String, location: String) -> Result<(), String> {
        let mut lang = Lang::new();
        let err = lang.parse_file(location);

        self.language_graph.insert(name.clone(), lang);
        err
    }

    /// Generates content from the grammar identified by 'name'.
    ///
    /// # Arguments
    /// * `name` - The grammar name to use
    /// * `starting_node` - The starting heading in the grammar for generation
    /// * `tokens` - Number of tokens to generate
    ///
    /// # Returns
    /// A string containing the generated content.
    /// The generation is non-deterministic (random).
    pub fn generate_random(
        &self,
        name: &str,
        starting_node: String,
        tokens: usize,
    ) -> Result<String, &str> {
        let prng = PRNG::new(0);
        self.language_graph
            .get(name)
            .unwrap()
            .get_graph()
            .unwrap()
            .walk_graph(prng, starting_node, tokens)
    }

    /// Generates content from the grammar identified by 'name' with a seed.
    ///
    /// # Arguments
    /// * `name` - The grammar name to use
    /// * `starting_node` - The starting symbol in the grammar for generation
    /// * `seed` - A numeric seed to make generation deterministic
    /// * `tokens` - Number of tokens to generate
    ///
    /// # Returns
    /// A string containing the generated content.
    pub fn generate_with_seeded(
        &self,
        name: &str,
        starting_node: String,
        seed: u64,
        tokens: usize,
    ) -> Result<String, &str> {
        let prng = PRNG::new(seed);
        self.language_graph
            .get(name)
            .unwrap()
            .get_graph()
            .unwrap()
            .walk_graph(prng, starting_node, tokens)
    }
}

impl Default for Resrap {
    fn default() -> Self {
        Self::new()
    }
}
