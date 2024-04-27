#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Minus,
    Plus,
    True,
    False,
    Null,
    Colon,
    Comma,
    DoubleQuote,
    SingleQuote,
    BackSlash,
    Dot,
    String(String),
    Number(f64),
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub position: usize,
}

impl Token {
    fn new(kind: TokenKind, position: usize) -> Self {
        Self { kind, position }
    }
}

pub struct Lexer {
    source: Vec<char>,
    index: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            index: 0,
        }
    }

    fn next<'a>(&'a mut self) -> Option<&'a char> {
        let next = self.source.get(self.index);
        self.index += 1;
        return next;
    }

    fn peek<'a>(&'a self) -> Option<&'a char> {
        return self.source.get(self.index);
    }

    fn string(&mut self, tokens: &mut Vec<Token>) -> Option<()> {
        let mut closed = false;
        let start_index = self.index;
        while let Some(&ch) = self.peek() {
            if ch == '"' {
                closed = true;
                break;
            }
            self.next();
        }
        if !closed {
            eprintln!("Unmatched '\"' at index: {}", start_index);
            return None;
        }

        let string: String = self
            .source
            .iter()
            .enumerate()
            .skip(start_index)
            .take(self.index - start_index)
            .map(|(_, ch)| ch)
            .collect();

        tokens.push(Token::new(TokenKind::String(string), start_index));
        tokens.push(Token::new(TokenKind::DoubleQuote, self.index));
        Some(())
    }

    fn number(&mut self, tokens: &mut Vec<Token>) -> Option<()> {
        let start_index = self.index;
        while let Some(&ch) = self.peek() {
            if !matches!(ch, 'e' | 'E' | '.' | '0'..='9') {
                break;
            }
            self.next();
        }
        let num = self
            .source
            .iter()
            .enumerate()
            .skip(start_index)
            .take(self.index - start_index)
            .map(|(_, ch)| ch)
            .collect::<String>()
            .parse::<f64>();

        let num = match num {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Malformed Number Expression at index: {}", start_index);
                return None;
            }
        };
        tokens.push(Token::new(TokenKind::Number(num), start_index));
        Some(())
    }

    fn keyword(&mut self, tokens: &mut Vec<Token>) -> Option<()> {
        let start_index = self.index;
        while let Some(&ch) = self.peek() {
            if !ch.is_alphabetic() {
                break;
            }
            self.next();
        }

        let string: String = self
            .source
            .iter()
            .enumerate()
            .skip(start_index)
            .take(self.index - start_index)
            .map(|(_, ch)| ch)
            .collect();

        let token = match string.as_str() {
            "true" => Token::new(TokenKind::True, start_index),
            "false" => Token::new(TokenKind::False, start_index),
            "null" => Token::new(TokenKind::Null, start_index),
            _ => return None,
        };
        tokens.push(token);
        Some(())
    }

    pub fn lex(mut self) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek() {
            use TokenKind::*;
            match ch {
                '{' => tokens.push(Token::new(LeftBrace, self.index)),
                '}' => tokens.push(Token::new(RightBrace, self.index)),
                '[' => tokens.push(Token::new(LeftBracket, self.index)),
                ']' => tokens.push(Token::new(RightBracket, self.index)),
                ',' => tokens.push(Token::new(Comma, self.index)),
                '\'' => tokens.push(Token::new(SingleQuote, self.index)),
                '\\' => tokens.push(Token::new(BackSlash, self.index)),
                '.' => tokens.push(Token::new(Dot, self.index)),
                '-' => tokens.push(Token::new(Minus, self.index)),
                '+' => tokens.push(Token::new(Plus, self.index)),
                ':' => tokens.push(Token::new(Colon, self.index)),

                '"' => {
                    tokens.push(Token::new(DoubleQuote, self.index));
                    self.index += 1;
                    self.string(&mut tokens)?;
                }
                c if c.is_whitespace() => {}
                c if c.is_digit(10) => {
                    self.number(&mut tokens)?;
                    continue;
                }
                c if c.is_alphabetic() => {
                    self.keyword(&mut tokens)?;
                    continue;
                }
                c => panic!("Unexpected character '{c}' at index: {}", self.index),
            }
            self.next();
        }
        tokens.push(Token::new(TokenKind::EOF, self.index - 1));
        return Some(tokens);
    }
}
