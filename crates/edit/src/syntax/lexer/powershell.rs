use super::Lexer;
use crate::syntax::token::{Token, TokenKind};

pub struct PowerShellLexer;

impl Lexer for PowerShellLexer {
    fn tokenize(&self, text: &[u8]) -> Vec<Token> {
        let mut tokens = Vec::new();
        let bytes = text;
        let mut pos = 0;

        while pos < bytes.len() {
            let start = pos;
            let ch = bytes[pos] as char;

            match ch {
                // Whitespace
                ' ' | '\t' | '\r' | '\n' => {
                    while pos < bytes.len() && matches!(bytes[pos] as char, ' ' | '\t' | '\r' | '\n') {
                        pos += 1;
                    }
                    tokens.push(Token {
                        kind: TokenKind::Whitespace,
                        span: start..pos,
                    });
                }

                // Comment
                '#' => {
                    while pos < bytes.len() && bytes[pos] != b'\n' {
                        pos += 1;
                    }
                    tokens.push(Token {
                        kind: TokenKind::Comment,
                        span: start..pos,
                    });
                }

                // Block comment
                '<' if pos + 1 < bytes.len() && bytes[pos + 1] == b'#' => {
                    pos += 2;
                    while pos + 1 < bytes.len() {
                        if bytes[pos] == b'#' && bytes[pos + 1] == b'>' {
                            pos += 2;
                            break;
                        }
                        pos += 1;
                    }
                    tokens.push(Token {
                        kind: TokenKind::Comment,
                        span: start..pos,
                    });
                }

                // Double-quoted string
                '"' => {
                    pos += 1;
                    while pos < bytes.len() {
                        match bytes[pos] as char {
                            '`' => pos += 2, // Escape with backtick
                            '"' => {
                                pos += 1;
                                break;
                            }
                            '$' if pos + 1 < bytes.len() && matches!(bytes[pos + 1] as char, '{' | '(' | 'a'..='z' | 'A'..='Z' | '_') => {
                                // Variable inside string
                                pos += 1;
                            }
                            _ => pos += 1,
                        }
                    }
                    tokens.push(Token {
                        kind: TokenKind::String,
                        span: start..pos,
                    });
                }

                // Single-quoted string
                '\'' => {
                    pos += 1;
                    while pos < bytes.len() {
                        if bytes[pos] == b'\'' {
                            if pos + 1 < bytes.len() && bytes[pos + 1] == b'\'' {
                                pos += 2; // Escaped quote
                            } else {
                                pos += 1;
                                break;
                            }
                        } else {
                            pos += 1;
                        }
                    }
                    tokens.push(Token {
                        kind: TokenKind::String,
                        span: start..pos,
                    });
                }

                // Here-string @" or @'
                '@' if pos + 1 < bytes.len() && matches!(bytes[pos + 1] as char, '"' | '\'') => {
                    let quote = bytes[pos + 1];
                    pos += 2;
                    // Must start on new line
                    if pos < bytes.len() && bytes[pos] == b'\n' {
                        pos += 1;
                    }
                    // Read until closing quote on new line
                    while pos + 1 < bytes.len() {
                        if bytes[pos] == b'\n' && bytes[pos + 1] == quote && pos + 2 < bytes.len() && bytes[pos + 2] == b'@' {
                            pos += 3;
                            break;
                        }
                        pos += 1;
                    }
                    tokens.push(Token {
                        kind: TokenKind::String,
                        span: start..pos,
                    });
                }

                // Variables
                '$' => {
                    pos += 1;
                    if pos < bytes.len() {
                        match bytes[pos] as char {
                            // Special variables
                            '?' | '^' | '$' => {
                                pos += 1;
                            }
                            // Braced variable
                            '{' => {
                                pos += 1;
                                while pos < bytes.len() && bytes[pos] != b'}' {
                                    pos += 1;
                                }
                                if pos < bytes.len() {
                                    pos += 1;
                                }
                            }
                            // Subexpression
                            '(' => {
                                pos += 1;
                                let mut depth = 1;
                                while pos < bytes.len() && depth > 0 {
                                    match bytes[pos] as char {
                                        '(' => depth += 1,
                                        ')' => depth -= 1,
                                        _ => {}
                                    }
                                    pos += 1;
                                }
                            }
                            // Regular variable
                            'a'..='z' | 'A'..='Z' | '_' => {
                                while pos < bytes.len() && matches!(bytes[pos] as char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | ':') {
                                    pos += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                    tokens.push(Token {
                        kind: TokenKind::VariableName,
                        span: start..pos,
                    });
                }

                // Numbers
                '0'..='9' => {
                    // Hex
                    if ch == '0' && pos + 1 < bytes.len() && matches!(bytes[pos + 1] as char, 'x' | 'X') {
                        pos += 2;
                        while pos < bytes.len() && (bytes[pos] as char).is_ascii_hexdigit() {
                            pos += 1;
                        }
                    } else {
                        while pos < bytes.len() && (bytes[pos] as char).is_ascii_digit() {
                            pos += 1;
                        }
                        // Decimal point
                        if pos < bytes.len() && bytes[pos] == b'.' && pos + 1 < bytes.len() && (bytes[pos + 1] as char).is_ascii_digit() {
                            pos += 1;
                            while pos < bytes.len() && (bytes[pos] as char).is_ascii_digit() {
                                pos += 1;
                            }
                        }
                        // Type suffix (KB, MB, GB, TB, PB)
                        if pos + 1 < bytes.len() && matches!(bytes[pos] as char, 'k' | 'K' | 'm' | 'M' | 'g' | 'G' | 't' | 'T' | 'p' | 'P') &&
                           matches!(bytes[pos + 1] as char, 'b' | 'B') {
                            pos += 2;
                        }
                    }
                    tokens.push(Token {
                        kind: TokenKind::Number,
                        span: start..pos,
                    });
                }

                // Keywords, cmdlets, and identifiers
                'a'..='z' | 'A'..='Z' | '_' => {
                    while pos < bytes.len() {
                        match bytes[pos] as char {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => pos += 1,
                            _ => break,
                        }
                    }
                    
                    let word = std::str::from_utf8(&bytes[start..pos]).unwrap_or("");
                    let kind = match word.to_lowercase().as_str() {
                        // Keywords
                        "begin" | "break" | "catch" | "class" | "continue" | "data" | "define" |
                        "do" | "dynamicparam" | "else" | "elseif" | "end" | "exit" | "filter" |
                        "finally" | "for" | "foreach" | "from" | "function" | "if" | "in" |
                        "param" | "process" | "return" | "switch" | "throw" | "trap" | "try" |
                        "until" | "using" | "var" | "while" | "workflow" | "parallel" | "sequence" |
                        "inlinescript" => TokenKind::Keyword,

                        // Operators (word-based)
                        "and" | "or" | "not" | "xor" | "band" | "bor" | "bnot" | "bxor" |
                        "eq" | "ne" | "gt" | "ge" | "lt" | "le" | "like" | "notlike" |
                        "match" | "notmatch" | "contains" | "notcontains" | "notin" |
                        "replace" | "is" | "isnot" | "as" | "split" | "join" | "f" => TokenKind::Operator,

                        // Boolean literals
                        "true" | "false" => TokenKind::Boolean,

                        // Null
                        "null" => TokenKind::Null,

                        _ => TokenKind::Identifier,
                    };
                    
                    tokens.push(Token { kind, span: start..pos });
                }

                // Operators and punctuation
                '-' if pos + 1 < bytes.len() && (bytes[pos + 1] as char).is_ascii_alphabetic() => {
                    // Parameter or operator starting with -
                    pos += 1;
                    while pos < bytes.len() && matches!(bytes[pos] as char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
                        pos += 1;
                    }
                    tokens.push(Token {
                        kind: TokenKind::Operator,
                        span: start..pos,
                    });
                }

                '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '&' | '|' | '^' | '~' |
                '?' | ':' | '.' | ',' | ';' | '(' | ')' | '[' | ']' | '{' | '}' | '@' => {
                    pos += 1;
                    // Handle multi-character operators
                    if pos < bytes.len() {
                        let next = bytes[pos] as char;
                        if matches!((ch, next),
                            ('+', '+') | ('-', '-') | ('=', '=') | ('!', '=') |
                            ('<', '=') | ('>', '=') | ('+', '=') | ('-', '=') |
                            ('*', '=') | ('/', '=') | ('%', '=') | ('&', '&') | ('|', '|') |
                            ('.', '.') | (':', ':')) {
                            pos += 1;
                        }
                    }
                    tokens.push(Token {
                        kind: TokenKind::Operator,
                        span: start..pos,
                    });
                }

                // Backtick (escape or line continuation)
                '`' => {
                    pos += 1;
                    if pos < bytes.len() {
                        pos += 1;
                    }
                    tokens.push(Token {
                        kind: TokenKind::Operator,
                        span: start..pos,
                    });
                }

                // Unknown character
                _ => {
                    pos += 1;
                    tokens.push(Token {
                        kind: TokenKind::Error,
                        span: start..pos,
                    });
                }
            }
        }

        tokens
    }
}
