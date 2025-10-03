use std::{collections::HashMap, sync::Arc, vec};

use petgraph::graph::DiGraph;

use crate::core::prng::PRNG;
enum NodeType {
    START,
    HEADER,
    JUMP,
    END,
    CH,
    RX,
    POINTER,
    IDK,
}
pub struct SyntaxNode {
    id: u32,
    typ: NodeType,
    pointer: u32,
    cf: Vec<f32>,
}

struct SyntaxGraph {
    node_ref: HashMap<u32, Arc<SyntaxNode>>,
    name_map: HashMap<String, u32>,
    char_map: HashMap<u32, String>,
    prng: PRNG,
    graph: DiGraph<SyntaxNode, ()>,
}
