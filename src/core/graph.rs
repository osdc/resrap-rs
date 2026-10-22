use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::core::{
    frozen_graph::{FrozenSyntaxEdge, FrozenSyntaxGraph, FrozenSyntaxNode},
    regex::Regexer,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum NodeType {
    START,
    HEADER,
    JUMP,
    END,
    CH,
    RX,
    POINTER,
    IDK,
    LOOP,
    GROUP_END,
}

pub struct SyntaxGraph {
    pub node_ref: HashMap<u32, Arc<Mutex<SyntaxNode>>>,
    pub name_map: HashMap<String, u32>,
    pub print_map: HashMap<u32, String>,
    pub regexer: Regexer,
    pub words: HashMap<u32, Vec<String>>,
    pub rule_ends: HashMap<u32, u32>,
}

pub struct SyntaxNode {
    pub options: Vec<SyntaxEdge>,
    pub cumulative_frequency: Vec<f32>,
    pub id: u32,
    pub typ: NodeType,
    pub pointer: u32,
}

pub struct SyntaxEdge {
    pub probability: f32,
    pub node: Arc<Mutex<SyntaxNode>>,
}

impl SyntaxGraph {
    pub fn new() -> Self {
        SyntaxGraph {
            node_ref: HashMap::new(),
            name_map: HashMap::new(),
            print_map: HashMap::new(),
            regexer: Regexer::new(),
            words: HashMap::new(),
            rule_ends: HashMap::new(),
        }
    }

    /// Finds or creates a node. Never fails.
    pub fn force_get_node(&mut self, id: u32, typ: NodeType) -> Arc<Mutex<SyntaxNode>> {
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

    pub fn normalize(&self) {
        for node in self.node_ref.values() {
            let mut node_guard = node.lock().unwrap();

            let mut frequencies: Vec<f32> = Vec::with_capacity(node_guard.options.len());
            let mut sum: f32 = 0.0;
            for edge in &node_guard.options {
                let mut p = edge.probability;
                if p < 0.0 {
                    p = 0.0;
                }
                frequencies.push(p);
                sum += p;
            }

            // A node with zero/NaN weights should still be traversable. Treat
            // it as a uniform choice instead of producing NaN cumulative
            // frequencies.
            if sum.is_nan() || sum <= 0.0 {
                frequencies.fill(1.0);
                sum = frequencies.len() as f32;
            }

            let mut cf: f32 = 0.0;
            for f in &mut frequencies {
                *f = cf + *f / sum;
                cf = *f;
            }

            node_guard.cumulative_frequency = frequencies;
        }
    }

    pub fn freeze(self) -> FrozenSyntaxGraph {
        // Step 1: create all nodes without edges.
        let mut frozen_nodes: HashMap<u32, Arc<FrozenSyntaxNode>> = HashMap::new();

        for (&id, node_arc) in &self.node_ref {
            let node_guard = node_arc.lock().unwrap();
            frozen_nodes.insert(
                id,
                Arc::new(FrozenSyntaxNode {
                    id: node_guard.id,
                    typ: node_guard.typ,
                    pointer: node_guard.pointer,
                    cumulative_frequency: node_guard.cumulative_frequency.clone(),
                    options: vec![], // filled in step 2
                }),
            );
        }

        // Step 2: rebuild with filled options.
        let mut filled_nodes: HashMap<u32, Arc<FrozenSyntaxNode>> = HashMap::new();

        for (&id, node_arc) in &self.node_ref {
            // Collect target handles while holding the lock only briefly.
            // Locking the target inside this block would deadlock on self-edges
            // such as `item*` / `item+`.
            let (node_id, typ, pointer, cumulative_frequency, targets) = {
                let node_guard = node_arc.lock().unwrap();
                (
                    node_guard.id,
                    node_guard.typ,
                    node_guard.pointer,
                    node_guard.cumulative_frequency.clone(),
                    node_guard
                        .options
                        .iter()
                        .map(|edge| Arc::clone(&edge.node))
                        .collect::<Vec<_>>(),
                )
            };

            let options = targets
                .into_iter()
                .filter_map(|target| {
                    let target_id = target.lock().unwrap().id;
                    let _ = frozen_nodes.get(&target_id)?;
                    Some(FrozenSyntaxEdge { node: target_id })
                })
                .collect::<Vec<_>>();

            filled_nodes.insert(
                id,
                Arc::new(FrozenSyntaxNode {
                    id: node_id,
                    typ,
                    pointer,
                    cumulative_frequency,
                    options,
                }),
            );
        }

        FrozenSyntaxGraph::new(
            filled_nodes,
            self.name_map,
            self.print_map,
            self.regexer,
            self.words,
            self.rule_ends,
        )
    }
}

impl Default for SyntaxGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SyntaxNode {
    pub fn add_edge(&mut self, node: Arc<Mutex<SyntaxNode>>, probability: f32) {
        self.options.push(SyntaxEdge { probability, node });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_cumulative_frequencies() {
        let mut g = SyntaxGraph::new();
        let a = g.force_get_node(1, NodeType::HEADER);
        let b = g.force_get_node(2, NodeType::HEADER);
        let c = g.force_get_node(3, NodeType::HEADER);
        {
            let mut a = a.lock().unwrap();
            a.add_edge(Arc::clone(&b), 1.0);
            a.add_edge(Arc::clone(&c), 3.0);
        }
        g.normalize();
        let a = a.lock().unwrap();
        assert_eq!(a.cumulative_frequency, vec![0.25, 1.0]);
    }

    #[test]
    fn normalize_zero_sum_uses_uniform() {
        let mut g = SyntaxGraph::new();
        let a = g.force_get_node(1, NodeType::HEADER);
        let b = g.force_get_node(2, NodeType::HEADER);
        let c = g.force_get_node(3, NodeType::HEADER);
        {
            let mut a = a.lock().unwrap();
            a.add_edge(Arc::clone(&b), 0.0);
            a.add_edge(Arc::clone(&c), 0.0);
        }
        g.normalize();
        let a = a.lock().unwrap();
        assert_eq!(a.cumulative_frequency, vec![0.5, 1.0]);
    }

    #[test]
    fn leaf_has_no_cumulative_frequency() {
        let mut g = SyntaxGraph::new();
        let a = g.force_get_node(1, NodeType::HEADER);
        g.normalize();
        assert!(a.lock().unwrap().cumulative_frequency.is_empty());
    }

    #[test]
    fn force_get_node_reuses_existing_type() {
        let mut g = SyntaxGraph::new();
        let a = g.force_get_node(1, NodeType::CH);
        let b = g.force_get_node(1, NodeType::HEADER);
        assert!(Arc::ptr_eq(&a, &b));
        assert_eq!(b.lock().unwrap().typ, NodeType::CH);
    }
}
