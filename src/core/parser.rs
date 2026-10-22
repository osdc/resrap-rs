use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::core::{
    graph::{NodeType, SyntaxGraph, SyntaxNode},
    regex::Regexer,
    scanner::{Token, TokenType},
};

/// A node returned from parsing a rule or a bracketed group.
type RuleNode = Arc<Mutex<SyntaxNode>>;

pub struct Parser {
    pub func_ptr: u32,
    pub print_ptr: u32,
    pub name_map: HashMap<String, u32>,
    pub rev_name_map: HashMap<u32, String>,
    pub def_check: HashMap<u32, bool>,
    pub charmap: HashMap<u32, String>,
    pub words: HashMap<String, Vec<String>>,
    pub tokens: Vec<Token>,
    pub errors: Vec<String>,
    pub index: usize,
    pub graph: SyntaxGraph,
    pub regexhandler: Regexer,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            print_ptr: i32::MAX as u32, // grows downward
            func_ptr: 1000,             // ids below 1000 reserved for core nodes
            name_map: HashMap::new(),
            rev_name_map: HashMap::new(),
            def_check: HashMap::new(),
            charmap: HashMap::new(),
            words: HashMap::new(),
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

    fn curr(&self) -> Token {
        if self.index < self.tokens.len() {
            self.tokens[self.index].clone()
        } else {
            Token {
                typ: TokenType::Padding,
                text: String::new(),
                line: 0,
                col: 0,
            }
        }
    }

    fn err_at(&mut self, token: &Token, msg: String) {
        self.errors
            .push(format!("line {}:{}: {}", token.line, token.col, msg));
    }

    fn err_here(&mut self, msg: String) {
        let token = self.curr();
        self.err_at(&token, msg);
    }

    fn match_token(word: TokenType, expected: &[TokenType]) -> bool {
        expected.contains(&word)
    }

    fn expect(&mut self, expected: &[TokenType], errmsg: &str) -> bool {
        if self.index >= self.tokens.len() {
            self.errors
                .push(format!("{} (unexpected end of grammar)", errmsg));
            return true;
        }
        if !Self::match_token(self.curr().typ, expected) {
            self.err_here(errmsg.to_string());
            self.index += 1;
            return true;
        }
        self.index += 1;
        false
    }

    fn get_index(&mut self, name: &str) -> u32 {
        if let Some(value) = self.name_map.get(name) {
            return *value;
        }
        let value = self.get_func_ptr();
        self.def_check.insert(value, false);
        self.name_map.insert(name.to_string(), value);
        self.rev_name_map.insert(value, name.to_string());
        value
    }

    pub fn parse_grammar(&mut self) {
        while self.index < self.tokens.len() {
            self.parse_subject();
            if !self.errors.is_empty() {
                return; // Stop at the first broken statement.
            }
        }
    }

    fn parse_subject(&mut self) {
        let subject = self.curr();

        if self.expect(
            &[TokenType::Identifier],
            "expected subject at start of statement",
        ) {
            return;
        }
        if self.expect(&[TokenType::Colon], "expected colon after subject") {
            return;
        }

        let id = self.get_index(&subject.text);
        if *self.def_check.get(&id).unwrap_or(&false) {
            self.err_at(
                &subject,
                format!("multiple definitions for {}", subject.text),
            );
        }

        self.def_check.insert(id, true);
        let start_node = self.graph.force_get_node(0, NodeType::START);
        let header = self.graph.force_get_node(id, NodeType::HEADER);
        {
            let mut start = start_node.lock().unwrap();
            start.add_edge(header, 1.0);
        }

        // Every rule gets its own end node. Sharing one global end node made
        // the '^' operator jump between unrelated rules.
        let end_id = self.get_func_ptr();
        let end_node = self.graph.force_get_node(end_id, NodeType::END);
        self.graph.rule_ends.insert(id, end_id);

        self.parse_rules(id, false, Some(end_node));
    }

