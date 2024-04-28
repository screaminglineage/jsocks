use crate::json_value::*;
use crate::lexer::*;
use indexmap::IndexMap;
use TokenKind as tk;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

struct JsonParseError {
    msg: String,
    location: usize,
}

impl JsonParseError {
    fn new(msg: String, parser: &Parser) -> Self {
        Self {
            msg,
            location: parser.current,
        }
    }
}

type JsonResult = Result<JsonValue, JsonParseError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<JsonValue> {
        match self.json() {
            Ok(v) => Some(v),
            Err(e) => {
                eprintln!("{} at {}", e.msg, e.location);
                None
            }
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn advance(&mut self) -> Option<&Token> {
        if let Some(_) = self.peek() {
            self.current += 1;
        }
        return self.previous();
    }

    fn check(&self, token_kind: TokenKind) -> bool {
        if let Some(t) = self.peek() {
            token_kind == t.kind
        } else {
            false
        }
    }

    fn json(&mut self) -> JsonResult {
        let value = self.value()?;
        if let Some(_) = self.peek() {
            return Err(JsonParseError::new(
                String::from("Unexpected element after value"),
                self,
            ));
        }
        Ok(value)
    }

    #[rustfmt::skip]
    fn value(&mut self) -> JsonResult {
        match self.advance() {
            Some(Token { kind: tk::LeftBrace, .. }) => self.object(),
            Some(Token { kind: tk::LeftBracket, ..}) => self.array(),
            Some(Token { kind: tk::DoubleQuote, ..}) => self.string(),
            Some(Token { kind: tk::Number(num), ..}) => Ok(JsonValue::Number(*num)),
            Some(Token { kind: tk::True, ..}) => Ok(JsonValue::JsonBool(JsonBool::True)),
            Some(Token { kind: tk::False, ..}) => Ok(JsonValue::JsonBool(JsonBool::False)),
            Some(Token { kind: tk::Null, ..}) => Ok(JsonValue::Null),

            _ => Err(JsonParseError::new(String::from("Expected value"), self)),
        }
    }

    fn object(&mut self) -> JsonResult {
        if self.check(tk::RightBrace) {
            self.advance();
            return Ok(JsonValue::Object(IndexMap::new()));
        }
        let mut members = IndexMap::new();
        self.member(&mut members)?;

        while self.check(tk::Comma) {
            self.advance();
            self.member(&mut members)?;
        }

        if !self.check(tk::RightBrace) {
            return Err(JsonParseError::new(String::from("Expected '}}'"), self));
        }
        self.advance();
        return Ok(JsonValue::Object(members));
    }

    fn array(&mut self) -> JsonResult {
        if self.check(tk::RightBracket) {
            self.advance();
            return Ok(JsonValue::Array(Vec::new()));
        }
        let mut elements = Vec::new();
        elements.push(self.value()?);

        while self.check(tk::Comma) {
            self.advance();
            elements.push(self.value()?);
        }
        if !self.check(tk::RightBracket) {
            return Err(JsonParseError::new(String::from("Expected ']'"), self));
        }
        self.advance();
        return Ok(JsonValue::Array(elements));
    }

    fn string(&mut self) -> JsonResult {
        if self.check(tk::DoubleQuote) {
            self.advance();
            return Ok(JsonValue::String(String::new()));
        }

        match self.advance().cloned() {
            Some(Token {
                kind: tk::String(str),
                ..
            }) => {
                if self.check(tk::DoubleQuote) {
                    self.advance();
                    return Ok(JsonValue::String(str));
                } else {
                    return Err(JsonParseError::new(
                        String::from("Expected '\"' after String"),
                        self,
                    ));
                }
            }
            _ => return Err(JsonParseError::new(String::from("Expected String"), self)),
        }
    }

    fn member(&mut self, object: &mut IndexMap<String, JsonValue>) -> Result<(), JsonParseError> {
        self.advance();
        let JsonValue::String(str) = self.string()? else {
            return Err(JsonParseError::new(String::from("Expected String"), self));
        };

        if self.check(tk::Colon) {
            self.advance();
            let value = self.value()?;
            object.insert(str, value);
            return Ok(());
        }
        Err(JsonParseError::new(String::from("Expected ':'"), self))
    }
}
