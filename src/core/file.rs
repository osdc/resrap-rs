use crate::core::{frozen_graph::FrozenSyntaxGraph, graph_builder::GraphBuilder};
use std::{path::Path, sync::Arc};

pub struct Lang {
    graph: Option<Arc<FrozenSyntaxGraph>>,
}

impl Lang {
    pub fn new() -> Self {
        Lang { graph: None }
    }

    pub fn get_graph(&self) -> Option<&FrozenSyntaxGraph> {
        self.graph.as_deref()
    }

    /// Returns a cheap, shared snapshot of the parsed graph. Used by the
    /// worker pool so each job does not need to re-acquire the grammar map.
    pub(crate) fn get_graph_arc(&self) -> Option<Arc<FrozenSyntaxGraph>> {
        self.graph.clone()
    }

    pub fn parse_file<P: AsRef<Path>>(&mut self, filename: P) -> Result<(), String> {
        // The scanner understands comments and @words directives, so read the
        // file as a whole instead of reassembling statements line by line.
        let data = std::fs::read_to_string(filename)
            .map_err(|err| format!("Failed to read file: {}", err))?;
        self.parse_string(&data)
    }

    pub fn parse_string(&mut self, data: &str) -> Result<(), String> {
        let mut gb = GraphBuilder::new();
        gb.start_generation(data)?;
        self.graph = Some(Arc::new(gb.take_graph()));
        Ok(())
    }
}

impl Default for Lang {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_whole_file_with_comments_and_words() {
        let dir = std::env::temp_dir().join(format!("resrap-file-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("grammar.g4");
        std::fs::write(
            &path,
            "# a comment\n@words identifier = hello, world\nprogram: identifier ';';\n",
        )
        .unwrap();

        let mut lang = Lang::new();
        lang.parse_file(&path).unwrap();
        assert!(lang.get_graph().is_some());
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn missing_file_is_an_error() {
        let mut lang = Lang::new();
        assert!(lang.parse_file("/nonexistent/grammar.g4").is_err());
    }
}
