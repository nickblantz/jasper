pub trait IntoToken {
    fn get_symbol(&self) -> String;
    fn get_value(&self) -> &String;
}

impl std::fmt::Debug for dyn IntoToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{{ {} '{}' }}", self.get_symbol(), *self.get_value())
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(u8)]
pub enum IdentifierSymbol {
    COMMENT,
    STRING,
    INT,
    FLOAT,
    VARIABLE
}
impl IdentifierSymbol {
    fn to_str(&self) -> &str {
        match *self {
            IdentifierSymbol::COMMENT => "comment",
            IdentifierSymbol::STRING => "string",
            IdentifierSymbol::INT => "integer",
            IdentifierSymbol::FLOAT => "float",
            IdentifierSymbol::VARIABLE => "variable"
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    symbol: IdentifierSymbol,
    value: String
}
impl Identifier {
    pub fn create(sym: IdentifierSymbol, val: String) -> Identifier {
        Identifier { symbol: sym, value: val }
    }
}
impl IntoToken for Identifier {
    fn get_symbol(&self) -> String {
        format!("identifier::{}", self.symbol.to_str())
    }
    fn get_value(&self) -> &String {
        &self.value
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(u8)]
pub enum KeywordSymbol {
    END,
    ELSE,
    CASE,
    ENSURE,
    MODULE,
    ELSIF,
    DEF,
    RESCUE,
    NOT,
    THEN,
    YIELD,
    FOR,
    SELF,
    FALSE,
    RETRY,
    RETURN,
    TRUE,
    IF,
    DEFINED_P,
    SUPER,
    UNDEF,
    BREAK,
    IN,
    DO,
    NIL,
    UNTIL,
    UNLESS,
    OR,
    NEXT,
    WHEN,
    REDO,
    AND,
    BEGIN,
    __LINE__,
    CLASS,
    __FILE__,
    LEND,
    LBEGIN,
    WHILE,
    ALIAS,
    __ENCODING__,
    ILLEGAL
}
impl KeywordSymbol {
    pub fn from_string(s: &String) -> KeywordSymbol {
        match &s[..] {
            "end"          => KeywordSymbol::END,
            "else"         => KeywordSymbol::ELSE,
            "case"         => KeywordSymbol::CASE,
            "ensure"       => KeywordSymbol::ENSURE,
            "module"       => KeywordSymbol::MODULE,
            "elsif"        => KeywordSymbol::ELSIF,
            "def"          => KeywordSymbol::DEF,
            "rescue"       => KeywordSymbol::RESCUE,
            "not"          => KeywordSymbol::NOT,
            "then"         => KeywordSymbol::THEN,
            "yield"        => KeywordSymbol::YIELD,
            "for"          => KeywordSymbol::FOR,
            "self"         => KeywordSymbol::SELF,
            "false"        => KeywordSymbol::FALSE,
            "retry"        => KeywordSymbol::RETRY,
            "return"       => KeywordSymbol::RETURN,
            "true"         => KeywordSymbol::TRUE,
            "if"           => KeywordSymbol::IF,
            "defined?"     => KeywordSymbol::DEFINED_P,
            "super"        => KeywordSymbol::SUPER,
            "undef"        => KeywordSymbol::UNDEF,
            "break"        => KeywordSymbol::BREAK,
            "in"           => KeywordSymbol::IN,
            "do"           => KeywordSymbol::DO,
            "nil"          => KeywordSymbol::NIL,
            "until"        => KeywordSymbol::UNTIL,
            "unless"       => KeywordSymbol::UNLESS,
            "or"           => KeywordSymbol::OR,
            "next"         => KeywordSymbol::NEXT,
            "when"         => KeywordSymbol::WHEN,
            "redo"         => KeywordSymbol::REDO,
            "and"          => KeywordSymbol::AND,
            "begin"        => KeywordSymbol::BEGIN,
            "__LINE__"     => KeywordSymbol::__LINE__,
            "class"        => KeywordSymbol::CLASS,
            "__FILE__"     => KeywordSymbol::__FILE__,
            "END"          => KeywordSymbol::LEND,
            "BEGIN"        => KeywordSymbol::LBEGIN,
            "while"        => KeywordSymbol::WHILE,
            "alias"        => KeywordSymbol::ALIAS,
            "__ENCODING__" => KeywordSymbol::__ENCODING__,
            _              => KeywordSymbol::ILLEGAL,
        }
    }
    fn to_str(&self) -> &str {
        match *self {
            KeywordSymbol::END          => "end",
            KeywordSymbol::ELSE         => "else",
            KeywordSymbol::CASE         => "case",
            KeywordSymbol::ENSURE       => "ensure",
            KeywordSymbol::MODULE       => "module",
            KeywordSymbol::ELSIF        => "elsif",
            KeywordSymbol::DEF          => "def",
            KeywordSymbol::RESCUE       => "rescue",
            KeywordSymbol::NOT          => "not",
            KeywordSymbol::THEN         => "then",
            KeywordSymbol::YIELD        => "yield",
            KeywordSymbol::FOR          => "for",
            KeywordSymbol::SELF         => "self",
            KeywordSymbol::FALSE        => "false",
            KeywordSymbol::RETRY        => "retry",
            KeywordSymbol::RETURN       => "return",
            KeywordSymbol::TRUE         => "true",
            KeywordSymbol::IF           => "if",
            KeywordSymbol::DEFINED_P    => "defined?",
            KeywordSymbol::SUPER        => "super",
            KeywordSymbol::UNDEF        => "undef",
            KeywordSymbol::BREAK        => "break",
            KeywordSymbol::IN           => "in",
            KeywordSymbol::DO           => "do",
            KeywordSymbol::NIL          => "nil",
            KeywordSymbol::UNTIL        => "until",
            KeywordSymbol::UNLESS       => "unless",
            KeywordSymbol::OR           => "or",
            KeywordSymbol::NEXT         => "next",
            KeywordSymbol::WHEN         => "when",
            KeywordSymbol::REDO         => "redo",
            KeywordSymbol::AND          => "and",
            KeywordSymbol::BEGIN        => "begin",
            KeywordSymbol::__LINE__     => "__LINE__",
            KeywordSymbol::CLASS        => "class",
            KeywordSymbol::__FILE__     => "__FILE__",
            KeywordSymbol::LEND         => "END",
            KeywordSymbol::LBEGIN       => "BEGIN",
            KeywordSymbol::WHILE        => "while",
            KeywordSymbol::ALIAS        => "alias",
            KeywordSymbol::__ENCODING__ => "__ENCODING__",
            KeywordSymbol::ILLEGAL      => "ILLEGAL"
        }
    }
}

#[derive(Debug)]
pub struct Keyword {
    symbol: KeywordSymbol,
    value: String
}
impl Keyword {
    pub fn create(sym: KeywordSymbol, val: String) -> Keyword {
        Keyword { symbol: sym, value: val }
    }
}
impl IntoToken for Keyword {
    fn get_symbol(&self) -> String {
        format!("keyword::{}", self.symbol.to_str())
    }
    fn get_value(&self) -> &String {
        &self.value
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(u8)]
pub enum OperatorSymbol {
    ASSIGN,
    PLUS,
    PLUS_EQ,
    MINUS,
    MINUS_EQ,
    BANG,
    ASTERISK,
    POW,
    SLASH,
    DOT,
    AND,
    OR,
    OR_EQ,
    MODULO,
    MATCH,
    LT,
    LTE,
    GT,
    GTE,
    COMP,
    EQ,
    NOT_EQ,
    RANGE,
    RESOLUTION,
    ILLEGAL
}
impl OperatorSymbol {
    pub fn from_string(s: &String) -> OperatorSymbol {
        match &s[..] {
            "="   => OperatorSymbol::ASSIGN,
            "+"   => OperatorSymbol::PLUS,
            "+="  => OperatorSymbol::PLUS_EQ,
            "-"   => OperatorSymbol::MINUS,
            "-="  => OperatorSymbol::MINUS_EQ,
            "!"   => OperatorSymbol::BANG,
            "*"   => OperatorSymbol::ASTERISK,
            "**"  => OperatorSymbol::POW,
            "/"   => OperatorSymbol::SLASH,
            "."   => OperatorSymbol::DOT,
            "&&"  => OperatorSymbol::AND,
            "||"  => OperatorSymbol::OR,
            "||=" => OperatorSymbol::OR_EQ,
            "%"   => OperatorSymbol::MODULO,
            "=~"  => OperatorSymbol::MATCH,
            "<"   => OperatorSymbol::LT,
            "<="  => OperatorSymbol::LTE,
            ">"   => OperatorSymbol::GT,
            ">="  => OperatorSymbol::GTE,
            "<=>" => OperatorSymbol::COMP,
            "=="  => OperatorSymbol::EQ,
            "!="  => OperatorSymbol::NOT_EQ,
            ".."  => OperatorSymbol::RANGE,
            "::"  => OperatorSymbol::RESOLUTION,
            _     => OperatorSymbol::ILLEGAL
        }
    }
    fn to_str(&self) -> &str {
        match *self {
            OperatorSymbol::ASSIGN     => "=",
            OperatorSymbol::PLUS       => "+",
            OperatorSymbol::PLUS_EQ    => "+=",
            OperatorSymbol::MINUS      => "-",
            OperatorSymbol::MINUS_EQ   => "-=",
            OperatorSymbol::BANG       => "!",
            OperatorSymbol::ASTERISK   => "*",
            OperatorSymbol::POW        => "**",
            OperatorSymbol::SLASH      => "/",
            OperatorSymbol::DOT        => ".",
            OperatorSymbol::AND        => "&&",
            OperatorSymbol::OR         => "||",
            OperatorSymbol::OR_EQ      => "||=",
            OperatorSymbol::MODULO     => "%",
            OperatorSymbol::MATCH      => "~=",
            OperatorSymbol::LT         => "<",
            OperatorSymbol::LTE        => "<=",
            OperatorSymbol::GT         => ">",
            OperatorSymbol::GTE        => ">=",
            OperatorSymbol::COMP       => "<=>",
            OperatorSymbol::EQ         => "==",
            OperatorSymbol::NOT_EQ     => "!=",
            OperatorSymbol::RANGE      => "..",
            OperatorSymbol::RESOLUTION => "::",
            OperatorSymbol::ILLEGAL    => "ILLEGAL"
        }
    }
}

#[derive(Debug)]
pub struct Operator {
    symbol: OperatorSymbol,
    value: String
}
impl Operator {
    pub fn create(sym: OperatorSymbol, val: String) -> Operator {
        Operator { symbol: sym, value: val }
    }
}
impl IntoToken for Operator {
    fn get_symbol(&self) -> String {
        format!("operator::{}", self.symbol.to_str())
    }
    fn get_value(&self) -> &String {
        &self.value
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(u8)]
pub enum SeparatorSymbol {
    COMMA,
    SEMICOLON,
    COLON,
    BAR,
    L_PAREN,
    R_PAREN,
    L_BRACE,
    R_BRACE,
    L_BRACKET,
    R_BRACKET,
    ILLEGAL
}
impl SeparatorSymbol {
    pub fn from_string(s: &String) -> SeparatorSymbol {
        match &s[..] {
            "," => SeparatorSymbol::COMMA,
            ";" => SeparatorSymbol::SEMICOLON,
            ":" => SeparatorSymbol::COLON,
            "|" => SeparatorSymbol::BAR,
            "(" => SeparatorSymbol::L_PAREN,
            ")" => SeparatorSymbol::R_PAREN,
            "{" => SeparatorSymbol::L_BRACE,
            "}" => SeparatorSymbol::R_BRACE,
            "[" => SeparatorSymbol::L_BRACKET,
            "]" => SeparatorSymbol::R_BRACKET,
            _   => SeparatorSymbol::ILLEGAL,
        }
    }
    fn to_str(&self) -> &str {
        match *self {
            SeparatorSymbol::COMMA      => ",",
            SeparatorSymbol::SEMICOLON  => ";",
            SeparatorSymbol::COLON      => ":",
            SeparatorSymbol::BAR        => "|",
            SeparatorSymbol::L_PAREN    => "(",
            SeparatorSymbol::R_PAREN    => ")",
            SeparatorSymbol::L_BRACE    => "{",
            SeparatorSymbol::R_BRACE    => "}",
            SeparatorSymbol::L_BRACKET  => "[",
            SeparatorSymbol::R_BRACKET  => "]",
            SeparatorSymbol::ILLEGAL    => "ILLEGAL"
        }
    }
}

#[derive(Debug)]
pub struct Separator {
    symbol: SeparatorSymbol,
    value: String
}
impl Separator {
    pub fn create(sym: SeparatorSymbol, val: String) -> Separator {
        Separator { symbol: sym, value: val }
    }
}
impl IntoToken for Separator {
    fn get_symbol(&self) -> String {
        format!("separator::{}", self.symbol.to_str())
    }
    fn get_value(&self) -> &String {
        &self.value
    }
}