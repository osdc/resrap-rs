use std::{collections::HashMap, sync::Arc};

use crate::core::{
    compiled_graph::{Compiled_Graph, Compiled_Syntax_Edge, Compiled_Syntax_Node},
    regex::Regexer,
};

#[derive(Clone)]
pub enum NodeType {
    START,
    HEADER,
    JUMP,
    END,
    CH,
    RX,
    POINTER,
    IDK,
}

pub struct SyntaxEdge {
    pub probability: f32,
    pub target_id: u32, // Store ID instead of reference
}

pub struct SyntaxNode {
    pub id: u32,
    pub typ: NodeType,
    pub pointer: u32,
    pub cf: Vec<f32>,
    pub edges: Vec<SyntaxEdge>,
}

impl SyntaxNode {
    pub fn add_edge(&mut self, target_id: u32, prob: f64) {
        let edge = SyntaxEdge {
            probability: prob as f32,
            target_id,
        };
        self.edges.push(edge);
    }

    pub fn add_pointer(&mut self, id: u32) {
        self.pointer = id;
    }
}

pub struct SyntaxGraph {
    pub node_ref: HashMap<u32, Box<SyntaxNode>>,
    pub name_map: HashMap<String, u32>,
    pub char_map: HashMap<u32, String>,
}

impl SyntaxGraph {
    pub fn new() -> Self {
        SyntaxGraph {
            node_ref: HashMap::new(),
            name_map: HashMap::new(),
            char_map: HashMap::new(),
        }
    }

    pub fn normalize(&mut self) {
        // Collect all node IDs first to avoid borrow checker issues
        let node_ids: Vec<u32> = self.node_ref.keys().copied().collect();

        for id in node_ids {
            if let Some(node) = self.node_ref.get_mut(&id) {
                // Extract the probabilities
                let mut cf_array: Vec<f32> = Vec::new();
                let mut sum: f32 = 0.0;

                for edge in &node.edges {
                    cf_array.push(edge.probability);
                    sum += edge.probability;
                }

                // Avoid division by zero
                if sum == 0.0 {
                    continue;
                }

                // Divide each element by the sum and convert to cumulative frequency
                let mut cf: f32 = 0.0;
                for i in 0..cf_array.len() {
                    cf_array[i] = cf + cf_array[i] / sum;
                    cf = cf_array[i];
                }

                // Now update the node's cf field
                node.cf = cf_array;
            }
        }
    }

    pub fn compile(self) -> Compiled_Graph {
        // Pass 1: Create all nodes without edges
        let mut compiled_nodes: HashMap<u32, Arc<Compiled_Syntax_Node>> = HashMap::new();

        for (id, node) in &self.node_ref {
            let compiled_node = Arc::new(Compiled_Syntax_Node {
                id: node.id,
                typ: node.typ.clone(),
                pointer: node.pointer,
                cf: node.cf.clone(),
                edges: None, // Will fill in pass 2
            });
            compiled_nodes.insert(*id, compiled_node);
        }

        // Pass 2: Fill in edges now that all nodes exist
        let mut final_nodes: HashMap<u32, Arc<Compiled_Syntax_Node>> = HashMap::new();

        for (id, node) in &self.node_ref {
            let compiled_edges: Vec<Compiled_Syntax_Edge> = node
                .edges
                .iter()
                .map(|edge| {
                    let target_node = compiled_nodes
                        .get(&edge.target_id)
                        .expect("Target node should exist")
                        .clone();

                    Compiled_Syntax_Edge {
                        probability: edge.probability,
                        option: target_node,
                    }
                })
                .collect();

            let final_node = Arc::new(Compiled_Syntax_Node {
                id: node.id,
                typ: node.typ.clone(),
                pointer: node.pointer,
                cf: node.cf.clone(),
                edges: Some(compiled_edges),
            });

            final_nodes.insert(*id, final_node);
        }

        Compiled_Graph {
            node_ref: final_nodes,
            name_map: self.name_map,
            char_map: self.char_map,
            regexer: Regexer::new(),
        }
    }

    pub fn get_node(&mut self, id: u32, typ: NodeType) -> &mut Box<SyntaxNode> {
        self.node_ref.entry(id).or_insert_with(|| {
            Box::new(SyntaxNode {
                id,
                typ,
                pointer: 0,
                cf: vec![],
                edges: vec![],
            })
        })
    }
}

impl Default for SyntaxGraph {
    fn default() -> Self {
        Self::new()
    }
}
