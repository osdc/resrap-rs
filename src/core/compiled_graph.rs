use std::{collections::HashMap, sync::Arc};

use crate::core::{
    graph::{NodeType, SyntaxEdge, SyntaxGraph, SyntaxNode},
    prng::PRNG,
    regex::{self, Regexer},
};

//Compiled, Read only graph with Arc
pub struct Compiled_Graph {
    pub node_ref: HashMap<u32, Arc<Compiled_Syntax_Node>>,
    pub name_map: HashMap<String, u32>,
    pub char_map: HashMap<u32, String>,
    pub regexer: Regexer,
}
impl Compiled_Graph {
    pub fn graphwalk(&mut self, prng: &mut PRNG, start: &str, tokens: u32) -> String {
        let mut result = String::new();
        let mut jump_stack: Vec<u32> = Vec::new();

        let mut current = self.get_node(
            self.name_map
                .get(start)
                .expect("Starting Token does not exist")
                .clone(),
            NodeType::IDK,
        );
        let mut printed_tokens: u32 = 0;

        loop {
            if printed_tokens >= tokens {
                return result;
            }

            // Process logic based on node type
            match current.typ {
                NodeType::CH => {
                    // Extract content and handle escape sequences
                    if let Some(content) = self.char_map.get(&current.id) {
                        let unescaped = unescape_string(content);
                        printed_tokens += 1;
                        result.push_str(&unescaped);
                    }
                }
                NodeType::RX => {
                    if let Some(pattern) = self.char_map.get(&current.id) {
                        let generated = self.regexer.generate_string(pattern, prng);
                        result.push_str(&generated);
                    }
                }
                NodeType::POINTER => {
                    jump_stack.push(current.edges.as_ref().unwrap().get(0).unwrap().option.id);
                    let nextnode = self.get_node(current.pointer, NodeType::HEADER);
                    current = nextnode;
                    continue; // Skip the normal next node selection
                }
                NodeType::END => {
                    if let Some(id) = jump_stack.pop() {
                        current = self.get_node(id, NodeType::IDK);
                        continue; // Skip the normal next node selection
                    }
                }
                _ => {}
            }

            // Move to next (randomly selected if multiple options)
            if !current.edges.as_ref().unwrap().is_empty() {
                let value = prng.random() as f32;

                // Binary search for the CF value
                let index = current
                    .cf
                    .binary_search_by(|probe| {
                        if probe < &value {
                            std::cmp::Ordering::Less
                        } else {
                            std::cmp::Ordering::Greater
                        }
                    })
                    .unwrap_or_else(|i| i);

                // Ensure index is within bounds
                let index = index.min(current.edges.as_ref().unwrap().len() - 1);

                let nextnode =
                    Arc::clone(&current.edges.as_ref().unwrap().get(index).unwrap().option);
                current = nextnode
            }
        }
    }

    pub fn get_node(&self, id: u32, _typ: NodeType) -> Arc<Compiled_Syntax_Node> {
        self.node_ref.get(&id).expect("Node not found").clone()
    }
}

// Helper function to handle escape sequences
fn unescape_string(s: &str) -> String {
    let mut result = String::new();
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            match bytes[i + 1] {
                b'n' => {
                    result.push('\n');
                    i += 2;
                }
                b't' => {
                    result.push('\t');
                    i += 2;
                }
                b'r' => {
                    result.push('\r');
                    i += 2;
                }
                b'\\' => {
                    result.push('\\');
                    i += 2;
                }
                b'\'' => {
                    result.push('\'');
                    i += 2;
                }
                b'"' => {
                    result.push('"');
                    i += 2;
                }
                _ => {
                    // If it's not a recognized escape sequence, keep the backslash
                    result.push(bytes[i] as char);
                    i += 1;
                }
            }
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }

    result
}

pub struct Compiled_Syntax_Node {
    pub id: u32,
    pub typ: NodeType,
    pub pointer: u32,
    pub cf: Vec<f32>,
    pub edges: Option<Vec<Compiled_Syntax_Edge>>,
}

pub struct Compiled_Syntax_Edge {
    pub probability: f32,
    pub option: Arc<Compiled_Syntax_Node>,
}
