use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::core::{graph::NodeType, prng::PRNG, regex::Regexer};

pub struct FrozenSyntaxGraph {
    pub node_ref: HashMap<u32, Arc<FrozenSyntaxNode>>,
    pub name_map: HashMap<String, u32>,
    pub print_map: HashMap<u32, String>,
    pub regexer: Regexer,
    /// Rule id -> words emitted instead of expanding the rule.
    pub words: RwLock<HashMap<u32, Vec<String>>>,
    /// Rule id -> the end node of that rule.
    pub rule_ends: HashMap<u32, u32>,
}

pub struct FrozenSyntaxNode {
    pub options: Vec<FrozenSyntaxEdge>,
    pub cumulative_frequency: Vec<f32>,
    pub id: u32,
    pub typ: NodeType,
    pub pointer: u32,
}

pub struct FrozenSyntaxEdge {
    /// Id of the target node. Ids keep the frozen graph cycle-safe without
    /// pointing every edge at a separate heap allocation.
    pub node: u32,
}

impl FrozenSyntaxGraph {
    pub fn new(
        node_ref: HashMap<u32, Arc<FrozenSyntaxNode>>,
        name_map: HashMap<String, u32>,
        print_map: HashMap<u32, String>,
        regexer: Regexer,
        words: HashMap<u32, Vec<String>>,
        rule_ends: HashMap<u32, u32>,
    ) -> Self {
        FrozenSyntaxGraph {
            node_ref,
            name_map,
            print_map,
            regexer,
            words: RwLock::new(words),
            rule_ends,
        }
    }

    /// Attaches words to a rule. An empty list removes the word list.
    pub fn set_words(&self, rule: &str, words: &[String]) -> Result<(), String> {
        let id = self
            .name_map
            .get(rule)
            .copied()
            .ok_or_else(|| format!("rule '{}' not found", rule))?;
        let mut guard = self.words.write().unwrap();
        if words.is_empty() {
            guard.remove(&id);
        } else {
            guard.insert(id, words.to_vec());
        }
        Ok(())
    }

    pub fn walk_graph(&self, prng: PRNG, start: &str, tokens: usize) -> String {
        self.walk(prng, start, tokens, false)
    }

    pub fn walk_graph_complete(&self, prng: PRNG, start: &str, tokens: usize) -> String {
        self.walk(prng, start, tokens, true)
    }

    fn walk(&self, mut prng: PRNG, start: &str, tokens: usize, complete: bool) -> String {
        if tokens == 0 {
            return String::new();
        }

        let start_id = match self.name_map.get(start) {
            Some(id) => *id,
            None => return String::new(),
        };

        // A rule can itself be a word list, e.g. @words identifier = ...
        {
            let words = self.words.read().unwrap();
            if let Some(list) = words.get(&start_id)
                && !list.is_empty()
            {
                return list[prng.random_int(0, list.len())].clone();
            }
        }

        let starting_node = match self.node_ref.get(&start_id) {
            Some(node) => Arc::clone(node),
            None => return String::new(),
        };

        let start_end = self.rule_ends.get(&start_id).copied();
        let max_iterations = tokens
            .saturating_mul(1000)
            .saturating_add(10_000)
            .max(10_000);

        let mut result = String::new();
        let mut jump_stack: Vec<u32> = Vec::new();
        let mut printed_tokens: usize = 0;
        let mut current = starting_node;

        for _ in 0..max_iterations {
            if tokens > 0 && printed_tokens >= tokens {
                if !complete {
                    return result;
                }
                // In complete mode only stop at the starting rule's boundary.
                if jump_stack.is_empty() && Some(current.id) == start_end {
                    return result;
                }
            }

            match current.typ {
                NodeType::CH => {
                    if let Some(content) = self.print_map.get(&current.id) {
                        result.push_str(&unescape_string(content));
                    }
                    printed_tokens += 1;
                }
                NodeType::RX => {
                    if let Some(content) = self.print_map.get(&current.id) {
                        result.push_str(&self.regexer.generate_string(content, &mut prng));
                    }
                    printed_tokens += 1;
                }
                NodeType::LOOP => {
                    // '^' repeats the construct immediately before it until
                    // the token budget is reached. In complete mode the walk
                    // then falls through to whatever follows the loop.
                    if !complete || printed_tokens < tokens {
                        match self.node_ref.get(&current.pointer) {
                            Some(target) => {
                                current = Arc::clone(target);
                                continue;
                            }
                            None => return result,
                        }
                    }
                }
                NodeType::POINTER => {
                    // If the target rule has a word list, emit a word directly
                    // and return to the caller instead of traversing its rule.
                    let word = {
                        let words = self.words.read().unwrap();
                        words.get(&current.pointer).and_then(|list| {
                            if list.is_empty() {
                                None
                            } else {
                                Some(list[prng.random_int(0, list.len())].clone())
                            }
                        })
                    };
                    if let Some(word) = word {
                        result.push_str(&word);
                        printed_tokens += 1;
                        match current.options.first() {
                            Some(next) => match self.node_ref.get(&next.node) {
                                Some(node) => {
                                    current = Arc::clone(node);
                                    continue;
                                }
                                None => return result,
                            },
                            None => return result,
                        }
                    }

                    let next = match current.options.first() {
                        Some(next) => next.node,
                        None => return result,
                    };
                    jump_stack.push(next);
                    match self.node_ref.get(&current.pointer) {
                        Some(target) => {
                            current = Arc::clone(target);
                            continue;
                        }
                        None => return result,
                    }
                }
                NodeType::END => {
                    if let Some(target) = jump_stack.pop() {
                        match self.node_ref.get(&target) {
                            Some(node) => {
                                current = Arc::clone(node);
                                continue;
                            }
                            None => return result,
                        }
                    }
                }
                _ => {}
            }

            match self.choice(&current, &mut prng) {
                Some(next) => match self.node_ref.get(&next) {
                    Some(node) => current = Arc::clone(node),
                    None => return result,
                },
                None => return result,
            }
        }

        result
    }