    fn parse_rules(
        &mut self,
        root: u32,
        is_deep: bool,
        end_node: Option<Arc<Mutex<SyntaxNode>>>,
    ) -> (Option<RuleNode>, Option<RuleNode>) {
        let rootnode = self.graph.force_get_node(root, NodeType::IDK);
        let mut buffer_node = Arc::clone(&rootnode);
        let end_node = match end_node {
            Some(node) => node,
            None => {
                let typ = if is_deep {
                    NodeType::GROUP_END
                } else {
                    NodeType::END
                };
                let id = self.get_func_ptr();
                self.graph.force_get_node(id, typ)
            }
        };
        let mut start_buffer: Option<Arc<Mutex<SyntaxNode>>> = None;

        while self.index < self.tokens.len() {
            let token = self.curr();
            match token.typ {
                TokenType::Identifier => {
                    let pointer_id = self.get_index(&token.text);
                    let ptr = self.get_func_ptr();
                    let node = self.graph.force_get_node(ptr, NodeType::POINTER);
                    {
                        node.lock().unwrap().pointer = pointer_id;
                    }
                    let probability = self.get_probability();
                    {
                        buffer_node
                            .lock()
                            .unwrap()
                            .add_edge(Arc::clone(&node), probability);
                    }
                    let ptr = self.get_func_ptr();
                    let jump_node = self.graph.force_get_node(ptr, NodeType::JUMP);
                    {
                        node.lock().unwrap().add_edge(Arc::clone(&jump_node), 1.0);
                    }
                    start_buffer = Some(Arc::clone(&buffer_node));
                    buffer_node = jump_node;
                }
                TokenType::Character | TokenType::Regex => {
                    let index = self.get_print_ptr();
                    self.charmap.insert(index, token.text.clone());

                    let node_type = if token.typ == TokenType::Character {
                        NodeType::CH
                    } else {
                        self.regexhandler.cache_regex(&token.text);
                        NodeType::RX
                    };

                    let leafnode = self.graph.force_get_node(index, node_type);
                    let probability = self.get_probability();
                    {
                        buffer_node
                            .lock()
                            .unwrap()
                            .add_edge(Arc::clone(&leafnode), probability);
                    }
                    let ptr = self.get_func_ptr();
                    let jump_node = self.graph.force_get_node(ptr, NodeType::JUMP);
                    {
                        leafnode
                            .lock()
                            .unwrap()
                            .add_edge(Arc::clone(&jump_node), 1.0);
                    }

                    start_buffer = Some(Arc::clone(&buffer_node));
                    buffer_node = jump_node;
                }
                TokenType::Colon => {
                    self.err_here("missing semicolon before ':'".to_string());
                    return (None, None);
                }
                TokenType::Maybe => {
                    if let Some(sb) = start_buffer.as_ref() {
                        let probability = self.get_probability();
                        sb.lock()
                            .unwrap()
                            .add_edge(Arc::clone(&buffer_node), 1.0 - probability);
                    } else {
                        self.err_here("'?' has nothing to repeat".to_string());
                        return (None, None);
                    }
                }
                TokenType::OneOrMore => {
                    if let Some(sb) = start_buffer.as_ref() {
                        let probability = self.get_probability();
                        buffer_node
                            .lock()
                            .unwrap()
                            .add_edge(Arc::clone(sb), probability);
                    } else {
                        self.err_here("'+' has nothing to repeat".to_string());
                        return (None, None);
                    }
                }
                TokenType::AnyNo => {
                    if let Some(sb) = start_buffer.as_ref() {
                        // The reference implementation calls getProbability
                        // twice here: once for the exit edge and once for the
                        // loop edge. Preserve that quirk for seed parity.
                        let exit_probability = self.get_probability();
                        sb.lock()
                            .unwrap()
                            .add_edge(Arc::clone(&buffer_node), 1.0 - exit_probability);
                        let loop_probability = self.get_probability();
                        buffer_node
                            .lock()
                            .unwrap()
                            .add_edge(Arc::clone(sb), loop_probability);
                    } else {
                        self.err_here("'*' has nothing to repeat".to_string());
                        return (None, None);
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
                        self.err_here("unexpected ';' inside group".to_string());
                        self.index += 1;
                        return (None, None);
                    }
                    self.index += 1;
                    return (None, None);
                }
                TokenType::BracOpen => {
                    self.index += 1;
                    let buffer_id = buffer_node.lock().unwrap().id;
                    let (new_start, new_end) = self.parse_rules(buffer_id, true, None);
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
                    self.err_here("stray ')' found".to_string());
                    self.index += 1;
                    return (None, None);
                }
                TokenType::Infinite => {
                    if let Some(sb) = start_buffer.as_ref() {
                        // '^' repeats the preceding construct in place. The
                        // explicit loop node lets complete generation stop at
                        // the budget and then emit what follows the loop.
                        let ptr = self.get_func_ptr();
                        let loop_node = self.graph.force_get_node(ptr, NodeType::LOOP);
                        {
                            loop_node.lock().unwrap().pointer = sb.lock().unwrap().id;
                        }
                        buffer_node
                            .lock()
                            .unwrap()
                            .add_edge(Arc::clone(&loop_node), 1.0);
                        buffer_node = loop_node;
                        start_buffer = None;
                    } else {
                        self.err_here("'^' has nothing to loop".to_string());
                        return (None, None);
                    }
                }
                TokenType::Probability => {
                    self.err_here("probability '<...>' must follow a token".to_string());
                    return (None, None);
                }
            }
            self.index += 1;
        }

        if is_deep {
            self.errors.push("unterminated '(' in grammar".to_string());
        } else {
            self.errors
                .push("unexpected end of grammar: missing ';'".to_string());
        }
        (None, None)
    }

