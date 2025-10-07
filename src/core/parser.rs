use crate::core::graph::{NodeType, SyntaxGraph, SyntaxNode};
use crate::core::regex::Regexer;
use crate::core::scanner::{Token, TokenType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Parser {
    func_ptr: u32,
    print_ptr: u32,
    name_map: HashMap<String, u32>,     // Maps func names to their ids
    rev_name_map: HashMap<u32, String>, // Help in debugging
    def_check: HashMap<u32, bool>,      // To check if a function exists
    charmap: HashMap<u32, String>,      // To store the print values corresponding to ids
    inter_rep: HashMap<u32, Vec<Token>>, // Intermediate Representation
    tokens: Vec<Token>,
    errors: Vec<String>,
    index: usize,
    graph: SyntaxGraph,
    regexhandler: Regexer,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            print_ptr: u32::MAX, // grows downward
            func_ptr: 1000,      // IDs below 1000 reserved for core graph nodes
            name_map: HashMap::new(),
            rev_name_map: HashMap::new(),
            def_check: HashMap::new(),
            charmap: HashMap::new(),
            inter_rep: HashMap::new(),
            tokens: Vec::new(),
            errors: Vec::new(),
            index: 0,
            graph: SyntaxGraph::new(),
            regexhandler: Regexer::new(),
        }
    }

    fn get_print_ptr(&mut self) -> u32 {
        self.print_ptr = self.print_ptr.wrapping_sub(1);
        self.print_ptr
    }

    fn get_func_ptr(&mut self) -> u32 {
        self.func_ptr += 1;
        self.func_ptr
    }

    fn curr(&self) -> &Token {
        &self.tokens[self.index]
    }

    fn match_token(&self, word: TokenType, expected: &[TokenType]) -> bool {
        expected.contains(&word)
    }

    fn expect(&mut self, expected: &[TokenType], errmsg: &str) -> bool {
        if !self.match_token(self.curr().typ, expected) {
            self.errors.push(errmsg.to_string());
            self.index += 1;
            return true;
        }
        self.index += 1;
        false
    }

    fn get_index(&mut self, name: &str) -> u32 {
        if let Some(&value) = self.name_map.get(name) {
            return value;
        }

        let value = self.get_func_ptr();
        self.def_check.insert(value, false); // tb set back to true in the subject, else remain false
        self.name_map.insert(name.to_string(), value);
        self.rev_name_map.insert(value, name.to_string());
        value
    }

    pub fn parse_grammar(&mut self) {
        while self.index < self.tokens.len() {
            self.parse_subject();
            if !self.errors.is_empty() {
                return; // Crash on errors for now
            }
        }
    }

    fn parse_subject(&mut self) {
        let subject = self.curr().clone();

        if self.expect(
            &[TokenType::Identifier],
            "Expected Subject at start of statement",
        ) {
            return;
        }
        if self.expect(&[TokenType::Colon], "Expected Colon after Subject") {
            return;
        }

        let id = self.get_index(&subject.text);

        // If map is already set to true
        if *self.def_check.get(&id).unwrap_or(&false) {
            self.errors
                .push(format!("Multiple definitions for {}", subject.text));
        }

        self.def_check.insert(id, true);

        let mut startnode = self.graph.get_node(NodeType::START as u32, NodeType::START);
        let header_node = self.graph.get_node(id, NodeType::HEADER);
        startnode.add_edge(header_node, 1.0);

        // Send here only if current is colon else crash code
        if self.match_token(self.tokens[self.index - 1].typ, &[TokenType::Colon]) {
            self.parse_rules(id, false);
        }
    }

    fn parse_rules(
        &mut self,
        root: u32,
        is_deep: bool,
    ) -> (Option<Arc<SyntaxNode>>, Option<Arc<SyntaxNode>>) {
        let rootnode = self.graph.get_node(root, NodeType::IDK);
        let mut buffer_node = Arc::clone(&rootnode);
        let mut end_node = self.graph.get_node(NodeType::END as u32, NodeType::END);
        let mut start_buffer: Option<Arc<SyntaxNode>> = None;

        if is_deep {
            // Means called from a bracket so a pseudo end branch
            end_node = self.graph.get_node(self.get_func_ptr(), NodeType::END);
        }

        loop {
            if self.index >= self.tokens.len() {
                break;
            }

            match self.curr().typ {
                TokenType::Identifier => {
                    // Means it's a reference to a different Subject (presumably)
                    let pointer_id = self.get_index(&self.tokens[self.index].text);
                    let func_ptr = self.get_func_ptr();
                    let mut node = self.graph.get_node(func_ptr, NodeType::POINTER);
                    node.add_pointer(pointer_id);
                    buffer_node.add_edge(
                        &self.graph,
                        Arc::clone(&node),
                        self.get_probability() as f64,
                    );
                    let jump_node = self.graph.get_node(self.get_func_ptr(), NodeType::JUMP);
                    node.add_edge(&self.graph, Arc::clone(&jump_node), 1.0);
                    start_buffer = Some(buffer_node);
                    buffer_node = jump_node;
                }
                TokenType::Character | TokenType::Regex => {
                    let index = self.get_print_ptr();
                    self.charmap
                        .insert(index, self.tokens[self.index].text.clone());

                    let leafnode = if self.tokens[self.index].typ == TokenType::Character {
                        self.graph.get_node(index, NodeType::CH)
                    } else {
                        let node = self.graph.get_node(index, NodeType::RX);
                        self.regexhandler.cache_regex(&self.curr().text);
                        node
                    };

                    buffer_node.add_edge(
                        &self.graph,
                        Arc::clone(&leafnode),
                        self.get_probability() as f64,
                    );
                    let jump_node = self.graph.get_node(self.get_func_ptr(), NodeType::JUMP);
                    leafnode.add_edge(&self.graph, Arc::clone(&jump_node), 1.0);
                    start_buffer = Some(buffer_node);
                    buffer_node = jump_node;
                }
                TokenType::Colon => {
                    // Colon is not allowed here
                    self.errors.push("Missing Semicolon".to_string());
                    return (None, None);
                }
                TokenType::Maybe => {
                    if let Some(ref start_buf) = start_buffer {
                        start_buf.add_edge(
                            &self.graph,
                            Arc::clone(&buffer_node),
                            1.0 - self.get_probability() as f64,
                        );
                    }
                }
                TokenType::OneOrMore => {
                    if let Some(ref start_buf) = start_buffer {
                        buffer_node.add_edge(
                            &self.graph,
                            Arc::clone(start_buf),
                            self.get_probability() as f64,
                        );
                    }
                }
                TokenType::AnyNo => {
                    if let Some(ref start_buf) = start_buffer {
                        start_buf.add_edge(
                            &self.graph,
                            Arc::clone(&buffer_node),
                            1.0 - self.get_probability() as f64,
                        );
                        buffer_node.add_edge(
                            &self.graph,
                            Arc::clone(start_buf),
                            self.get_probability() as f64,
                        );
                    }
                }
                TokenType::Option => {
                    buffer_node.add_edge(
                        &self.graph,
                        Arc::clone(&end_node),
                        self.get_probability() as f64,
                    );
                    buffer_node = Arc::clone(&rootnode);
                    start_buffer = None;
                }
                TokenType::Padding => {
                    buffer_node.add_edge(&self.graph, Arc::clone(&end_node), 1.0);
                    if is_deep {
                        self.errors.push("Stray '('".to_string());
                    }
                    self.index += 1;
                    return (None, None); // End of this statement
                }
                TokenType::BracOpen => {
                    self.index += 1;
                    let (start_buf, buf_node) = self.parse_rules(buffer_node.id, true);
                    start_buffer = start_buf;
                    if let Some(node) = buf_node {
                        buffer_node = node;
                    }
                }
                TokenType::BracClose => {
                    if is_deep {
                        buffer_node.add_edge(&self.graph, Arc::clone(&end_node), 1.0);
                        return (Some(rootnode), Some(end_node));
                    }
                    self.errors.push("Stray ')' found".to_string());
                }
                TokenType::Infinite => {
                    if let Some(ref start_buf) = start_buffer {
                        end_node.add_edge(&self.graph, Arc::clone(start_buf), 1.0);
                    }
                }
                _ => {}
            }
            self.index += 1;
        }

        (start_buffer, Some(buffer_node))
    }

    fn get_probability(&mut self) -> f32 {
        self.index += 1;

        if self.index < self.tokens.len() && self.tokens[self.index].typ == TokenType::Probability {
            let num = &self.tokens[self.index].text;
            match num.parse::<f32>() {
                Ok(numf) => {
                    if numf < 0.0 {
                        self.errors.push("Negative Probability Found".to_string());
                        return 0.0;
                    }
                    return numf;
                }
                Err(_) => {
                    self.index -= 1;
                    self.errors.push("Failed to parse probability".to_string());
                    return 0.0;
                }
            }
        }

        self.index -= 1; // Reverting
        0.5
    }

    pub fn validate_graph(&self) -> Vec<String> {
        if !self.errors.is_empty() {
            return vec![];
        }

        let mut errors = Vec::new();
        for (key, val) in &self.def_check {
            if !val {
                if let Some(name) = self.rev_name_map.get(key) {
                    errors.push(format!("Definition of '{}' not found", name));
                }
            }
        }
        errors
    }

    pub fn set_tokens(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
        self.index = 0;
    }

    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }

    pub fn get_graph(&self) -> &SyntaxGraph {
        &self.graph
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
