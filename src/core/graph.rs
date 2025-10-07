use std::{
    clone,
    collections::HashMap,
    hash::Hash,
    io::Error,
    sync::{Arc, Mutex},
};

use crate::core::{
    frozen_graph::{FrozenSyntaxEdge, FrozenSyntaxGraph, FrozenSyntaxNode},
    regex::Regexer,
};

pub struct SyntaxGraph {
    node_ref: HashMap<u32, Arc<Mutex<SyntaxNode>>>,
    name_map: HashMap<String, u32>,
    print_map: HashMap<u32, String>,
    regexer: Regexer,
}

pub struct SyntaxNode {
    pub options: Vec<SyntaxEdge>,
    pub cumulative_frequency: Vec<f32>,
    pub id: u32,
    pub typ: NodeType,
    pub pointer: u32,
}
struct SyntaxEdge {
    pub probability: f32,
    pub node: Arc<Mutex<SyntaxNode>>,
}
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
impl SyntaxGraph {
    pub fn new() -> Self {
        SyntaxGraph {
            node_ref: HashMap::new(),
            name_map: HashMap::new(),
            print_map: HashMap::new(),
            regexer: Regexer::new(),
        }
    }
    pub fn force_get_node(&mut self, id: u32, typ: NodeType) -> Arc<Mutex<SyntaxNode>> {
        //Will find or create node if not exists. Never fails
        if let Some(node) = self.node_ref.get(&id) {
            Arc::clone(node)
        } else {
            let node = SyntaxNode {
                id,
                typ,
                options: vec![],
                cumulative_frequency: vec![],
                pointer: 0,
            };

            let node = Arc::new(Mutex::new(node));
            self.node_ref.insert(id, Arc::clone(&node));
            node
        }
    }
    fn fetch_node(&self, id: u32) -> Result<Arc<Mutex<SyntaxNode>>, &str> {
        if let Some(node) = self.node_ref.get(&id) {
            Ok(Arc::clone(node))
        } else {
            Err("Node not found in graph")
        }
    }
    fn normalize(&self) {
        for node in self.node_ref.values() {
            let mut node_guard = node.lock().unwrap();

            // Collect probabilities
            let mut CF: Vec<f32> = vec![];
            let mut cf: f32 = 0.0;
            let mut sum: f32 = 0.0;
            for edge in &node_guard.options {
                CF.push(edge.probability);
                sum += edge.probability;
            }

            // Build cumulative frequency
            for i in 0..CF.len() {
                CF[i] = cf + CF[i] / sum;
                cf = CF[i];
            }

            node_guard.cumulative_frequency = CF;
            //Now we have a cool little CF Array Normalized to 1
            // When picking random values, we pick one between 0 and 1
            // And then choose its closest value from the array
            // For probability based selection
        }
    }

    fn freeze(self) -> FrozenSyntaxGraph {
        // Step 1: Create all nodes (no edges)
        let mut frozen_nodes: HashMap<u32, Arc<FrozenSyntaxNode>> = HashMap::new();

        for (&id, node_arc) in &self.node_ref {
            let node_guard = node_arc.lock().unwrap();
            frozen_nodes.insert(
                id,
                Arc::new(FrozenSyntaxNode {
                    id: node_guard.id,
                    typ: node_guard.typ.clone(),
                    pointer: node_guard.pointer,
                    cumulative_frequency: node_guard.cumulative_frequency.clone(),
                    options: vec![], // fill later
                }),
            );
        }

        // Step 2: Rebuild with filled options
        let mut filled_nodes: HashMap<u32, Arc<FrozenSyntaxNode>> = HashMap::new();

        for (&id, node_arc) in &self.node_ref {
            let node_guard = node_arc.lock().unwrap();

            let options = node_guard
                .options
                .iter()
                .map(|edge| {
                    let target_node = frozen_nodes.get(&edge.node.lock().unwrap().id).unwrap();
                    FrozenSyntaxEdge {
                        probability: edge.probability,
                        node: Arc::clone(target_node),
                    }
                })
                .collect::<Vec<_>>();

            filled_nodes.insert(
                id,
                Arc::new(FrozenSyntaxNode {
                    id: node_guard.id,
                    typ: node_guard.typ.clone(),
                    pointer: node_guard.pointer,
                    cumulative_frequency: node_guard.cumulative_frequency.clone(),
                    options,
                }),
            );
        }

        FrozenSyntaxGraph {
            node_ref: filled_nodes,
            name_map: self.name_map,
            print_map: self.print_map,
            regexer: self.regexer,
        }
    }
}

impl SyntaxNode {
    pub fn add_edge(&mut self, node: Arc<Mutex<SyntaxNode>>, probability: f32) {
        self.options.push(SyntaxEdge { probability, node });
    }
}
