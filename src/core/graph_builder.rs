use std::collections::HashMap;

use crate::core::{
    frozen_graph::FrozenSyntaxGraph, parser::Parser, regex::Regexer, scanner::Scanner,
};

pub struct GraphBuilder {
    pars: Parser,
    frozen: FrozenSyntaxGraph,
}
impl GraphBuilder {
    pub fn new() -> Self {
        GraphBuilder {
            pars: Parser::new(),
            frozen: FrozenSyntaxGraph {
                node_ref: HashMap::new(),
                name_map: HashMap::new(),
                print_map: HashMap::new(),
                regexer: Regexer::new(),
            },
        }
    }
    pub fn take_graph(self) -> FrozenSyntaxGraph {
        self.frozen
    }
    pub fn start_generation(&mut self, grammar: String) -> Result<(), String> {
        let sc = Scanner::new(grammar);
        let (tokens, errors) = sc.scan();

        if !errors.is_empty() {
            Err(String::from("Scan Error"))
        } else {
            self.pars.tokens = tokens;

            self.pars.parse_grammar();
            self.pars.graph.normalize();
            self.pars.graph.print_map = self.pars.charmap.clone();
            self.pars.graph.name_map = self.pars.name_map.clone();
            self.pars.graph.regexer = self.pars.regexhandler.clone();
            self.frozen = self.pars.graph.clone().freeze();

            if !self.pars.errors.is_empty() {
                Err(String::from(&self.pars.errors[0]))
            } else {
                return Ok(());
            }
        }
    }
}
