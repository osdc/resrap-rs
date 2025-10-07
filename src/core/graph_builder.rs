use std::{collections::HashMap, error, hash::Hash};

use petgraph::{Graph, graph::Frozen};

use crate::core::{
    frozen_graph::FrozenSyntaxGraph,
    parser::Parser,
    regex::Regexer,
    scanner::{self, Scanner, Token},
};

pub struct GraphBuilder {
    grammar: String,
    pars: Parser,
    tokens: Vec<Token>,
    frozen: FrozenSyntaxGraph,
}
impl GraphBuilder {
    pub fn new() -> Self {
        GraphBuilder {
            grammar: String::from(""),
            pars: Parser::new(),
            tokens: vec![],
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
            self.pars.graph.print_map = self.pars.charmap.clone();
            self.pars.graph.regexer = self.pars.regexhandler.clone();
            self.pars.parse_grammar();
            self.pars.graph.normalize();
            self.frozen = self.pars.graph.clone().freeze();
            if !self.pars.errors.is_empty() {
                Err(String::from(&self.pars.errors[0]))
            } else {
                return Ok(());
            }
        }
    }
}
