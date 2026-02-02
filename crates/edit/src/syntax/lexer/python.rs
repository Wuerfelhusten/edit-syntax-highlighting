// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! High-performance Python lexer.

use crate::syntax::lexer::{Lexer, is_ident_start, is_ident_continue, is_ascii_digit};
use crate::syntax::{Token, TokenKind};

pub struct PythonLexer;

impl Lexer for PythonLexer {
    fn tokenize(&self, text: &[u8]) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(text.len() / 8);
        let mut pos = 0;

        while pos < text.len() {
            let start = pos;
            let b = text[pos];

            match b {
                // Whitespace
                b' ' | b'\t' | b'\r' => {
                    while pos < text.len() && matches!(text[pos], b' ' | b'\t' | b'\r') {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Whitespace, start..pos));
                }

                // Newline (significant in Python)
                b'\n' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Whitespace, start..pos));
                }

                // Comments
                b'#' => {
                    pos += 1;
                    while pos < text.len() && text[pos] != b'\n' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Comment, start..pos));
                }

                // String literals
                b'"' | b'\'' => {
                    let quote = b;
                    pos += 1;
                    
                    // Check for triple-quoted string
                    let triple = pos + 1 < text.len() 
                        && text[pos] == quote 
                        && text[pos + 1] == quote;
                    
                    if triple {
                        pos += 2;
                        while pos + 2 < text.len() {
                            if text[pos] == quote && text[pos + 1] == quote && text[pos + 2] == quote {
                                pos += 3;
                                break;
                            }
                            pos += 1;
                        }
                    } else {
                        let mut escaped = false;
                        while pos < text.len() {
                            if escaped {
                                escaped = false;
                            } else if text[pos] == b'\\' {
                                escaped = true;
                            } else if text[pos] == quote {
                                pos += 1;
                                break;
                            } else if text[pos] == b'\n' {
                                break; // Unterminated string
                            }
                            pos += 1;
                        }
                    }
                    
                    tokens.push(Token::new(TokenKind::String, start..pos));
                }

                // F-strings
                b'f' | b'F' if pos + 1 < text.len() && matches!(text[pos + 1], b'"' | b'\'') => {
                    pos += 1;
                    let quote = text[pos];
                    pos += 1;
                    
                    let mut escaped = false;
                    while pos < text.len() {
                        if escaped {
                            escaped = false;
                        } else if text[pos] == b'\\' {
                            escaped = true;
                        } else if text[pos] == quote {
                            pos += 1;
                            break;
                        }
                        pos += 1;
                    }
                    
                    tokens.push(Token::new(TokenKind::String, start..pos));
                }

                // Numbers
                b'0'..=b'9' => {
                    pos += 1;
                    
                    // Binary
                    if start + 1 < text.len() && text[start] == b'0' && matches!(text[start + 1], b'b' | b'B') {
                        pos += 1;
                        while pos < text.len() && matches!(text[pos], b'0' | b'1' | b'_') {
                            pos += 1;
                        }
                    }
                    // Octal
                    else if start + 1 < text.len() && text[start] == b'0' && matches!(text[start + 1], b'o' | b'O') {
                        pos += 1;
                        while pos < text.len() && matches!(text[pos], b'0'..=b'7' | b'_') {
                            pos += 1;
                        }
                    }
                    // Hexadecimal
                    else if start + 1 < text.len() && text[start] == b'0' && matches!(text[start + 1], b'x' | b'X') {
                        pos += 1;
                        while pos < text.len() && (is_ascii_digit(text[pos]) || matches!(text[pos], b'a'..=b'f' | b'A'..=b'F' | b'_')) {
                            pos += 1;
                        }
                    }
                    // Decimal or float
                    else {
                        while pos < text.len() && (is_ascii_digit(text[pos]) || text[pos] == b'_') {
                            pos += 1;
                        }
                        
                        // Float
                        if pos < text.len() && text[pos] == b'.' && pos + 1 < text.len() && is_ascii_digit(text[pos + 1]) {
                            pos += 1;
                            while pos < text.len() && (is_ascii_digit(text[pos]) || text[pos] == b'_') {
                                pos += 1;
                            }
                        }
                        
                        // Exponent
                        if pos < text.len() && matches!(text[pos], b'e' | b'E') {
                            pos += 1;
                            if pos < text.len() && matches!(text[pos], b'+' | b'-') {
                                pos += 1;
                            }
                            while pos < text.len() && (is_ascii_digit(text[pos]) || text[pos] == b'_') {
                                pos += 1;
                            }
                        }
                    }
                    
                    tokens.push(Token::new(TokenKind::Number, start..pos));
                }

                // Identifiers and keywords
                _ if is_ident_start(b) => {
                    while pos < text.len() && is_ident_continue(text[pos]) {
                        pos += 1;
                    }
                    
                    let word = &text[start..pos];
                    let kind = match word {
                        b"and" | b"or" | b"not" | b"in" | b"is" => TokenKind::KeywordOperator,
                        b"if" | b"elif" | b"else" | b"for" | b"while" | b"break" | b"continue" | b"return" | b"yield" | b"pass" | b"match" | b"case" => TokenKind::KeywordControl,
                        b"def" | b"lambda" | b"async" | b"await" => TokenKind::KeywordFunction,
                        b"import" | b"from" | b"as" => TokenKind::KeywordImport,
                        b"class" => TokenKind::KeywordType,
                        b"global" | b"nonlocal" | b"del" => TokenKind::KeywordStorage,
                        b"try" | b"except" | b"finally" | b"raise" | b"assert" | b"with" => TokenKind::Keyword,
                        b"True" | b"False" => TokenKind::Boolean,
                        b"None" => TokenKind::Null,
                        _ => TokenKind::Identifier,
                    };
                    
                    tokens.push(Token::new(kind, start..pos));
                }

                // Decorators
                b'@' if pos + 1 < text.len() && is_ident_start(text[pos + 1]) => {
                    pos += 1;
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'.') {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Attribute, start..pos));
                }

                // Operators
                b'+' | b'-' | b'*' | b'/' | b'%' | b'&' | b'|' | b'^' | b'~' | b'!' | b'=' | b'<' | b'>' => {
                    pos += 1;
                    // Handle multi-character operators
                    if pos < text.len() {
                        match text[pos] {
                            b'=' | b'*' | b'/' | b'<' | b'>' => {
                                pos += 1;
                            }
                            _ => {}
                        }
                    }
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
                }

                // Delimiters
                b'{' | b'}' | b'[' | b']' | b'(' | b')' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Delimiter, start..pos));
                }

                // Punctuation
                b',' | b';' | b':' | b'.' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Punctuation, start..pos));
                }

                // Unknown
                _ => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Error, start..pos));
                }
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_keywords() {
        let lexer = PythonLexer;
        let text = b"def main(): pass";
        let tokens = lexer.tokenize(text);
        
        let has_def = tokens.iter().any(|t| t.kind == TokenKind::KeywordFunction);
        let has_pass = tokens.iter().any(|t| t.kind == TokenKind::KeywordControl);
        
        assert!(has_def);
        assert!(has_pass);
    }

    #[test]
    fn test_python_strings() {
        let lexer = PythonLexer;
        let text = br#"'single' "double" """triple""""#;
        let tokens = lexer.tokenize(text);
        
        let strings: Vec<_> = tokens.iter()
            .filter(|t| t.kind == TokenKind::String)
            .collect();
        
        assert_eq!(strings.len(), 3);
    }

    #[test]
    fn test_python_decorator() {
        let lexer = PythonLexer;
        let text = b"@decorator\ndef foo(): pass";
        let tokens = lexer.tokenize(text);
        
        let has_decorator = tokens.iter().any(|t| t.kind == TokenKind::Attribute);
        assert!(has_decorator);
    }
}
