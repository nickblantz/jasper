use super::input_stream::InputStream;
use super::lexicon::{
    IntoToken,
    Identifier,
    IdentifierSymbol,
    Keyword,
    KeywordSymbol,
    Operator,
    OperatorSymbol,
    Separator,
    SeparatorSymbol
};

#[derive(Debug)]
pub struct Token<T: ?Sized + IntoToken> {
    line: u32,
    col: u32,
    data: Box<T>
}

static WHITESPACE_CHARS: [char; 4] = [' ', '\t', '\n', '\r'];
static COMMENT_START_CHAR: char = '#';
static STRING_START_CHAR: char = '\"';
static DIGIT_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
static LETTER_CHARS: [char; 52] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
static OPERATION_CHARS: [char; 12] = ['.', '+', '-', '*', '/', '%', '=', '&', '|', '<', '>', '!'];
static SEPARATOR_CHARS: [char; 9] = [',', ';', '(', ')', '{', '}', '[', ']', '|'];
static IDENTIFIER_SPECIAL_CHARS: [char; 3] = ['?', '!', '_'];

pub struct TokenStream<'a> {
    input_stream: &'a mut InputStream<'a>
}

impl<'a> TokenStream<'a> {
    pub fn create(is: &'a mut InputStream<'a>) -> TokenStream<'a> {
        TokenStream { input_stream: is }
    }

    fn is_comment_start(c: char) -> bool { c == COMMENT_START_CHAR }
    fn is_string_start(c: char) -> bool { c == STRING_START_CHAR }
    fn is_number_start(c: char) -> bool { DIGIT_CHARS.contains(&c) }
    fn is_identifier_start(c: char) -> bool { LETTER_CHARS.contains(&c) }
    fn is_operator_start(c: char) -> bool { OPERATION_CHARS.contains(&c) }
    fn is_separator_start(c: char) -> bool { SEPARATOR_CHARS.contains(&c) }

    fn is_whitespace(c: char) -> bool { WHITESPACE_CHARS.contains(&c) }
    fn is_comment(c: char) -> bool { c != '\n' && c != '\r' }
    fn is_number(c: char) -> bool { DIGIT_CHARS.contains(&c) || c == '.' }
    fn is_identifier(c: char) -> bool { LETTER_CHARS.contains(&c) || DIGIT_CHARS.contains(&c) || IDENTIFIER_SPECIAL_CHARS.contains(&c) }
    fn is_operator(c: char) -> bool { OPERATION_CHARS.contains(&c) }

    fn read_comment(&mut self) -> Token<dyn IntoToken> {
        let v = self.read_while(TokenStream::is_comment);
        Token { line: self.input_stream.get_line(), col: self.input_stream.get_col(), data: Box::new(Identifier::create(IdentifierSymbol::COMMENT, v)) }
    }
    fn read_string(&mut self) -> Token<dyn IntoToken> {
        let pos: (u32, u32) = (self.input_stream.get_line(), self.input_stream.get_col());
        let start_char: char = self.input_stream.next().unwrap();
        let v = self.read_while_string(start_char);
        self.input_stream.next();
        Token { line: pos.0, col: pos.1, data: Box::new(Identifier::create(IdentifierSymbol::STRING, v)) }
    }
    fn read_number(&mut self) -> Token<dyn IntoToken> {
        let v = self.read_while(TokenStream::is_number);
        if v.contains('.') {
            Token { line: self.input_stream.get_line(), col: self.input_stream.get_col(), data: Box::new(Identifier::create(IdentifierSymbol::FLOAT, v)) }
        } else {
            Token { line: self.input_stream.get_line(), col: self.input_stream.get_col(), data: Box::new(Identifier::create(IdentifierSymbol::INT, v)) }
        }
    }
    fn read_identifier(&mut self) -> Token<dyn IntoToken> {
        let pos: (u32, u32) = (self.input_stream.get_line(), self.input_stream.get_col());
        let v: String = self.read_while(TokenStream::is_identifier);
        match KeywordSymbol::from_string(&v) {
            KeywordSymbol::ILLEGAL => Token { line: pos.0, col: pos.1, data: Box::new(Identifier::create(IdentifierSymbol::VARIABLE, v)) },
            _ => Token { line: pos.0, col: pos.1, data: Box::new(Keyword::create(KeywordSymbol::from_string(&v), v)) }
        }
    }
    fn read_operator(&mut self) -> Token<dyn IntoToken> {
        let v = self.read_while(TokenStream::is_operator);
        match OperatorSymbol::from_string(&v) {
            OperatorSymbol::ILLEGAL => {
                if v.len() == 1 {
                    Token { line: self.input_stream.get_line(), col: self.input_stream.get_col(), data: Box::new(Separator::create(SeparatorSymbol::from_string(&v), v)) }
                } else {
                    Token { line: self.input_stream.get_line(), col: self.input_stream.get_col(), data: Box::new(Operator::create(OperatorSymbol::from_string(&v), v)) }
                }
            },
            _ => Token { line: self.input_stream.get_line(), col: self.input_stream.get_col(), data: Box::new(Operator::create(OperatorSymbol::from_string(&v), v)) }
        }
    }
    fn read_separator(&mut self) -> Token<dyn IntoToken> {
        let v = self.input_stream.next().unwrap().to_string();
        Token { line: self.input_stream.get_line(), col: self.input_stream.get_col(), data: Box::new(Separator::create(SeparatorSymbol::from_string(&v), v)) }
    }

    fn read_while(&mut self, func: fn(char) -> bool) -> String {
        let mut out: String = String::new();
        let mut unboxed_next: Option<&char> = self.input_stream.peek();
        while unboxed_next.is_some() && func(*unboxed_next.unwrap()) {
            out.push(self.input_stream.next().unwrap());
            unboxed_next = self.input_stream.peek();
        }
        out
    }
    fn read_while_string(&mut self, end: char) -> String {
        let mut escaped = false;
        let mut out: String = String::new();
        while let Some(next_c) = self.input_stream.peek() {
            if escaped {
                out.push(self.input_stream.next().unwrap());
                escaped = false;
            } else if *next_c == '\\' {
                out.push(self.input_stream.next().unwrap());
                escaped = true;
            } else if end == *next_c {
                break;
            } else {
                out.push(self.input_stream.next().unwrap());
            }
        }
        out
    }

    pub fn read_next(&mut self) -> Option<Token<dyn IntoToken>> {
        self.read_while(TokenStream::is_whitespace);
        if self.input_stream.is_eof() { return None; }
        let c: &char = self.input_stream.peek().unwrap();
        if TokenStream::is_comment_start(*c) { return Some(self.read_comment()); }
        if TokenStream::is_string_start(*c) { return Some(self.read_string()); }
        if TokenStream::is_number_start(*c) { return Some(self.read_number()); }
        if TokenStream::is_identifier_start(*c) { return Some(self.read_identifier()); }
        if TokenStream::is_operator_start(*c) { return Some(self.read_operator()); }
        if TokenStream::is_separator_start(*c) { return Some(self.read_separator()); }
        panic!("can't process character <{}> at [l: {}, c: {}]", *c as u32, self.input_stream.get_line(), self.input_stream.get_col())
    }
}