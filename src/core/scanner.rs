#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    OneOrMore,   // +
    AnyNo,       // *
    Infinite,    // ^
    Maybe,       // ?
    Option,      // |
    Padding,     // ;
    BracOpen,    // (
    BracClose,   // )
    Colon,       // :
    Character,   // '...'
    Probability, // <...>
    Regex,       // [...]
    Identifier,  // variable names
}

#[derive(Debug, Clone)]
pub struct Token {
    pub typ: TokenType,
    pub text: String,
}

impl Token {
    fn new(_: usize, typ: TokenType, text: String) -> Self {
        Token { typ, text }
    }
}

#[derive(Debug, Clone)]
pub struct ScanError {
    _msg: String,
}

impl ScanError {
    fn new(msg: String) -> Self {
        ScanError { _msg: msg }
    }
}

pub struct Scanner {
    pos: usize,   // current byte offset
    width: usize, // width of last char in bytes
    curr_r: char, // current rune/char
    tokens: Vec<Token>,
    chars: Vec<char>, // cached char array for easier iteration
}

impl Scanner {
    pub fn new(input: String) -> Self {
        let chars: Vec<char> = input.chars().collect();
        Scanner {
            pos: 0,
            width: 0,
            curr_r: '\0',
            tokens: Vec::new(),
            chars,
        }
    }

    // advance and return the next char
    fn next(&mut self) -> Option<char> {
        if self.pos >= self.chars.len() {
            self.width = 0;
            self.curr_r = '\0';
            return None;
        }

        let c = self.chars[self.pos];
        self.width = 1;
        self.pos += 1;
        self.curr_r = c;
        Some(c)
    }

    // look at the next char without consuming it
    fn peek(&mut self) -> Option<char> {
        if self.pos >= self.chars.len() {
            return None;
        }
        Some(self.chars[self.pos])
    }

    fn scan_delimited(
        &mut self,
        open: char,
        close: char,
        _allow_escapes: bool,
    ) -> Result<String, ScanError> {
        let mut buf = String::new();

        loop {
            match self.next() {
                None => {
                    return Err(ScanError::new(format!("unterminated '{}'", open)));
                }
                Some(r) => {
                    if r == close {
                        return Ok(buf);
                    }
                    buf.push(r);
                }
            }
        }
    }

    fn scan_identifier(&mut self) -> String {
        let mut buf = String::new();
        buf.push(self.curr_r);

        // Keep reading while next chars are valid identifier parts
        loop {
            match self.peek() {
                Some(r) if is_ident_part(r) => {
                    self.next();
                    buf.push(self.curr_r);
                }
                _ => break,
            }
        }

        buf
    }

    pub fn scan(mut self) -> (Vec<Token>, Vec<ScanError>) {
        let mut errs = Vec::new();

        while let Some(c) = self.next() {
            match c {
                '+' => {
                    self.tokens.push(Token::new(
                        self.pos - 1,
                        TokenType::OneOrMore,
                        String::new(),
                    ));
                }
                '*' => {
                    self.tokens
                        .push(Token::new(self.pos - 1, TokenType::AnyNo, String::new()));
                }
                '^' => {
                    self.tokens
                        .push(Token::new(self.pos - 1, TokenType::Infinite, String::new()));
                }
                '?' => {
                    self.tokens
                        .push(Token::new(self.pos - 1, TokenType::Maybe, String::new()));
                }
                '|' => {
                    self.tokens
                        .push(Token::new(self.pos - 1, TokenType::Option, String::new()));
                }
                ';' => {
                    self.tokens
                        .push(Token::new(self.pos - 1, TokenType::Padding, String::new()));
                }
                '(' => {
                    self.tokens
                        .push(Token::new(self.pos - 1, TokenType::BracOpen, String::new()));
                }
                ')' => {
                    self.tokens.push(Token::new(
                        self.pos - 1,
                        TokenType::BracClose,
                        String::new(),
                    ));
                }
                ':' => {
                    self.tokens
                        .push(Token::new(self.pos - 1, TokenType::Colon, String::new()));
                }
                '\'' => match self.scan_delimited('\'', '\'', false) {
                    Ok(val) => {
                        self.tokens.push(Token::new(
                            self.pos - val.len() - 2,
                            TokenType::Character,
                            val,
                        ));
                    }
                    Err(err) => {
                        errs.push(err);
                    }
                },
                '<' => match self.scan_delimited('<', '>', false) {
                    Ok(val) => {
                        self.tokens.push(Token::new(
                            self.pos - val.len() - 2,
                            TokenType::Probability,
                            val,
                        ));
                    }
                    Err(err) => {
                        errs.push(err);
                    }
                },
                '[' => match self.scan_delimited('[', ']', false) {
                    Ok(val) => {
                        self.tokens.push(Token::new(
                            self.pos - val.len() - 2,
                            TokenType::Regex,
                            val,
                        ));
                    }
                    Err(err) => {
                        errs.push(err);
                    }
                },
                _ => {
                    if is_ident_start(c) {
                        let buff = self.scan_identifier();
                        if !buff.is_empty() {
                            self.tokens.push(Token::new(
                                self.pos - buff.len(),
                                TokenType::Identifier,
                                buff,
                            ));
                        }
                    }
                    // Ignore whitespace and other characters
                }
            }
        }

        (self.tokens, errs)
    }
}

// Helper functions
fn is_alpha(r: char) -> bool {
    r.is_ascii_alphabetic()
}

fn is_digit(r: char) -> bool {
    r.is_ascii_digit()
}

fn is_ident_start(r: char) -> bool {
    is_alpha(r) || r == '_'
}

fn is_ident_part(r: char) -> bool {
    is_alpha(r) || is_digit(r) || r == '_'
}
