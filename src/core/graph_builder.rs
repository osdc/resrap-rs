use std::collections::HashMap;

use crate::core::{frozen_graph::FrozenSyntaxGraph, parser::Parser, scanner::extract_tokens};

pub struct GraphBuilder {
    pub pars: Parser,
}

impl GraphBuilder {
    pub fn new() -> Self {
        GraphBuilder {
            pars: Parser::new(),
        }
    }

    pub fn take_graph(self) -> FrozenSyntaxGraph {
        self.pars.graph.freeze()
    }

    pub fn start_generation(&mut self, grammar: &str) -> Result<(), String> {
        let (cleaned, words) = extract_words(grammar)?;
        let (tokens, scan_errors) = extract_tokens(&cleaned);

        if !scan_errors.is_empty() {
            let all: Vec<String> = scan_errors
                .iter()
                .map(|err| format!("ERROR Scanning grammar >>> {}", err))
                .collect();
            return Err(all.join("\n"));
        }

        self.pars.tokens = tokens;
        self.pars.words = words;
        self.pars.parse_grammar();
        self.pars.attach_words();

        let mut all: Vec<String> = self
            .pars
            .errors
            .iter()
            .map(|err| format!("ERROR Parsing >>> {}", err))
            .collect();

        for err in self.pars.validate_graph() {
            all.push(format!("ERROR Validating >>> {}", err));
        }

        if !all.is_empty() {
            return Err(all.join("\n"));
        }

        self.pars.graph.print_map = self.pars.charmap.clone();
        self.pars.graph.name_map = self.pars.name_map.clone();
        self.pars.graph.regexer = self.pars.regexhandler.clone();
        self.pars.graph.normalize();

        Ok(())
    }
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Matches a line-oriented directive:
///
/// ```text
/// @words identifier = count, total, index;
/// ```
///
/// Directives are removed before the normal grammar is scanned.
fn extract_words(grammar: &str) -> Result<(String, HashMap<String, Vec<String>>), String> {
    let mut out = String::with_capacity(grammar.len());
    let mut words: HashMap<String, Vec<String>> = HashMap::new();

    for raw in grammar.split_inclusive('\n') {
        let line = raw.trim_start();
        let rest = if let Some(rest) = line.strip_prefix("@words") {
            Some(rest)
        } else {
            line.strip_prefix("@keywords")
        };

        let rest = match rest {
            Some(rest)
                if rest
                    .chars()
                    .next()
                    .map(char::is_whitespace)
                    .unwrap_or(false) =>
            {
                rest
            }
            _ => {
                out.push_str(raw);
                continue;
            }
        };

        let rest = rest.trim_start();
        let name_end = rest.find(|c: char| c.is_whitespace()).unwrap_or(rest.len());
        let name = &rest[..name_end];
        if name.is_empty() {
            return Err("@words/@keywords: expected a rule name".to_string());
        }

        let after = rest[name_end..].trim_start();
        let after = after
            .strip_prefix('=')
            .or_else(|| after.strip_prefix(':'))
            .ok_or_else(|| {
                format!(
                    "@words/@keywords {}: expected '=' or ':' after the rule name",
                    name
                )
            })?;

        let value = after.trim();
        let value = value.strip_suffix(';').unwrap_or(value);
        let list: Vec<String> = value
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter(|word| !word.is_empty())
            .map(|word| word.to_string())
            .collect();

        if list.is_empty() {
            return Err(format!(
                "@words/@keywords {}: expected at least one word",
                name
            ));
        }

        words.insert(name.to_string(), list);
        // Keep line numbering stable for scanner errors.
        out.push('\n');
    }

    Ok((out, words))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_words_directive() {
        let (grammar, words) =
            extract_words("@words identifier = count, total, index;\nprogram: identifier ';';\n")
                .unwrap();
        assert_eq!(
            words.get("identifier").unwrap(),
            &vec![
                "count".to_string(),
                "total".to_string(),
                "index".to_string()
            ]
        );
        assert!(!grammar.contains("@words"));
    }

    #[test]
    fn extracts_keywords_alias() {
        let (_, words) = extract_words("@keywords kind = int, float\nprogram: kind;").unwrap();
        assert_eq!(words.get("kind").unwrap().len(), 2);
    }

    #[test]
    fn rejects_empty_word_list() {
        assert!(extract_words("@words x = ;\nprogram:x;").is_err());
    }

    #[test]
    fn parses_simple_grammar() {
        let mut gb = GraphBuilder::new();
        assert!(gb.start_generation("program:'hello' 'world';").is_ok());
        assert!(gb.pars.graph.name_map.contains_key("program"));
    }

    #[test]
    fn missing_definition_is_an_error() {
        let mut gb = GraphBuilder::new();
        let err = gb.start_generation("program:missing;").unwrap_err();
        assert!(err.contains("not found"), "{}", err);
    }

    #[test]
    fn duplicate_definition_is_an_error() {
        let mut gb = GraphBuilder::new();
        let err = gb
            .start_generation("program:'a';\nprogram:'b';")
            .unwrap_err();
        assert!(err.contains("multiple definitions"), "{}", err);
    }

    #[test]
    fn scan_errors_are_prefixed() {
        let mut gb = GraphBuilder::new();
        let err = gb.start_generation("program:'unterminated").unwrap_err();
        assert!(err.contains("ERROR Scanning grammar >>>"), "{}", err);
    }

    #[test]
    fn word_only_rule_is_allowed() {
        let mut gb = GraphBuilder::new();
        assert!(
            gb.start_generation("@words identifier = alpha, beta\nprogram:identifier;")
                .is_ok()
        );
    }

    #[test]
    fn recursive_grammar_parses() {
        let mut gb = GraphBuilder::new();
        assert!(
            gb.start_generation(
                "program:function^;\nfunction:identifier '(' ')';\nidentifier:[a-z];"
            )
            .is_ok()
        );
    }
}
