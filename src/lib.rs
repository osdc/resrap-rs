//! Resrap is a seedable, grammar-based code snippet generator.
//!
//! Instead of parsing code, it generates code from formal ABNF-style grammars.
//! This crate is a Rust port of the Go package
//! [`github.com/osdc/resrap`](https://pkg.go.dev/github.com/osdc/resrap) and aims
//! to match its API and generation behaviour exactly, including deterministic
//! output for a given seed.

mod core;
mod mt;
pub mod templates;

pub use core::codebasegen::{FileType, Node, parse_dsl};
pub use mt::{CodeChannel, CodeGenRes, ERR_SHUT_DOWN, ResrapMT};

use std::collections::HashMap;

use crate::core::{file::Lang, prng::PRNG};

/// Implemented by both [`Resrap`] and [`ResrapMT`], allowing the built-in
/// templates to load into either one.
pub trait GrammarLoader {
    fn parse_grammar(&mut self, name: &str, grammar: &str) -> Result<(), String>;
}

/// Resrap is the main access point for single-threaded use. It is a collection
/// of grammars which can be generated from a parsed grammar graph.
pub struct Resrap {
    language_graph: HashMap<String, Lang>,
}

impl Resrap {
    /// Creates a new instance with no loaded grammars.
    pub fn new() -> Self {
        Resrap {
            language_graph: HashMap::new(),
        }
    }

    /// Parses a grammar string and stores it under the given name.
    ///
    /// `name` is a unique identifier for this grammar (e.g. `"C"`). `grammar`
    /// must use the ABNF dialect documented by the reference package.
    pub fn parse_grammar(&mut self, name: &str, grammar: &str) -> Result<(), String> {
        self.parse_grammar_impl(name, grammar)
    }

    fn parse_grammar_impl(&mut self, name: &str, grammar: &str) -> Result<(), String> {
        let mut lang = Lang::new();
        lang.parse_string(grammar)?;
        self.language_graph.insert(name.to_string(), lang);
        Ok(())
    }

    /// Parses a grammar from a file and stores it under the given name.
    pub fn parse_grammar_file(&mut self, name: &str, location: &str) -> Result<(), String> {
        let mut lang = Lang::new();
        lang.parse_file(location)?;
        self.language_graph.insert(name.to_string(), lang);
        Ok(())
    }

    /// Attaches a list of words to a rule in a parsed grammar. Any reference to
    /// that rule will emit one of the words instead of expanding the rule.
    pub fn add_words(&self, name: &str, rule: &str, words: &[String]) -> Result<(), String> {
        let lang = self
            .language_graph
            .get(name)
            .ok_or_else(|| format!("resrap: grammar {:?} is not loaded", name))?;
        let graph = lang
            .get_graph()
            .ok_or_else(|| format!("resrap: grammar {:?} is not loaded", name))?;
        graph.set_words(rule, words)
    }

    /// Alias for [`Resrap::add_words`].
    pub fn add_keywords(&self, name: &str, rule: &str, words: &[String]) -> Result<(), String> {
        self.add_words(name, rule, words)
    }

    /// Generates content, stopping exactly at the token budget. The output may
    /// be cut in the middle of an expression.
    pub fn generate_random(&self, name: &str, starting_node: &str, tokens: usize) -> String {
        self.generate(name, starting_node, PRNG::new(0), tokens, false)
    }

    /// Deterministic variant of [`Resrap::generate_random`].
    ///
    /// A seed of `0` is not deterministic: it selects a fresh random seed for
    /// this call, matching the Go reference implementation.
    pub fn generate_with_seeded(
        &self,
        name: &str,
        starting_node: &str,
        seed: u64,
        tokens: usize,
    ) -> String {
        self.generate(name, starting_node, PRNG::new(seed), tokens, false)
    }

    /// Backwards-compatible alias for [`Resrap::generate_with_seeded`].
    pub fn generate_with_seed(
        &self,
        name: &str,
        starting_node: &str,
        seed: u64,
        tokens: usize,
    ) -> String {
        self.generate_with_seeded(name, starting_node, seed, tokens)
    }

    /// Generates content that stops at the boundary of the starting rule once
    /// the token budget is reached. The budget is a soft limit, so output may
    /// contain slightly more than `tokens` terminals.
    pub fn generate_complete(&self, name: &str, starting_node: &str, tokens: usize) -> String {
        self.generate(name, starting_node, PRNG::new(0), tokens, true)
    }

    /// Deterministic variant of [`Resrap::generate_complete`].
    ///
    /// A seed of `0` is not deterministic: it selects a fresh random seed for
    /// this call, matching the Go reference implementation.
    pub fn generate_complete_with_seeded(
        &self,
        name: &str,
        starting_node: &str,
        seed: u64,
        tokens: usize,
    ) -> String {
        self.generate(name, starting_node, PRNG::new(seed), tokens, true)
    }

    fn generate(
        &self,
        name: &str,
        starting_node: &str,
        prng: PRNG,
        tokens: usize,
        complete: bool,
    ) -> String {
        let graph = match self
            .language_graph
            .get(name)
            .and_then(|lang| lang.get_graph())
        {
            Some(graph) => graph,
            None => return String::new(),
        };
        if complete {
            graph.walk_graph_complete(prng, starting_node, tokens)
        } else {
            graph.walk_graph(prng, starting_node, tokens)
        }
    }

    /// Generates a complete codebase from a DSL file.
    ///
    /// ```text
    /// src/
    ///   code/
    ///     c[10x20 code_*.c]
    ///   more/
    ///     sql[10x100 *.sql]
    /// ```
    pub fn generate_codebase(&self, config_loc: &str, target: &str) -> Result<(), String> {
        let config = std::fs::read_to_string(config_loc).map_err(|err| err.to_string())?;
        let parent = parse_dsl(&config)?;
        parent.generate_structure(self, target)
    }
}