    fn get_probability(&mut self) -> f32 {
        self.index += 1;
        if self.index >= self.tokens.len() {
            self.index -= 1;
            return 0.5;
        }

        if self.tokens[self.index].typ == TokenType::Probability {
            let token = self.tokens[self.index].clone();
            let num = token.text.as_str();
            match num.parse::<f32>() {
                Ok(numf) => {
                    if numf < 0.0 {
                        self.err_at(&token, "negative probability found".to_string());
                        self.index -= 1;
                        return 0.0;
                    }
                    if !numf.is_finite() {
                        self.err_at(&token, format!("invalid probability {:?}", num));
                        self.index -= 1;
                        return 0.0;
                    }
                    return numf;
                }
                Err(err) => {
                    self.err_at(&token, format!("invalid probability {:?}: {}", num, err));
                    self.index -= 1;
                    return 0.0;
                }
            }
        }

        self.index -= 1;
        0.5
    }

    /// Resolves `@words` rule names to graph ids.
    pub fn attach_words(&mut self) {
        let entries: Vec<(String, Vec<String>)> = self
            .words
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        for (name, words) in entries {
            let id = match self.name_map.get(&name) {
                Some(id) => *id,
                None => {
                    // A word-only rule does not need a grammar definition.
                    let id = self.get_index(&name);
                    self.def_check.insert(id, true);
                    id
                }
            };
            self.graph.words.insert(id, words);
        }
    }

    pub fn validate_graph(&self) -> Vec<String> {
        if !self.errors.is_empty() {
            return Vec::new();
        }
        let mut errors = Vec::new();
        for (&key, &defined) in &self.def_check {
            let has_words = self
                .graph
                .words
                .get(&key)
                .map(|w| !w.is_empty())
                .unwrap_or(false);
            if !defined && !has_words {
                let name = self
                    .rev_name_map
                    .get(&key)
                    .cloned()
                    .unwrap_or_else(|| key.to_string());
                errors.push(format!("definition of '{}' not found", name));
            }
        }
        errors
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::scanner::extract_tokens;

    fn parse(input: &str) -> Result<Parser, String> {
        let (tokens, scan_errs) = extract_tokens(input);
        if !scan_errs.is_empty() {
            return Err(scan_errs[0].to_string());
        }
        let mut p = Parser::new();
        p.tokens = tokens;
        p.parse_grammar();
        if !p.errors.is_empty() {
            return Err(p.errors.join("\n"));
        }
        let errs = p.validate_graph();
        if !errs.is_empty() {
            return Err(errs.join("\n"));
        }
        Ok(p)
    }

    #[test]
    fn parses_simple_grammar() {
        let p = parse("program:'hello' 'world';").unwrap();
        assert!(p.name_map.contains_key("program"));
    }

    #[test]
    fn missing_definition_is_reported() {
        let err = parse("program:missing;").err().unwrap();
        assert!(err.contains("not found"), "{}", err);
    }

    #[test]
    fn duplicate_definition_is_reported() {
        let err = parse("program:'a';\nprogram:'b';").err().unwrap();
        assert!(err.contains("multiple definitions"), "{}", err);
    }

    #[test]
    fn malformed_grammars_do_not_panic() {
        for grammar in [
            "program:'a'",
            "program:+'a';",
            "program:*'a';",
            "program:?'a';",
            "program 'a';",
            "'a';",
            "program:'a');",
            "program:('a';",
            "program:'a'<nope>;",
            "program:'a'<-1>;",
            "program:'a'<NaN>;",
            "program:'a'<Inf>;",
            "program:'a' $ 'b';",
            "program:<0.5>;",
        ] {
            let result = std::panic::catch_unwind(|| parse(grammar));
            assert!(result.is_ok(), "panicked for {:?}", grammar);
            assert!(result.unwrap().is_err(), "expected error for {:?}", grammar);
        }
    }

    #[test]
    fn errors_have_positions() {
        let err = parse("program:+'a';").err().unwrap();
        assert!(err.contains("line 1:"), "{}", err);
    }

    #[test]
    fn word_only_rules_validate() {
        // Add the word list before validation.
        let (tokens, _) = extract_tokens("program:identifier;");
        let mut p = Parser::new();
        p.tokens = tokens;
        p.words
            .insert("identifier".to_string(), vec!["count".to_string()]);
        p.parse_grammar();
        p.attach_words();
        assert!(p.validate_graph().is_empty());
    }
}
