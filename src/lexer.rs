use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Literals
    Identifier(String),
    NumericLiteral(f64),
    StringLiteral(String),

    // Punctuation
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    Comma,
    Semicolon,
    Colon,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Exponent,
    Equal,
    EqualEqual,
    EqualEqualEqual,
    NotEqual,
    NotEqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    Increment,
    Decrement,
    Dot,

    // Keywords
    Function,
    Let,
    Const,
    Var,
    If,
    Else,
    For,
    While,
    Do,
    Return,
    True,
    False,
    Null,
    Undefined,
    This,
    New,
    Delete,
    Void,
    Typeof,
    Instanceof,
    In,
    Of,
    Switch,
    Case,
    Default,
    Break,
    Continue,
    Try,
    Catch,
    Finally,
    Throw,
    Debugger,
    Class,
    Extends,
    Super,
    Import,
    From,
    Export,
    Async,
    Await,
    Yield,
    Enum,
    Implements,
    Interface,
    Package,
    Private,
    Protected,
    Public,
    Static,
    Get,
    Set,

    // EOF
    EOF,
}

pub struct Lexer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            current_char: None,
            position: 0,
            line: 1,
            column: 0,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) -> Option<char> {
        let prev_char = self.current_char;
        self.current_char = self.input.next();
        self.position += 1;
        self.column += 1;

        if let Some('\n') = self.current_char {
            println!("yeaaaah");
            self.line += 1;
            self.column = 0;
        }

        prev_char
    }

    fn peek(&mut self) -> Option<char> {
        self.input.clone().nth(self.position)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        match self.current_char {
            Some('/') => {
                if let Some('/') = self.peek() {
                    // Single-line comment
                    self.advance(); // consume the second /
                    self.advance(); // consume the first character of comment
                    while let Some(c) = self.current_char {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                } else if let Some('*') = self.peek() {
                    // Multi-line comment
                    self.advance(); // consume the *
                    self.advance(); // consume the first character of comment
                    loop {
                        match self.current_char {
                            Some('*') => {
                                if let Some('/') = self.peek() {
                                    self.advance(); // consume the /
                                    break;
                                }
                                self.advance();
                            }
                            Some('\n') => {
                                self.line += 1;
                                self.column = 1;
                                self.advance();
                            }
                            Some(_) => {
                                self.advance();
                            }
                            None => break,
                        }
                    }
                } else {
                    return;
                }
            }
            _ => return,
        }
    }

    fn is_identifier_start(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_identifier_part(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_hex_digit(c: char) -> bool {
        c.is_ascii_hexdigit()
    }

    fn is_oct_digit(c: char) -> bool {
        c.is_ascii_digit() && c != '8' && c != '9'
    }

    fn is_binary_digit(c: char) -> bool {
        c == '0' || c == '1'
    }

    fn is_whitespace(c: Option<char>) -> bool {
        match c {
            Some(c) => c.is_whitespace(),
            None => false,
        }
    }

    fn get_position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
            offset: self.position,
        }
    }

    fn identifier(&mut self) -> Token {
        let start_pos = self.get_position();
        let mut identifier = String::new();

        while let Some(c) = self.current_char {
            if Self::is_identifier_part(c) {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let end_pos = self.get_position();
        let kind = match identifier.as_str() {
            "function" => TokenKind::Function,
            "let" => TokenKind::Let,
            "const" => TokenKind::Const,
            "var" => TokenKind::Var,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "do" => TokenKind::Do,
            "return" => TokenKind::Return,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            "undefined" => TokenKind::Undefined,
            "this" => TokenKind::This,
            "new" => TokenKind::New,
            "delete" => TokenKind::Delete,
            "void" => TokenKind::Void,
            "typeof" => TokenKind::Typeof,
            "instanceof" => TokenKind::Instanceof,
            "in" => TokenKind::In,
            "of" => TokenKind::Of,
            "switch" => TokenKind::Switch,
            "case" => TokenKind::Case,
            "default" => TokenKind::Default,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "try" => TokenKind::Try,
            "catch" => TokenKind::Catch,
            "finally" => TokenKind::Finally,
            "throw" => TokenKind::Throw,
            "debugger" => TokenKind::Debugger,
            "class" => TokenKind::Class,
            "extends" => TokenKind::Extends,
            "super" => TokenKind::Super,
            "import" => TokenKind::Import,
            "from" => TokenKind::From,
            "export" => TokenKind::Export,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "yield" => TokenKind::Yield,
            "enum" => TokenKind::Enum,
            "implements" => TokenKind::Implements,
            "interface" => TokenKind::Interface,
            "package" => TokenKind::Package,
            "private" => TokenKind::Private,
            "protected" => TokenKind::Protected,
            "public" => TokenKind::Public,
            "static" => TokenKind::Static,
            "get" => TokenKind::Get,
            "set" => TokenKind::Set,
            _ => TokenKind::Identifier(identifier),
        };

        Token {
            kind,
            start: start_pos,
            end: end_pos,
        }
    }

    fn number(&mut self) -> Token {
        let start_pos = self.get_position();
        let mut number = String::new();

        // Handle decimal point
        if self.current_char == Some('.') {
            number.push('.');
            self.advance();
        }

        // Handle digits
        while let Some(c) = self.current_char {
            if Self::is_digit(c) {
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Handle fractional part
        if self.current_char == Some('.') {
            number.push('.');
            self.advance();

            while let Some(c) = self.current_char {
                if Self::is_digit(c) {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Handle exponent
        if self.current_char == Some('e') || self.current_char == Some('E') {
            number.push(self.current_char.unwrap());
            self.advance();

            if self.current_char == Some('+') || self.current_char == Some('-') {
                number.push(self.current_char.unwrap());
                self.advance();
            }

            while let Some(c) = self.current_char {
                if Self::is_digit(c) {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let end_pos = self.get_position();
        number
            .parse::<f64>()
            .map(|n| Token {
                kind: TokenKind::NumericLiteral(n),
                start: start_pos,
                end: end_pos,
            })
            .unwrap_or_else(|_| {
                panic!(
                    "Invalid numeric literal at position {}:{}",
                    start_pos.line, start_pos.column
                )
            })
    }

    fn string(&mut self) -> Token {
        let start_pos = self.get_position();
        let mut string = String::new();
        let quote = self.current_char.unwrap();

        self.advance(); // consume the opening quote

        while let Some(c) = self.current_char {
            if c == quote {
                self.advance(); // consume the closing quote
                break;
            } else if c == '\\' {
                self.advance(); // consume the backslash
                if let Some(escape) = self.current_char {
                    string.push(escape);
                    self.advance();
                }
            } else if c == '\n' {
                panic!(
                    "Unterminated string literal at position {}:{}",
                    self.line, self.column
                );
            } else {
                string.push(c);
                self.advance();
            }
        }

        let end_pos = self.get_position();
        Token {
            kind: TokenKind::StringLiteral(string),
            start: start_pos,
            end: end_pos,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_comment();

        let start_pos = self.get_position();

        let token = match self.current_char {
            Some('(') => {
                self.advance();
                Token {
                    kind: TokenKind::ParenOpen,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            Some(')') => {
                self.advance();
                Token {
                    kind: TokenKind::ParenClose,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            Some('{') => {
                self.advance();
                Token {
                    kind: TokenKind::BraceOpen,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            Some('}') => {
                self.advance();
                Token {
                    kind: TokenKind::BraceClose,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            Some('[') => {
                self.advance();
                Token {
                    kind: TokenKind::BracketOpen,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            Some(']') => {
                self.advance();
                Token {
                    kind: TokenKind::BracketClose,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            Some(',') => {
                self.advance();
                Token {
                    kind: TokenKind::Comma,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            Some(';') => {
                self.advance();
                Token {
                    kind: TokenKind::Semicolon,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            Some(':') => {
                self.advance();
                Token {
                    kind: TokenKind::Colon,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            Some('+') => {
                self.advance();
                Token {
                    kind: TokenKind::Plus,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            Some('-') => {
                self.advance();
                Token {
                    kind: TokenKind::Minus,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            Some('*') => {
                self.advance();
                Token {
                    kind: TokenKind::Star,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            Some('/') => {
                self.advance();
                Token {
                    kind: TokenKind::Slash,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            Some('%') => {
                self.advance();
                Token {
                    kind: TokenKind::Percent,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            Some('=') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token {
                            kind: TokenKind::EqualEqualEqual,
                            start: start_pos,
                            end: self.get_position(),
                        }
                    } else {
                        Token {
                            kind: TokenKind::EqualEqual,
                            start: start_pos,
                            end: self.get_position(),
                        }
                    }
                } else {
                    Token {
                        kind: TokenKind::Equal,
                        start: start_pos,
                        end: self.get_position(),
                    }
                }
            }
            Some('!') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        Token {
                            kind: TokenKind::NotEqualEqual,
                            start: start_pos,
                            end: self.get_position(),
                        }
                    } else {
                        Token {
                            kind: TokenKind::NotEqual,
                            start: start_pos,
                            end: self.get_position(),
                        }
                    }
                } else {
                    Token {
                        kind: TokenKind::LogicalNot,
                        start: start_pos,
                        end: self.get_position(),
                    }
                }
            }
            Some('<') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token {
                        kind: TokenKind::LessEqual,
                        start: start_pos,
                        end: self.get_position(),
                    }
                } else {
                    Token {
                        kind: TokenKind::Less,
                        start: start_pos,
                        end: self.get_position(),
                    }
                }
            }
            Some('>') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token {
                        kind: TokenKind::GreaterEqual,
                        start: start_pos,
                        end: self.get_position(),
                    }
                } else {
                    Token {
                        kind: TokenKind::Greater,
                        start: start_pos,
                        end: self.get_position(),
                    }
                }
            }
            Some('&') => {
                self.advance();
                if self.current_char == Some('&') {
                    self.advance();
                    Token {
                        kind: TokenKind::LogicalAnd,
                        start: start_pos,
                        end: self.get_position(),
                    }
                } else {
                    panic!(
                        "Unexpected character '&' at position {}:{}",
                        start_pos.line, start_pos.column
                    );
                }
            }
            Some('|') => {
                self.advance();
                if self.current_char == Some('|') {
                    self.advance();
                    Token {
                        kind: TokenKind::LogicalOr,
                        start: start_pos,
                        end: self.get_position(),
                    }
                } else {
                    panic!(
                        "Unexpected character '|' at position {}:{}",
                        start_pos.line, start_pos.column
                    );
                }
            }
            Some('^') => {
                self.advance();
                Token {
                    kind: TokenKind::Exponent,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            // This has to be corrected, could be a + or ++
            // Some('++') => {
            //     self.advance();
            //     self.advance();
            //     Token {
            //         kind: TokenKind::Increment,
            //         start: start_pos,
            //         end: self.get_position(),
            //     }
            // }
            // Some('--') => {
            //     self.advance();
            //     self.advance();
            //     Token {
            //         kind: TokenKind::Decrement,
            //         start: start_pos,
            //         end: self.get_position(),
            //     }
            // }
            Some('.') => {
                self.advance();
                Token {
                    kind: TokenKind::Dot,
                    start: start_pos,
                    end: self.get_position(),
                }
            }
            Some('"') | Some('\'') => self.string(),
            Some(c) if Self::is_digit(c) => self.number(),
            Some(c) if Self::is_identifier_start(c) => self.identifier(),
            None => Token {
                kind: TokenKind::EOF,
                start: start_pos,
                end: start_pos,
            },
            Some(c) => {
                panic!(
                    "Unexpected character '{}' at position {}:{}",
                    c, start_pos.line, start_pos.column
                );
            }
        };

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_with_positions() {
        let input = r#"
            let x = 42;
            const y = "hello";
            function foo() {
                return x + y;
            }
        "#;

        let mut lexer = Lexer::new(input);
        let tokens = [
            Token {
                kind: TokenKind::Let,
                start: Position {
                    line: 2,
                    column: 13,
                    offset: 14,
                },
                end: Position {
                    line: 2,
                    column: 16,
                    offset: 17,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 2,
                    column: 17,
                    offset: 18,
                },
                end: Position {
                    line: 2,
                    column: 18,
                    offset: 19,
                },
            },
            Token {
                kind: TokenKind::Equal,
                start: Position {
                    line: 2,
                    column: 19,
                    offset: 20,
                },
                end: Position {
                    line: 2,
                    column: 20,
                    offset: 21,
                },
            },
            Token {
                kind: TokenKind::NumericLiteral(42.0),
                start: Position {
                    line: 2,
                    column: 21,
                    offset: 22,
                },
                end: Position {
                    line: 2,
                    column: 23,
                    offset: 24,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 2,
                    column: 23,
                    offset: 24,
                },
                end: Position {
                    line: 3,
                    column: 0,
                    offset: 25,
                },
            },
            Token {
                kind: TokenKind::Const,
                start: Position {
                    line: 3,
                    column: 13,
                    offset: 38,
                },
                end: Position {
                    line: 3,
                    column: 18,
                    offset: 43,
                },
            },
            Token {
                kind: TokenKind::Identifier("y".to_string()),
                start: Position {
                    line: 3,
                    column: 19,
                    offset: 44,
                },
                end: Position {
                    line: 3,
                    column: 20,
                    offset: 45,
                },
            },
            Token {
                kind: TokenKind::Equal,
                start: Position {
                    line: 3,
                    column: 21,
                    offset: 46,
                },
                end: Position {
                    line: 3,
                    column: 22,
                    offset: 47,
                },
            },
            Token {
                kind: TokenKind::StringLiteral("hello".to_string()),
                start: Position {
                    line: 3,
                    column: 23,
                    offset: 48,
                },
                end: Position {
                    line: 3,
                    column: 30,
                    offset: 55,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 3,
                    column: 30,
                    offset: 55,
                },
                end: Position {
                    line: 4,
                    column: 0,
                    offset: 56,
                },
            },
            Token {
                kind: TokenKind::Function,
                start: Position {
                    line: 4,
                    column: 13,
                    offset: 69,
                },
                end: Position {
                    line: 4,
                    column: 21,
                    offset: 77,
                },
            },
            Token {
                kind: TokenKind::Identifier("foo".to_string()),
                start: Position {
                    line: 4,
                    column: 22,
                    offset: 78,
                },
                end: Position {
                    line: 4,
                    column: 25,
                    offset: 81,
                },
            },
            Token {
                kind: TokenKind::ParenOpen,
                start: Position {
                    line: 4,
                    column: 25,
                    offset: 81,
                },
                end: Position {
                    line: 4,
                    column: 26,
                    offset: 82,
                },
            },
            Token {
                kind: TokenKind::ParenClose,
                start: Position {
                    line: 4,
                    column: 26,
                    offset: 82,
                },
                end: Position {
                    line: 4,
                    column: 27,
                    offset: 83,
                },
            },
            Token {
                kind: TokenKind::BraceOpen,
                start: Position {
                    line: 4,
                    column: 28,
                    offset: 84,
                },
                end: Position {
                    line: 5,
                    column: 0,
                    offset: 85,
                },
            },
            Token {
                kind: TokenKind::Return,
                start: Position {
                    line: 5,
                    column: 17,
                    offset: 102,
                },
                end: Position {
                    line: 5,
                    column: 23,
                    offset: 108,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 5,
                    column: 24,
                    offset: 109,
                },
                end: Position {
                    line: 5,
                    column: 25,
                    offset: 110,
                },
            },
            Token {
                kind: TokenKind::Plus,
                start: Position {
                    line: 5,
                    column: 26,
                    offset: 111,
                },
                end: Position {
                    line: 5,
                    column: 27,
                    offset: 112,
                },
            },
            Token {
                kind: TokenKind::Identifier("y".to_string()),
                start: Position {
                    line: 5,
                    column: 28,
                    offset: 113,
                },
                end: Position {
                    line: 5,
                    column: 29,
                    offset: 114,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 5,
                    column: 29,
                    offset: 114,
                },
                end: Position {
                    line: 6,
                    column: 0,
                    offset: 115,
                },
            },
            Token {
                kind: TokenKind::BraceClose,
                start: Position {
                    line: 6,
                    column: 13,
                    offset: 128,
                },
                end: Position {
                    line: 7,
                    column: 0,
                    offset: 129,
                },
            },
            Token {
                kind: TokenKind::EOF,
                start: Position {
                    line: 7,
                    column: 9,
                    offset: 138,
                },
                end: Position {
                    line: 7,
                    column: 9,
                    offset: 138,
                },
            },
        ];

        for (i, expected) in tokens.iter().enumerate() {
            let actual = lexer.next_token();
            println!("{:?}", actual);
            assert_eq!(actual.kind, expected.kind);
            // assert_eq!(actual.start, expected.start);
            // assert_eq!(actual.end, expected.end);
        }
    }

    #[test]
    fn test_comments_with_positions() {
        let input = r#"
            // This is a single-line comment
            /* This is a
               multi-line comment */
            let x = 42; // with a comment at the end
        "#;

        let mut lexer = Lexer::new(input);
        let tokens = [
            Token {
                kind: TokenKind::Let,
                start: Position {
                    line: 4,
                    column: 13,
                    offset: 56,
                },
                end: Position {
                    line: 4,
                    column: 16,
                    offset: 59,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 4,
                    column: 18,
                    offset: 61,
                },
                end: Position {
                    line: 4,
                    column: 19,
                    offset: 62,
                },
            },
            Token {
                kind: TokenKind::Equal,
                start: Position {
                    line: 4,
                    column: 21,
                    offset: 64,
                },
                end: Position {
                    line: 4,
                    column: 22,
                    offset: 65,
                },
            },
            Token {
                kind: TokenKind::NumericLiteral(42.0),
                start: Position {
                    line: 4,
                    column: 24,
                    offset: 67,
                },
                end: Position {
                    line: 4,
                    column: 26,
                    offset: 69,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 4,
                    column: 27,
                    offset: 70,
                },
                end: Position {
                    line: 4,
                    column: 28,
                    offset: 71,
                },
            },
            Token {
                kind: TokenKind::EOF,
                start: Position {
                    line: 5,
                    column: 1,
                    offset: 72,
                },
                end: Position {
                    line: 5,
                    column: 1,
                    offset: 72,
                },
            },
        ];

        for (i, expected) in tokens.iter().enumerate() {
            let actual = lexer.next_token();
            println!("{:?}", actual);
            // assert_eq!(actual.kind, expected.kind);
            // assert_eq!(actual.start, expected.start);
            // assert_eq!(actual.end, expected.end);
        }
    }

    #[test]
    fn test_operators_with_positions() {
        let input = r#"
            x == y;
            x === z;
            x != a;
            x !== b;
            x < c;
            x <= d;
            x > e;
            x >= f;
            x && g;
            x || h;
            !i;
            x + j;
            x - k;
            x * l;
            x / m;
            x % n;
            x ** o;
            x = p;
            x += q;
            x -= r;
            x *= s;
            x /= t;
            x %= u;
            x **= v;
        "#;

        let mut lexer = Lexer::new(input);
        let tokens = [
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 2,
                    column: 13,
                    offset: 12,
                },
                end: Position {
                    line: 2,
                    column: 14,
                    offset: 13,
                },
            },
            Token {
                kind: TokenKind::EqualEqual,
                start: Position {
                    line: 2,
                    column: 16,
                    offset: 15,
                },
                end: Position {
                    line: 2,
                    column: 18,
                    offset: 17,
                },
            },
            Token {
                kind: TokenKind::Identifier("y".to_string()),
                start: Position {
                    line: 2,
                    column: 20,
                    offset: 19,
                },
                end: Position {
                    line: 2,
                    column: 21,
                    offset: 20,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 2,
                    column: 22,
                    offset: 21,
                },
                end: Position {
                    line: 2,
                    column: 23,
                    offset: 22,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 3,
                    column: 13,
                    offset: 24,
                },
                end: Position {
                    line: 3,
                    column: 14,
                    offset: 25,
                },
            },
            Token {
                kind: TokenKind::EqualEqualEqual,
                start: Position {
                    line: 3,
                    column: 16,
                    offset: 26,
                },
                end: Position {
                    line: 3,
                    column: 19,
                    offset: 29,
                },
            },
            Token {
                kind: TokenKind::Identifier("z".to_string()),
                start: Position {
                    line: 3,
                    column: 21,
                    offset: 31,
                },
                end: Position {
                    line: 3,
                    column: 22,
                    offset: 32,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 3,
                    column: 23,
                    offset: 33,
                },
                end: Position {
                    line: 3,
                    column: 24,
                    offset: 34,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 4,
                    column: 13,
                    offset: 36,
                },
                end: Position {
                    line: 4,
                    column: 14,
                    offset: 37,
                },
            },
            Token {
                kind: TokenKind::NotEqual,
                start: Position {
                    line: 4,
                    column: 16,
                    offset: 38,
                },
                end: Position {
                    line: 4,
                    column: 18,
                    offset: 40,
                },
            },
            Token {
                kind: TokenKind::Identifier("a".to_string()),
                start: Position {
                    line: 4,
                    column: 20,
                    offset: 42,
                },
                end: Position {
                    line: 4,
                    column: 21,
                    offset: 43,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 4,
                    column: 22,
                    offset: 44,
                },
                end: Position {
                    line: 4,
                    column: 23,
                    offset: 45,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 5,
                    column: 13,
                    offset: 47,
                },
                end: Position {
                    line: 5,
                    column: 14,
                    offset: 48,
                },
            },
            Token {
                kind: TokenKind::NotEqualEqual,
                start: Position {
                    line: 5,
                    column: 16,
                    offset: 49,
                },
                end: Position {
                    line: 5,
                    column: 19,
                    offset: 52,
                },
            },
            Token {
                kind: TokenKind::Identifier("b".to_string()),
                start: Position {
                    line: 5,
                    column: 21,
                    offset: 54,
                },
                end: Position {
                    line: 5,
                    column: 22,
                    offset: 55,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 5,
                    column: 23,
                    offset: 56,
                },
                end: Position {
                    line: 5,
                    column: 24,
                    offset: 57,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 6,
                    column: 13,
                    offset: 59,
                },
                end: Position {
                    line: 6,
                    column: 14,
                    offset: 60,
                },
            },
            Token {
                kind: TokenKind::Less,
                start: Position {
                    line: 6,
                    column: 16,
                    offset: 61,
                },
                end: Position {
                    line: 6,
                    column: 17,
                    offset: 62,
                },
            },
            Token {
                kind: TokenKind::Identifier("c".to_string()),
                start: Position {
                    line: 6,
                    column: 19,
                    offset: 63,
                },
                end: Position {
                    line: 6,
                    column: 20,
                    offset: 64,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 6,
                    column: 21,
                    offset: 65,
                },
                end: Position {
                    line: 6,
                    column: 22,
                    offset: 66,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 7,
                    column: 13,
                    offset: 68,
                },
                end: Position {
                    line: 7,
                    column: 14,
                    offset: 69,
                },
            },
            Token {
                kind: TokenKind::LessEqual,
                start: Position {
                    line: 7,
                    column: 16,
                    offset: 70,
                },
                end: Position {
                    line: 7,
                    column: 18,
                    offset: 72,
                },
            },
            Token {
                kind: TokenKind::Identifier("d".to_string()),
                start: Position {
                    line: 7,
                    column: 20,
                    offset: 74,
                },
                end: Position {
                    line: 7,
                    column: 21,
                    offset: 75,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 7,
                    column: 22,
                    offset: 76,
                },
                end: Position {
                    line: 7,
                    column: 23,
                    offset: 77,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 8,
                    column: 13,
                    offset: 79,
                },
                end: Position {
                    line: 8,
                    column: 14,
                    offset: 80,
                },
            },
            Token {
                kind: TokenKind::Greater,
                start: Position {
                    line: 8,
                    column: 16,
                    offset: 81,
                },
                end: Position {
                    line: 8,
                    column: 17,
                    offset: 82,
                },
            },
            Token {
                kind: TokenKind::Identifier("e".to_string()),
                start: Position {
                    line: 8,
                    column: 19,
                    offset: 83,
                },
                end: Position {
                    line: 8,
                    column: 20,
                    offset: 84,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 8,
                    column: 21,
                    offset: 85,
                },
                end: Position {
                    line: 8,
                    column: 22,
                    offset: 86,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 9,
                    column: 13,
                    offset: 88,
                },
                end: Position {
                    line: 9,
                    column: 14,
                    offset: 89,
                },
            },
            Token {
                kind: TokenKind::GreaterEqual,
                start: Position {
                    line: 9,
                    column: 16,
                    offset: 90,
                },
                end: Position {
                    line: 9,
                    column: 18,
                    offset: 92,
                },
            },
            Token {
                kind: TokenKind::Identifier("f".to_string()),
                start: Position {
                    line: 9,
                    column: 20,
                    offset: 94,
                },
                end: Position {
                    line: 9,
                    column: 21,
                    offset: 95,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 9,
                    column: 22,
                    offset: 96,
                },
                end: Position {
                    line: 9,
                    column: 23,
                    offset: 97,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 10,
                    column: 13,
                    offset: 99,
                },
                end: Position {
                    line: 10,
                    column: 14,
                    offset: 100,
                },
            },
            Token {
                kind: TokenKind::LogicalAnd,
                start: Position {
                    line: 10,
                    column: 16,
                    offset: 101,
                },
                end: Position {
                    line: 10,
                    column: 18,
                    offset: 103,
                },
            },
            Token {
                kind: TokenKind::Identifier("g".to_string()),
                start: Position {
                    line: 10,
                    column: 20,
                    offset: 105,
                },
                end: Position {
                    line: 10,
                    column: 21,
                    offset: 106,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 10,
                    column: 22,
                    offset: 107,
                },
                end: Position {
                    line: 10,
                    column: 23,
                    offset: 108,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 11,
                    column: 13,
                    offset: 110,
                },
                end: Position {
                    line: 11,
                    column: 14,
                    offset: 111,
                },
            },
            Token {
                kind: TokenKind::LogicalOr,
                start: Position {
                    line: 11,
                    column: 16,
                    offset: 112,
                },
                end: Position {
                    line: 11,
                    column: 18,
                    offset: 114,
                },
            },
            Token {
                kind: TokenKind::Identifier("h".to_string()),
                start: Position {
                    line: 11,
                    column: 20,
                    offset: 116,
                },
                end: Position {
                    line: 11,
                    column: 21,
                    offset: 117,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 11,
                    column: 22,
                    offset: 118,
                },
                end: Position {
                    line: 11,
                    column: 23,
                    offset: 119,
                },
            },
            Token {
                kind: TokenKind::LogicalNot,
                start: Position {
                    line: 12,
                    column: 13,
                    offset: 121,
                },
                end: Position {
                    line: 12,
                    column: 14,
                    offset: 122,
                },
            },
            Token {
                kind: TokenKind::Identifier("i".to_string()),
                start: Position {
                    line: 12,
                    column: 16,
                    offset: 124,
                },
                end: Position {
                    line: 12,
                    column: 17,
                    offset: 125,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 12,
                    column: 18,
                    offset: 126,
                },
                end: Position {
                    line: 12,
                    column: 19,
                    offset: 127,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 13,
                    column: 13,
                    offset: 129,
                },
                end: Position {
                    line: 13,
                    column: 14,
                    offset: 130,
                },
            },
            Token {
                kind: TokenKind::Plus,
                start: Position {
                    line: 13,
                    column: 16,
                    offset: 131,
                },
                end: Position {
                    line: 13,
                    column: 17,
                    offset: 132,
                },
            },
            Token {
                kind: TokenKind::Identifier("j".to_string()),
                start: Position {
                    line: 13,
                    column: 19,
                    offset: 133,
                },
                end: Position {
                    line: 13,
                    column: 20,
                    offset: 134,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 13,
                    column: 21,
                    offset: 135,
                },
                end: Position {
                    line: 13,
                    column: 22,
                    offset: 136,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 14,
                    column: 13,
                    offset: 138,
                },
                end: Position {
                    line: 14,
                    column: 14,
                    offset: 139,
                },
            },
            Token {
                kind: TokenKind::Minus,
                start: Position {
                    line: 14,
                    column: 16,
                    offset: 140,
                },
                end: Position {
                    line: 14,
                    column: 17,
                    offset: 141,
                },
            },
            Token {
                kind: TokenKind::Identifier("k".to_string()),
                start: Position {
                    line: 14,
                    column: 19,
                    offset: 142,
                },
                end: Position {
                    line: 14,
                    column: 20,
                    offset: 143,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 14,
                    column: 21,
                    offset: 144,
                },
                end: Position {
                    line: 14,
                    column: 22,
                    offset: 145,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 15,
                    column: 13,
                    offset: 147,
                },
                end: Position {
                    line: 15,
                    column: 14,
                    offset: 148,
                },
            },
            Token {
                kind: TokenKind::Star,
                start: Position {
                    line: 15,
                    column: 16,
                    offset: 149,
                },
                end: Position {
                    line: 15,
                    column: 17,
                    offset: 150,
                },
            },
            Token {
                kind: TokenKind::Identifier("l".to_string()),
                start: Position {
                    line: 15,
                    column: 19,
                    offset: 151,
                },
                end: Position {
                    line: 15,
                    column: 20,
                    offset: 152,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 15,
                    column: 21,
                    offset: 153,
                },
                end: Position {
                    line: 15,
                    column: 22,
                    offset: 154,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 16,
                    column: 13,
                    offset: 156,
                },
                end: Position {
                    line: 16,
                    column: 14,
                    offset: 157,
                },
            },
            Token {
                kind: TokenKind::Slash,
                start: Position {
                    line: 16,
                    column: 16,
                    offset: 158,
                },
                end: Position {
                    line: 16,
                    column: 17,
                    offset: 159,
                },
            },
            Token {
                kind: TokenKind::Identifier("m".to_string()),
                start: Position {
                    line: 16,
                    column: 19,
                    offset: 160,
                },
                end: Position {
                    line: 16,
                    column: 20,
                    offset: 161,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 16,
                    column: 21,
                    offset: 162,
                },
                end: Position {
                    line: 16,
                    column: 22,
                    offset: 163,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 17,
                    column: 13,
                    offset: 165,
                },
                end: Position {
                    line: 17,
                    column: 14,
                    offset: 166,
                },
            },
            Token {
                kind: TokenKind::Percent,
                start: Position {
                    line: 17,
                    column: 16,
                    offset: 167,
                },
                end: Position {
                    line: 17,
                    column: 17,
                    offset: 168,
                },
            },
            Token {
                kind: TokenKind::Identifier("n".to_string()),
                start: Position {
                    line: 17,
                    column: 19,
                    offset: 169,
                },
                end: Position {
                    line: 17,
                    column: 20,
                    offset: 170,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 17,
                    column: 21,
                    offset: 171,
                },
                end: Position {
                    line: 17,
                    column: 22,
                    offset: 172,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 18,
                    column: 13,
                    offset: 174,
                },
                end: Position {
                    line: 18,
                    column: 14,
                    offset: 175,
                },
            },
            Token {
                kind: TokenKind::Exponent,
                start: Position {
                    line: 18,
                    column: 16,
                    offset: 176,
                },
                end: Position {
                    line: 18,
                    column: 17,
                    offset: 177,
                },
            },
            Token {
                kind: TokenKind::Identifier("o".to_string()),
                start: Position {
                    line: 18,
                    column: 19,
                    offset: 178,
                },
                end: Position {
                    line: 18,
                    column: 20,
                    offset: 179,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 18,
                    column: 21,
                    offset: 180,
                },
                end: Position {
                    line: 18,
                    column: 22,
                    offset: 181,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 19,
                    column: 13,
                    offset: 183,
                },
                end: Position {
                    line: 19,
                    column: 14,
                    offset: 184,
                },
            },
            Token {
                kind: TokenKind::Equal,
                start: Position {
                    line: 19,
                    column: 16,
                    offset: 185,
                },
                end: Position {
                    line: 19,
                    column: 17,
                    offset: 186,
                },
            },
            Token {
                kind: TokenKind::Identifier("p".to_string()),
                start: Position {
                    line: 19,
                    column: 19,
                    offset: 187,
                },
                end: Position {
                    line: 19,
                    column: 20,
                    offset: 188,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 19,
                    column: 21,
                    offset: 189,
                },
                end: Position {
                    line: 19,
                    column: 22,
                    offset: 190,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 20,
                    column: 13,
                    offset: 192,
                },
                end: Position {
                    line: 20,
                    column: 14,
                    offset: 193,
                },
            },
            Token {
                kind: TokenKind::Plus,
                start: Position {
                    line: 20,
                    column: 16,
                    offset: 194,
                },
                end: Position {
                    line: 20,
                    column: 17,
                    offset: 195,
                },
            },
            Token {
                kind: TokenKind::Equal,
                start: Position {
                    line: 20,
                    column: 18,
                    offset: 196,
                },
                end: Position {
                    line: 20,
                    column: 19,
                    offset: 197,
                },
            },
            Token {
                kind: TokenKind::Identifier("q".to_string()),
                start: Position {
                    line: 20,
                    column: 21,
                    offset: 199,
                },
                end: Position {
                    line: 20,
                    column: 22,
                    offset: 200,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 20,
                    column: 23,
                    offset: 201,
                },
                end: Position {
                    line: 20,
                    column: 24,
                    offset: 202,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 21,
                    column: 13,
                    offset: 204,
                },
                end: Position {
                    line: 21,
                    column: 14,
                    offset: 205,
                },
            },
            Token {
                kind: TokenKind::Minus,
                start: Position {
                    line: 21,
                    column: 16,
                    offset: 206,
                },
                end: Position {
                    line: 21,
                    column: 17,
                    offset: 207,
                },
            },
            Token {
                kind: TokenKind::Equal,
                start: Position {
                    line: 21,
                    column: 18,
                    offset: 208,
                },
                end: Position {
                    line: 21,
                    column: 19,
                    offset: 209,
                },
            },
            Token {
                kind: TokenKind::Identifier("r".to_string()),
                start: Position {
                    line: 21,
                    column: 21,
                    offset: 211,
                },
                end: Position {
                    line: 21,
                    column: 22,
                    offset: 212,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 21,
                    column: 23,
                    offset: 213,
                },
                end: Position {
                    line: 21,
                    column: 24,
                    offset: 214,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 22,
                    column: 13,
                    offset: 216,
                },
                end: Position {
                    line: 22,
                    column: 14,
                    offset: 217,
                },
            },
            Token {
                kind: TokenKind::Star,
                start: Position {
                    line: 22,
                    column: 16,
                    offset: 218,
                },
                end: Position {
                    line: 22,
                    column: 17,
                    offset: 219,
                },
            },
            Token {
                kind: TokenKind::Equal,
                start: Position {
                    line: 22,
                    column: 18,
                    offset: 220,
                },
                end: Position {
                    line: 22,
                    column: 19,
                    offset: 221,
                },
            },
            Token {
                kind: TokenKind::Identifier("s".to_string()),
                start: Position {
                    line: 22,
                    column: 21,
                    offset: 223,
                },
                end: Position {
                    line: 22,
                    column: 22,
                    offset: 224,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 22,
                    column: 23,
                    offset: 225,
                },
                end: Position {
                    line: 22,
                    column: 24,
                    offset: 226,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 23,
                    column: 13,
                    offset: 228,
                },
                end: Position {
                    line: 23,
                    column: 14,
                    offset: 229,
                },
            },
            Token {
                kind: TokenKind::Slash,
                start: Position {
                    line: 23,
                    column: 16,
                    offset: 230,
                },
                end: Position {
                    line: 23,
                    column: 17,
                    offset: 231,
                },
            },
            Token {
                kind: TokenKind::Equal,
                start: Position {
                    line: 23,
                    column: 18,
                    offset: 232,
                },
                end: Position {
                    line: 23,
                    column: 19,
                    offset: 233,
                },
            },
            Token {
                kind: TokenKind::Identifier("t".to_string()),
                start: Position {
                    line: 23,
                    column: 21,
                    offset: 235,
                },
                end: Position {
                    line: 23,
                    column: 22,
                    offset: 236,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 23,
                    column: 23,
                    offset: 237,
                },
                end: Position {
                    line: 23,
                    column: 24,
                    offset: 238,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 24,
                    column: 13,
                    offset: 240,
                },
                end: Position {
                    line: 24,
                    column: 14,
                    offset: 241,
                },
            },
            Token {
                kind: TokenKind::Percent,
                start: Position {
                    line: 24,
                    column: 16,
                    offset: 242,
                },
                end: Position {
                    line: 24,
                    column: 17,
                    offset: 243,
                },
            },
            Token {
                kind: TokenKind::Equal,
                start: Position {
                    line: 24,
                    column: 18,
                    offset: 244,
                },
                end: Position {
                    line: 24,
                    column: 19,
                    offset: 245,
                },
            },
            Token {
                kind: TokenKind::Identifier("u".to_string()),
                start: Position {
                    line: 24,
                    column: 21,
                    offset: 247,
                },
                end: Position {
                    line: 24,
                    column: 22,
                    offset: 248,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 24,
                    column: 23,
                    offset: 249,
                },
                end: Position {
                    line: 24,
                    column: 24,
                    offset: 250,
                },
            },
            Token {
                kind: TokenKind::Identifier("x".to_string()),
                start: Position {
                    line: 25,
                    column: 13,
                    offset: 252,
                },
                end: Position {
                    line: 25,
                    column: 14,
                    offset: 253,
                },
            },
            Token {
                kind: TokenKind::Exponent,
                start: Position {
                    line: 25,
                    column: 16,
                    offset: 254,
                },
                end: Position {
                    line: 25,
                    column: 17,
                    offset: 255,
                },
            },
            Token {
                kind: TokenKind::Equal,
                start: Position {
                    line: 25,
                    column: 18,
                    offset: 256,
                },
                end: Position {
                    line: 25,
                    column: 19,
                    offset: 257,
                },
            },
            Token {
                kind: TokenKind::Identifier("v".to_string()),
                start: Position {
                    line: 25,
                    column: 21,
                    offset: 259,
                },
                end: Position {
                    line: 25,
                    column: 22,
                    offset: 260,
                },
            },
            Token {
                kind: TokenKind::Semicolon,
                start: Position {
                    line: 25,
                    column: 23,
                    offset: 261,
                },
                end: Position {
                    line: 25,
                    column: 24,
                    offset: 262,
                },
            },
            Token {
                kind: TokenKind::EOF,
                start: Position {
                    line: 26,
                    column: 1,
                    offset: 263,
                },
                end: Position {
                    line: 26,
                    column: 1,
                    offset: 263,
                },
            },
        ];

        for (i, expected) in tokens.iter().enumerate() {
            let actual = lexer.next_token();
            assert_eq!(actual.kind, expected.kind);
            assert_eq!(actual.start, expected.start);
            assert_eq!(actual.end, expected.end);
        }
    }
}
