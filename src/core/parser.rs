use crate::core::{
    graph::{NodeType, SyntaxGraph, SyntaxNode},
    regex::Regexer,
    scanner::{Token, TokenType},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct Parser {
    func_ptr: u32,
    print_ptr: u32,
    name_map: HashMap<String, u32>,
    rev_name_map: HashMap<u32, String>,
    def_check: HashMap<u32, bool>,
    charmap: HashMap<u32, String>,
    inter_rep: HashMap<u32, Vec<Token>>,
    tokens: Vec<Token>,
    errors: Vec<String>,
    index: usize,
    graph: SyntaxGraph,
    regexhandler: Regexer,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            print_ptr: u32::MAX / 2, // grows downward
            func_ptr: 1000,          // IDs below 1000 reserved for core graph nodes
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
        self.print_ptr -= 1;
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
        self.def_check.insert(value, false); // to be set back to true in the subject
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
        if *self.def_check.get(&id).unwrap_or(&false) {
            self.errors
                .push(format!("Multiple definitions for {}", subject.text));
        }

        self.def_check.insert(id, true);
        let startnode = self.graph.force_get_node(0, NodeType::START); // assuming 0 is start
        let headernode = self.graph.force_get_node(id, NodeType::HEADER);
        startnode.lock().unwrap().add_edge(headernode, 1.0);

        if self.index > 0 && self.match_token(self.tokens[self.index - 1].typ, &[TokenType::Colon])
        {
            self.parse_rules(id, false);
        }
    }

    fn parse_rules(
        &mut self,
        root: u32,
        is_deep: bool,
    ) -> (
        Option<Arc<Mutex<SyntaxNode>>>,
        Option<Arc<Mutex<SyntaxNode>>>,
    ) {
        let rootnode = self.graph.force_get_node(root, NodeType::IDK);
        let mut buffer_node = Arc::clone(&rootnode);
        let mut end_node = self.graph.force_get_node(1, NodeType::END); // assuming 1 is end
        let mut start_buffer: Option<Arc<Mutex<SyntaxNode>>> = None;

        if is_deep {
            let ptr = self.get_func_ptr();
            end_node = self.graph.force_get_node(ptr, NodeType::END);
        }

        loop {
            if self.index >= self.tokens.len() {
                break;
            }

            match self.curr().typ {
                TokenType::Identifier => {
                    let node = self.tokens[self.index].text.clone();
                    let pointer_id = self.get_index(&node);
                    let ptr = self.get_func_ptr();
                    let node = self.graph.force_get_node(ptr, NodeType::POINTER);
                    node.lock().unwrap().pointer = pointer_id;

                    let probability = self.get_probability();
                    buffer_node
                        .lock()
                        .unwrap()
                        .add_edge(Arc::clone(&node), probability);
                    let ptr = self.get_func_ptr();
                    let jump_node = self.graph.force_get_node(ptr, NodeType::JUMP);
                    node.lock().unwrap().add_edge(Arc::clone(&jump_node), 1.0);

                    start_buffer = Some(Arc::clone(&buffer_node));
                    buffer_node = jump_node;
                }
                TokenType::Character | TokenType::Regex => {
                    let index = self.get_print_ptr();
                    self.charmap
                        .insert(index, self.tokens[self.index].text.clone());

                    let node_type = if self.tokens[self.index].typ == TokenType::Character {
                        NodeType::CH
                    } else {
                        let text = self.curr().text.clone();
                        self.regexhandler.cache_regex(&text);
                        NodeType::RX
                    };

                    let leafnode = self.graph.force_get_node(index, node_type);
                    let probability = self.get_probability();
                    buffer_node
                        .lock()
                        .unwrap()
                        .add_edge(Arc::clone(&leafnode), probability);
                    let ptr = self.get_func_ptr();
                    let jump_node = self.graph.force_get_node(ptr, NodeType::JUMP);
                    leafnode
                        .lock()
                        .unwrap()
                        .add_edge(Arc::clone(&jump_node), 1.0);

                    start_buffer = Some(Arc::clone(&buffer_node));
                    buffer_node = jump_node;
                }
                TokenType::Colon => {
                    self.errors.push("Missing Semicolon".to_string());
                    return (None, None);
                }
                TokenType::Maybe => {
                    if let Some(ref sb) = start_buffer {
                        let probability = self.get_probability();
                        sb.lock()
                            .unwrap()
                            .add_edge(Arc::clone(&buffer_node), 1.0 - probability);
                    }
                }
                TokenType::OneOrMore => {
                    if let Some(ref sb) = start_buffer {
                        let probability = self.get_probability();
                        buffer_node
                            .lock()
                            .unwrap()
                            .add_edge(Arc::clone(sb), probability);
                    }
                }
                TokenType::AnyNo => {
                    if let Some(ref sb) = start_buffer {
                        let probability = self.get_probability();
                        sb.lock()
                            .unwrap()
                            .add_edge(Arc::clone(&buffer_node), 1.0 - probability);
                        buffer_node
                            .lock()
                            .unwrap()
                            .add_edge(Arc::clone(sb), probability);
                    }
                }
                TokenType::Option => {
                    let probability = self.get_probability();
                    buffer_node
                        .lock()
                        .unwrap()
                        .add_edge(Arc::clone(&end_node), probability);
                    buffer_node = Arc::clone(&rootnode);
                    start_buffer = None;
                }
                TokenType::Padding => {
                    buffer_node
                        .lock()
                        .unwrap()
                        .add_edge(Arc::clone(&end_node), 1.0);
                    if is_deep {
                        self.errors.push("Stray '('".to_string());
                    }
                    self.index += 1;
                    return (None, None);
                }
                TokenType::BracOpen => {
                    self.index += 1;
                    let (new_start, new_end) =
                        self.parse_rules(buffer_node.lock().unwrap().id, true);
                    start_buffer = new_start;
                    if let Some(end) = new_end {
                        buffer_node = end;
                    }
                }
                TokenType::BracClose => {
                    if is_deep {
                        buffer_node
                            .lock()
                            .unwrap()
                            .add_edge(Arc::clone(&end_node), 1.0);
                        return (Some(rootnode), Some(end_node));
                    }
                    self.errors.push("Stray ')' found".to_string());
                }
                TokenType::Infinite => {
                    if let Some(ref sb) = start_buffer {
                        end_node.lock().unwrap().add_edge(Arc::clone(sb), 1.0);
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
    }

    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }
}
