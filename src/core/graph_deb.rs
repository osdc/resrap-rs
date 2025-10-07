use crate::core::{
    frozen_graph::FrozenSyntaxGraph,
    graph::{NodeType, SyntaxGraph},
};

impl SyntaxGraph {
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph SyntaxGraph {\n");
        dot.push_str("    rankdir=LR;\n"); // Left to right layout
        dot.push_str("    node [shape=box];\n\n");

        for (id, node_arc) in &self.node_ref {
            let node = node_arc.lock().unwrap();

            // Node styling based on type
            let (shape, color) = match node.typ {
                NodeType::START => ("diamond", "green"),
                NodeType::END => ("diamond", "red"),
                NodeType::HEADER => ("box", "lightblue"),
                NodeType::POINTER => ("ellipse", "yellow"),
                NodeType::CH => ("box", "lightgreen"),
                NodeType::RX => ("box", "orange"),
                NodeType::JUMP => ("circle", "gray"),
                _ => ("box", "white"),
            };

            // Node label
            let label = match node.typ {
                NodeType::CH | NodeType::RX => self
                    .print_map
                    .get(id)
                    .map(|s| s.replace("\"", "\\\""))
                    .unwrap_or_else(|| format!("{:?}", node.typ)),
                _ => format!("{:?}", node.typ),
            };

            dot.push_str(&format!(
                "    n{} [label=\"{}\\nid:{}\", shape={}, fillcolor={}, style=filled];\n",
                id, label, id, shape, color
            ));

            // Edges
            for edge in &node.options {
                let target_id = edge.node.lock().unwrap().id;
                let prob_label = if (edge.probability - 1.0).abs() < 0.001 {
                    String::new()
                } else {
                    format!(" [label=\"{:.2}\"]", edge.probability)
                };
                dot.push_str(&format!("    n{} -> n{}{};  \n", id, target_id, prob_label));
            }
        }

        dot.push_str("}\n");
        dot
    }

    pub fn save_dot(&self, filename: &str) -> std::io::Result<()> {
        std::fs::write(filename, self.to_dot())
    }
}

impl FrozenSyntaxGraph {
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph FrozenSyntaxGraph {\n");
        dot.push_str("    rankdir=LR;\n"); // Left to right layout
        dot.push_str("    node [shape=box];\n\n");

        for (id, node) in &self.node_ref {
            // Node styling based on type
            let (shape, color) = match node.typ {
                NodeType::START => ("diamond", "green"),
                NodeType::END => ("diamond", "red"),
                NodeType::HEADER => ("box", "lightblue"),
                NodeType::POINTER => ("ellipse", "yellow"),
                NodeType::CH => ("box", "lightgreen"),
                NodeType::RX => ("box", "orange"),
                NodeType::JUMP => ("circle", "gray"),
                _ => ("box", "white"),
            };

            // Node label
            let label = match node.typ {
                NodeType::POINTER => {
                    // Try to find the name this pointer points to
                    if let Some(name) = self
                        .name_map
                        .iter()
                        .find(|(_, v)| **v == node.pointer)
                        .map(|(k, _)| k)
                    {
                        format!("→{}", name)
                    } else {
                        format!("→{}", node.pointer)
                    }
                }
                NodeType::CH | NodeType::RX => self
                    .print_map
                    .get(id)
                    .map(|s| {
                        s.replace("\"", "\\\"")
                            .replace("\n", "\\\\n")
                            .replace("\t", "\\\\t")
                    })
                    .unwrap_or_else(|| format!("{:?}", node.typ)),
                _ => format!("{:?}", node.typ),
            };

            // Add node definition
            dot.push_str(&format!(
                "    n{} [label=\"{}\\nid:{}\", shape={}, fillcolor={}, style=filled];\n",
                id, label, id, shape, color
            ));

            // Add edges
            for (idx, edge) in node.options.iter().enumerate() {
                let target_id = edge.node.id;

                // Edge label with probability and cumulative frequency
                let prob_label = if (edge.probability - 1.0).abs() < 0.001 {
                    String::new()
                } else {
                    let cumulative = node
                        .cumulative_frequency
                        .get(idx)
                        .map(|cf| format!(" cf:{:.2}", cf))
                        .unwrap_or_default();
                    format!(
                        " [label=\"p:{:.2}{}\", fontsize=10]",
                        edge.probability, cumulative
                    )
                };

                dot.push_str(&format!("    n{} -> n{}{};\n", id, target_id, prob_label));
            }

            // Special edge for POINTER nodes to show what they point to
            if node.typ == NodeType::POINTER {
                dot.push_str(&format!(
                    "    n{} -> n{} [style=dashed, color=blue, label=\"ptr\"];\n",
                    id, node.pointer
                ));
            }
        }

        // Add legend
        dot.push_str("\n    // Legend\n");
        dot.push_str("    subgraph cluster_legend {\n");
        dot.push_str("        label=\"Legend\";\n");
        dot.push_str("        style=filled;\n");
        dot.push_str("        color=lightgrey;\n");
        dot.push_str("        node [shape=box, style=filled];\n");
        dot.push_str("        legend_start [label=\"START\", fillcolor=green, shape=diamond];\n");
        dot.push_str("        legend_end [label=\"END\", fillcolor=red, shape=diamond];\n");
        dot.push_str("        legend_header [label=\"HEADER\", fillcolor=lightblue];\n");
        dot.push_str(
            "        legend_pointer [label=\"POINTER\", fillcolor=yellow, shape=ellipse];\n",
        );
        dot.push_str("        legend_ch [label=\"CHARACTER\", fillcolor=lightgreen];\n");
        dot.push_str("        legend_rx [label=\"REGEX\", fillcolor=orange];\n");
        dot.push_str("        legend_jump [label=\"JUMP\", fillcolor=gray, shape=circle];\n");
        dot.push_str("    }\n");

        dot.push_str("}\n");
        dot
    }