    fn choice(&self, current: &FrozenSyntaxNode, prng: &mut PRNG) -> Option<u32> {
        if current.options.is_empty() {
            return None;
        }
        if current.cumulative_frequency.len() != current.options.len()
            || current.cumulative_frequency.is_empty()
        {
            // Graph was not normalized (or was built by hand). Fall back to a
            // uniform choice rather than indexing an empty array.
            let index = prng.random_int(0, current.options.len());
            return Some(current.options[index].node);
        }

        let value = prng.random() as f32;
        let mut index = current
            .cumulative_frequency
            .iter()
            .position(|&cf| cf >= value)
            .unwrap_or(current.cumulative_frequency.len() - 1);
        if index >= current.options.len() {
            index = current.options.len() - 1;
        }
        Some(current.options[index].node)
    }
}

/// Handles escape sequences in generated literals.
pub fn unescape_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(next_ch) = chars.next() {
                match next_ch {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '\\' => result.push('\\'),
                    '\'' => result.push('\''),
                    '"' => result.push('"'),
                    '0' => result.push('\0'),
                    _ => {
                        // Unknown escape: keep both characters.
                        result.push('\\');
                        result.push(next_ch);
                    }
                }
            } else {
                result.push('\\');
            }
        } else {
            result.push(ch);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{frozen_graph::FrozenSyntaxGraph, graph::SyntaxGraph};

    fn simple_graph(text: &str) -> FrozenSyntaxGraph {
        let mut g = SyntaxGraph::new();
        g.name_map.insert("start".to_string(), 1);
        g.print_map.insert(2, text.to_string());
        let head = g.force_get_node(1, NodeType::HEADER);
        let leaf = g.force_get_node(2, NodeType::CH);
        {
            head.lock().unwrap().add_edge(Arc::clone(&leaf), 1.0);
        }
        g.normalize();
        g.freeze()
    }

    #[test]
    fn walks_character_output() {
        let g = simple_graph("hello");
        let out = g.walk_graph(PRNG::new(5), "start", 10);
        assert_eq!(out, "hello");
    }

    #[test]
    fn unknown_start_is_empty() {
        let g = simple_graph("hello");
        assert_eq!(g.walk_graph(PRNG::new(1), "missing", 10), "");
    }

    #[test]
    fn token_limit_is_hard() {
        let mut g = SyntaxGraph::new();
        g.name_map.insert("start".to_string(), 1);
        g.print_map.insert(2, "abc".to_string());
        let head = g.force_get_node(1, NodeType::HEADER);
        let leaf = g.force_get_node(2, NodeType::CH);
        {
            head.lock().unwrap().add_edge(Arc::clone(&leaf), 1.0);
            leaf.lock().unwrap().add_edge(Arc::clone(&head), 1.0);
        }
        g.normalize();
        let frozen = g.freeze();
        let out = frozen.walk_graph(PRNG::new(5), "start", 3);
        assert_eq!(out.len(), 9);
    }

    #[test]
    fn unescapes_literals() {
        assert_eq!(unescape_string(r"a\nb\tc\\d"), "a\nb\tc\\d");
        assert_eq!(unescape_string(r"\q"), r"\q");
        assert_eq!(unescape_string("\\"), "\\");
    }

    #[test]
    fn words_are_used_for_pointer_targets() {
        let mut g = SyntaxGraph::new();
        g.name_map.insert("program".to_string(), 1);
        g.name_map.insert("identifier".to_string(), 2);
        g.words.insert(2, vec!["alpha".to_string()]);
        let head = g.force_get_node(1, NodeType::HEADER);
        let pointer = g.force_get_node(3, NodeType::POINTER);
        {
            let mut p = pointer.lock().unwrap();
            p.pointer = 2;
            let jump = g.force_get_node(4, NodeType::JUMP);
            p.add_edge(jump, 1.0);
        }
        {
            head.lock().unwrap().add_edge(pointer, 1.0);
        }
        g.normalize();
        let frozen = g.freeze();
        assert_eq!(frozen.walk_graph(PRNG::new(1), "program", 10), "alpha");
        assert_eq!(
            frozen.walk_graph_complete(PRNG::new(1), "program", 10),
            "alpha"
        );
    }

    #[test]
    fn complete_mode_finishes_the_starting_rule() {
        let mut g = SyntaxGraph::new();
        g.name_map.insert("program".to_string(), 1);
        g.print_map.insert(10, "begin".to_string());
        g.print_map.insert(11, "item".to_string());
        g.print_map.insert(12, "end".to_string());

        // program: 'begin' item* 'end';
        let head = g.force_get_node(1, NodeType::HEADER);
        let begin = g.force_get_node(10, NodeType::CH);
        let item = g.force_get_node(11, NodeType::CH);
        let end_lit = g.force_get_node(12, NodeType::CH);
        let end_node = g.force_get_node(100, NodeType::END);
        g.rule_ends.insert(1, 100);

        {
            head.lock().unwrap().add_edge(Arc::clone(&begin), 1.0);
            begin.lock().unwrap().add_edge(Arc::clone(&item), 1.0);
            item.lock().unwrap().add_edge(Arc::clone(&end_lit), 1.0);
            // zero-or-more-ish cycle back
            item.lock().unwrap().add_edge(Arc::clone(&item), 0.0);
            end_lit.lock().unwrap().add_edge(Arc::clone(&end_node), 1.0);
        }
        g.normalize();
        let frozen = g.freeze();
        let out = frozen.walk_graph_complete(PRNG::new(7), "program", 2);
        assert!(out.starts_with("begin"));
        assert!(out.ends_with("end"));
    }

    #[test]
    fn loop_node_stays_inside_its_rule() {
        let mut g = SyntaxGraph::new();
        g.name_map.insert("a".to_string(), 1);
        g.print_map.insert(10, "A".to_string());
        let head = g.force_get_node(1, NodeType::HEADER);
        let ch = g.force_get_node(10, NodeType::CH);
        let loop_node = g.force_get_node(11, NodeType::LOOP);
        let end = g.force_get_node(12, NodeType::END);
        g.rule_ends.insert(1, 12);
        {
            head.lock().unwrap().add_edge(Arc::clone(&ch), 1.0);
            ch.lock().unwrap().add_edge(Arc::clone(&loop_node), 1.0);
            loop_node.lock().unwrap().pointer = 1;
            loop_node.lock().unwrap().add_edge(Arc::clone(&end), 1.0);
        }
        g.normalize();
        let frozen = g.freeze();
        let out = frozen.walk_graph(PRNG::new(11), "a", 40);
        assert!(out.chars().all(|c| c == 'A'));
        assert!(!out.is_empty());
    }
}
