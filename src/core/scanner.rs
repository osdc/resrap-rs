use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub line: usize,
    pub col: usize,
}

impl Token {
    fn new(typ: TokenType, text: String, line: usize, col: usize) -> Self {
        Token {
            typ,
            text,
            line,
            col,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScanError {
    #[allow(dead_code)]
    pub pos: usize,
    pub line: usize,
    pub col: usize,
    pub msg: String,
}

impl ScanError {
    fn new(pos: usize, line: usize, col: usize, msg: String) -> Self {
        ScanError {
            pos,
            line,
            col,
            msg,
        }
    }
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}:{}: {}", self.line, self.col, self.msg)
    }
}

pub struct Scanner {
    chars: Vec<char>,
    pos: usize,   // current index into chars
    width: usize, // width of the last consumed char
    curr_r: Option<char>,
    line: usize, // line of the *next* char
    col: usize,  // column of the *next* char
    curr_ln: usize,
    curr_cl: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(input: &str) -> Self {
        Scanner {
            chars: input.chars().collect(),
            pos: 0,
            width: 0,
            curr_r: None,
            line: 1,
            col: 1,
            curr_ln: 1,
            curr_cl: 1,
            tokens: Vec::new(),
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.pos >= self.chars.len() {
            self.width = 0;
            self.curr_r = None;
            return None;
        }
        let c = self.chars[self.pos];
        self.width = 1;
        self.pos += 1;
        self.curr_ln = self.line;
        self.curr_cl = self.col;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        self.curr_r = Some(c);
        Some(c)
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn add(&mut self, typ: TokenType, text: String) {
        self.tokens
            .push(Token::new(typ, text, self.curr_ln, self.curr_cl));
    }

    /// Reads until `close`, honouring backslash escapes so literals such as
    /// `'it\'s'` and classes such as `[a\-z]` keep their payload intact.
    /// Escapes are decoded later by `unescape_string` / `ExpandClass`.
    fn scan_delimited(&mut self, open: char, close: char) -> Result<String, ScanError> {
        let start_pos = self.pos.saturating_sub(self.width);
        let start_line = self.curr_ln;
        let start_col = self.curr_cl;

        let mut buf = String::new();
        loop {
            match self.next() {
                None => {
                    return Err(ScanError::new(
                        start_pos,
                        start_line,
                        start_col,
                        format!("unterminated {:?}", open),
                    ));
                }
                Some(r) if r == close => return Ok(buf),
                Some('\\') => match self.next() {
                    None => {
                        return Err(ScanError::new(
                            start_pos,
                            start_line,
                            start_col,
                            format!("unterminated {:?}", open),
                        ));
                    }
                    Some(nr) => {
                        buf.push('\\');
                        buf.push(nr);
                    }
                },
                Some(r) => buf.push(r),
            }
        }
    }

    fn skip_line(&mut self) {
        while let Some(r) = self.peek() {
            if r == '\n' {
                return;
            }
            self.next();
        }
    }

    fn scan_identifier(&mut self) -> String {
        let mut buf = String::new();
        if let Some(c) = self.curr_r {
            buf.push(c);
        }
        while let Some(r) = self.peek() {
            if !is_ident_part(r) {
                break;
            }
            self.next();
            if let Some(c) = self.curr_r {
                buf.push(c);
            }
        }
        buf
    }

    pub fn scan(mut self) -> (Vec<Token>, Vec<ScanError>) {
        let mut errs = Vec::new();

        while let Some(c) = self.next() {
            match c {
                ' ' | '\t' | '\r' | '\n' => {}
                '#' => self.skip_line(),
                '/' => {
                    if self.peek() == Some('/') {
                        self.skip_line();
                    } else {
                        errs.push(ScanError::new(
                            self.pos.saturating_sub(self.width),
                            self.curr_ln,
                            self.curr_cl,
                            format!("unexpected character {:?}", c),
                        ));
                    }
                }
                '+' => self.add(TokenType::OneOrMore, String::new()),
                '*' => self.add(TokenType::AnyNo, String::new()),
                '^' => self.add(TokenType::Infinite, String::new()),
                '?' => self.add(TokenType::Maybe, String::new()),
                '|' => self.add(TokenType::Option, String::new()),
                ';' => self.add(TokenType::Padding, String::new()),
                '(' => self.add(TokenType::BracOpen, String::new()),
                ')' => self.add(TokenType::BracClose, String::new()),
                ':' => self.add(TokenType::Colon, String::new()),
                '\'' => match self.scan_delimited('\'', '\'') {
                    Ok(val) => self.add(TokenType::Character, val),
                    Err(err) => errs.push(err),
                },
                '<' => match self.scan_delimited('<', '>') {
                    Ok(val) => self.add(TokenType::Probability, val),
                    Err(err) => errs.push(err),
                },
                '[' => match self.scan_delimited('[', ']') {
                    Ok(val) => self.add(TokenType::Regex, val),
                    Err(err) => errs.push(err),
                },
                _ => {
                    if is_ident_start(c) {
                        let buff = self.scan_identifier();
                        if !buff.is_empty() {
                            self.add(TokenType::Identifier, buff);
                        }
                    } else {
                        errs.push(ScanError::new(
                            self.pos.saturating_sub(self.width),
                            self.curr_ln,
                            self.curr_cl,
                            format!("unexpected character {:?}", c),
                        ));
                    }
                }
            }
        }

        (self.tokens, errs)
    }

    #[allow(dead_code)]
    pub fn print_tokens(&self) {
        for token in &self.tokens {
            println!("{}", token.text);
        }
    }
}

pub fn extract_tokens(input: &str) -> (Vec<Token>, Vec<ScanError>) {
    Scanner::new(input).scan()
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_basic_tokens() {
        let (tokens, errs) = extract_tokens("program:'a' 'b';\n");
        assert!(errs.is_empty());
        let kinds: Vec<TokenType> = tokens.iter().map(|t| t.typ).collect();
        assert_eq!(
            kinds,
            vec![
                TokenType::Identifier,
                TokenType::Colon,
                TokenType::Character,
                TokenType::Character,
                TokenType::Padding,
            ]
        );
    }

    #[test]
    fn comments_are_skipped() {
        let (tokens, errs) = extract_tokens("# comment\nprogram:'a'; // trailing\n");
        assert!(errs.is_empty());
        assert_eq!(tokens.len(), 4);
    }

    #[test]
    fn escapes_are_preserved() {
        let (tokens, errs) = extract_tokens(r"program:'it\'s';");
        assert!(errs.is_empty());
        assert_eq!(tokens[2].text, r"it\'s");
    }

    #[test]
    fn unexpected_char_is_an_error() {
        let (_, errs) = extract_tokens("program:'a' $ 'b';");
        assert_eq!(errs.len(), 1);
        assert!(errs[0].to_string().contains("unexpected character"));
    }

    #[test]
    fn unterminated_literal_is_an_error() {
        let (_, errs) = extract_tokens("program:'unterminated");
        assert_eq!(errs.len(), 1);
        assert!(errs[0].to_string().contains("unterminated"));
    }

    #[test]
    fn error_has_position() {
        let (_, errs) = extract_tokens("program:\n  $;");
        assert_eq!(errs.len(), 1);
        assert!(errs[0].to_string().starts_with("line 2:3:"));
    }
}
