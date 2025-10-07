use crate::core::frozen_graph::FrozenSyntaxGraph;
use crate::core::graph_builder::GraphBuilder;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// ParseFile reads a file and returns statements split by lines ending with ';',
/// skipping lines starting with "//".
fn parse_file<P: AsRef<Path>>(filename: P) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut statements = Vec::new();
    let mut current = String::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip comment or empty lines
        if line.starts_with("//") || line.is_empty() {
            continue;
        }

        // Accumulate this line
        current.push_str(line);
        current.push(' ');

        // If line ends with ';', finalize statement
        if line.ends_with(';') {
            let stmt = current.trim().to_string();
            statements.push(stmt);
            current.clear();
        }
    }

    Ok(statements)
}

pub struct Lang {
    graph: Option<FrozenSyntaxGraph>,
}

impl Lang {
    pub fn new() -> Self {
        Lang { graph: None }
    }

    pub fn get_graph(&self) -> Option<&FrozenSyntaxGraph> {
        self.graph.as_ref()
    }

    pub fn parse_file<P: AsRef<Path>>(&mut self, filename: P) -> Result<(), String> {
        let lines = parse_file(filename).map_err(|e| format!("Failed to read file: {}", e))?;

        let mut gb = GraphBuilder::new();
        let content = lines.join("");
        gb.start_generation(content)?;

        self.graph = Some(gb.take_graph());
        Ok(())
    }

    pub fn parse_string(&mut self, data: String) -> Result<(), String> {
        let mut gb = GraphBuilder::new();
        gb.start_generation(data)?;

        self.graph = Some(gb.take_graph());
        Ok(())
    }
}

impl Default for Lang {
    fn default() -> Self {
        Self::new()
    }
}