    pub fn save_dot(&self, filename: &str) -> std::io::Result<()> {
        std::fs::write(filename, self.to_dot())
    }

    // Bonus: Generate a focused subgraph starting from a specific node
    pub fn to_dot_from(&self, start_name: &str, max_depth: usize) -> Result<String, &str> {
        let start_id = self
            .name_map
            .get(start_name)
            .ok_or("Start node not found")?;

        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((*start_id, 0));

        let mut dot = String::from("digraph FrozenSyntaxGraph {\n");
        dot.push_str("    rankdir=LR;\n");
        dot.push_str("    node [shape=box];\n\n");

        while let Some((node_id, depth)) = queue.pop_front() {
            if depth > max_depth || visited.contains(&node_id) {
                continue;
            }
            visited.insert(node_id);

            if let Some(node) = self.node_ref.get(&node_id) {
                // Same node styling as before
                let (shape, color) = match node.typ {
                    NodeType::START => ("diamond", "green"),
                    NodeType::END => ("diamond", "red"),
                    NodeType::HEADER => ("box", "lightblue"),
                    NodeType::POINTER => ("ellipse", "yellow"),
                    NodeType::CH => ("box", "lightgreen"),
                    NodeType::RX => ("box", "orange"),
                    NodeType::JUMP => ("circle", "gray"),
                    _ => ("box", "white"),
                };

                let label = match node.typ {
                    NodeType::POINTER => {
                        if let Some(name) = self
                            .name_map
                            .iter()
                            .find(|(_, v)| **v == node.pointer)
                            .map(|(k, _)| k)
                        {
                            format!("→{}", name)
                        } else {
                            format!("→{}", node.pointer)
                        }
                    }
                    NodeType::CH | NodeType::RX => self
                        .print_map
                        .get(&node_id)
                        .map(|s| s.replace("\"", "\\\"").replace("\n", "\\\\n"))
                        .unwrap_or_else(|| format!("{:?}", node.typ)),
                    _ => format!("{:?}", node.typ),
                };

                dot.push_str(&format!(
                    "    n{} [label=\"{}\\nid:{}\", shape={}, fillcolor={}, style=filled];\n",
                    node_id, label, node_id, shape, color
                ));

                for (idx, edge) in node.options.iter().enumerate() {
                    let target_id = edge.node.id;
                    queue.push_back((target_id, depth + 1));

                    let prob_label = if (edge.probability - 1.0).abs() < 0.001 {
                        String::new()
                    } else {
                        let cumulative = node
                            .cumulative_frequency
                            .get(idx)
                            .map(|cf| format!(" cf:{:.2}", cf))
                            .unwrap_or_default();
                        format!(" [label=\"p:{:.2}{}\"]", edge.probability, cumulative)
                    };

                    dot.push_str(&format!(
                        "    n{} -> n{}{};\n",
                        node_id, target_id, prob_label
                    ));
                }

                if node.typ == NodeType::POINTER {
                    queue.push_back((node.pointer, depth + 1));
                    dot.push_str(&format!(
                        "    n{} -> n{} [style=dashed, color=blue, label=\"ptr\"];\n",
                        node_id, node.pointer
                    ));
                }
            }
        }

        dot.push_str("}\n");
        Ok(dot)
    }
}
