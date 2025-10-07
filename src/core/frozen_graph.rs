use std::{collections::HashMap, sync::Arc};

use crate::core::{graph::NodeType, prng::PRNG, regex::Regexer};

pub struct FrozenSyntaxGraph {
    pub node_ref: HashMap<u32, Arc<FrozenSyntaxNode>>,
    pub name_map: HashMap<String, u32>,
    pub print_map: HashMap<u32, String>,
    pub regexer: Regexer,
}

pub struct FrozenSyntaxNode {
    pub options: Vec<FrozenSyntaxEdge>,
    pub cumulative_frequency: Vec<f32>,
    pub id: u32,
    pub typ: NodeType,
    pub pointer: u32,
}

pub struct FrozenSyntaxEdge {
    pub node: Arc<FrozenSyntaxNode>,
}
impl FrozenSyntaxGraph {
    pub fn walk_graph(&self, mut prng: PRNG, start: String, tokens: usize) -> Result<String, &str> {
        let mut result = String::from("");
        let mut graph_stack: Vec<u32> = vec![];

        if let Some(start_id) = self.name_map.get(&start) {
            let mut printed_tokens: usize = 0;
            let mut current_id = *start_id;

            loop {
                // Always fetch fresh from node_ref
                let current = self
                    .node_ref
                    .get(&current_id)
                    .ok_or("Node not found in graph")?;

                if printed_tokens >= tokens {
                    return Ok(result);
                }

                match current.typ {
                    NodeType::CH => {
                        if let Some(content) = self.print_map.get(&current.id) {
                            result.push_str(&unescape_string(&content));
                            printed_tokens += 1;
                        }
                    }
                    NodeType::RX => {
                        if let Some(content) = self.print_map.get(&current.id) {
                            let content = self.regexer.generate_string(content, &mut prng);
                            result.push_str(&content);
                            printed_tokens += 1;
                        }
                    }
                    NodeType::POINTER => {
                        if let Some(ret_node) = current.options.first() {
                            graph_stack.push(ret_node.node.id);
                            current_id = current.pointer;
                        }
                        continue;
                    }
                    NodeType::END => {
                        if graph_stack.is_empty() {
                            return Ok(result);
                        } else {
                            let ret_node = graph_stack.pop().unwrap();
                            current_id = ret_node;
                        }
                        continue;
                    }
                    _ => {}
                }

                if current.options.is_empty() {
                    return Ok(result);
                }

                let value = prng.random() as f32;
                let index = match current
                    .cumulative_frequency
                    .iter()
                    .position(|&x| x >= value)
                {
                    Some(i) => i,
                    None => current.cumulative_frequency.len() - 1,
                };

                current_id = current.options[index].node.id;
            }
        } else {
            Err("Could not find starting node")
        }
    }
}
// Helper function to handle escape sequences
fn unescape_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();

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
                    _ => {
                        // If it's not a recognized escape sequence, keep the backslash
                        result.push('\\');
                        result.push(next_ch);
                    }
                }
            } else {
                // Trailing backslash
                result.push('\\');
            }
        } else {
            result.push(ch);
        }
    }

    result
}