impl Default for Resrap {
    fn default() -> Self {
        Self::new()
    }
}

impl GrammarLoader for Resrap {
    fn parse_grammar(&mut self, name: &str, grammar: &str) -> Result<(), String> {
        self.parse_grammar_impl(name, grammar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates;

    const GRAMMAR: &str = "program:function^;\nfunction:'f' '(' ')';\n";

    fn loaded() -> Resrap {
        let mut r = Resrap::new();
        r.parse_grammar("Test", GRAMMAR).unwrap();
        r
    }

    #[test]
    fn new_instance_is_empty() {
        let r = Resrap::new();
        assert!(r.language_graph.is_empty());
    }

    #[test]
    fn parses_and_generates() {
        let r = loaded();
        assert!(!r.generate_random("Test", "program", 10).is_empty());
    }

    #[test]
    fn invalid_grammar_is_not_stored() {
        let mut r = Resrap::new();
        assert!(r.parse_grammar("Test", "program:missing;").is_err());
        assert_eq!(r.generate_random("Test", "program", 10), "");
    }

    #[test]
    fn missing_file_is_an_error() {
        let mut r = Resrap::new();
        assert!(
            r.parse_grammar_file("Test", "/nonexistent/grammar.g4")
                .is_err()
        );
    }

    #[test]
    fn parses_grammar_file() {
        let dir = std::env::temp_dir().join(format!("resrap-api-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("grammar.g4");
        std::fs::write(&path, GRAMMAR).unwrap();
        let mut r = Resrap::new();
        r.parse_grammar_file("Test", path.to_str().unwrap())
            .unwrap();
        assert!(!r.generate_random("Test", "program", 10).is_empty());
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn unknown_language_returns_empty() {
        let r = Resrap::new();
        assert_eq!(r.generate_random("Unknown", "program", 10), "");
        assert_eq!(r.generate_complete("Unknown", "program", 10), "");
    }

    #[test]
    fn seeded_generation_is_deterministic() {
        let r = loaded();
        let a = r.generate_with_seeded("Test", "program", 1234, 50);
        let b = r.generate_with_seeded("Test", "program", 1234, 50);
        assert_eq!(a, b);
    }

    #[test]
    fn different_seeds_differ() {
        let mut r = Resrap::new();
        r.parse_grammar(
            "Test",
            "program:function^;\nfunction:('a'<0.5>) | ('b'<0.5>);",
        )
        .unwrap();
        assert_ne!(
            r.generate_with_seeded("Test", "program", 1, 50),
            r.generate_with_seeded("Test", "program", 2, 50)
        );
    }

    #[test]
    fn zero_seed_is_randomised_not_stuck() {
        let r = loaded();
        let a = r.generate_with_seeded("Test", "program", 0, 50);
        assert!(!a.is_empty());
    }

    #[test]
    fn complete_generation_finishes_rule() {
        let mut r = Resrap::new();
        r.parse_grammar(
            "Test",
            "program:'begin\\n' item* 'end\\n';\nitem:'item\\n';",
        )
        .unwrap();
        for seed in 1..=50 {
            let out = r.generate_complete_with_seeded("Test", "program", seed, 3);
            assert!(out.starts_with("begin\n"), "seed {}: {:?}", seed, out);
            assert!(out.ends_with("end\n"), "seed {}: {:?}", seed, out);
        }
    }

    #[test]
    fn words_directive_is_used() {
        let mut r = Resrap::new();
        r.parse_grammar(
            "Test",
            "@words identifier = alpha, beta\nprogram:identifier;",
        )
        .unwrap();
        let out = r.generate_with_seeded("Test", "program", 3, 10);
        assert!(out == "alpha" || out == "beta", "{:?}", out);
    }

    #[test]
    fn add_words_updates_a_rule() {
        let mut r = Resrap::new();
        r.parse_grammar("Test", "program:identifier;\nidentifier:[a-z];")
            .unwrap();
        assert!(
            r.add_words("Test", "identifier", &["only".to_string()])
                .is_ok()
        );
        assert_eq!(r.generate_with_seeded("Test", "program", 1, 10), "only");
    }

    #[test]
    fn add_words_rejects_unknown_grammar_and_rule() {
        let r = loaded();
        assert!(r.add_words("Missing", "x", &["a".to_string()]).is_err());
        assert!(r.add_words("Test", "missing", &["a".to_string()]).is_err());
    }

    #[test]
    fn generate_codebase_writes_files() {
        let mut r = Resrap::new();
        r.parse_grammar("c", GRAMMAR).unwrap();
        let dir = std::env::temp_dir().join(format!("resrap-codebase-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let config = dir.join("format.dsl");
        std::fs::write(&config, "out/\n  c[2x10 code_*.c]\n").unwrap();
        let target = dir.join("target");
        r.generate_codebase(config.to_str().unwrap(), target.to_str().unwrap())
            .unwrap();
        for i in 1..=2 {
            assert!(target.join("out").join(format!("code_{}.c", i)).is_file());
        }
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn generate_codebase_reports_missing_config() {
        let r = Resrap::new();
        assert!(
            r.generate_codebase("/nonexistent/config.dsl", "/tmp/out")
                .is_err()
        );
    }

    #[test]
    fn templates_implement_grammar_loader() {
        let mut r = Resrap::new();
        templates::load(&mut r, templates::Lang::Cpp).unwrap();
        assert!(
            !r.generate_complete_with_seeded("cpp", "program", 7, 60)
                .is_empty()
        );
    }
}
