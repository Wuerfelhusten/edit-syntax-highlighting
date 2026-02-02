// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! High-performance Rust lexer with full language support.

use crate::syntax::lexer::{Lexer, is_whitespace, is_ident_start, is_ident_continue, is_ascii_digit};
use crate::syntax::{Token, TokenKind};

pub struct RustLexer;

impl Lexer for RustLexer {
    fn tokenize(&self, text: &[u8]) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(text.len() / 8);
        let mut pos = 0;

        while pos < text.len() {
            let start = pos;
            let b = text[pos];

            match b {
                // Whitespace
                b' ' | b'\t' | b'\n' | b'\r' => {
                    while pos < text.len() && is_whitespace(text[pos]) {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Whitespace, start..pos));
                }

                // Line comment
                b'/' if pos + 1 < text.len() && text[pos + 1] == b'/' => {
                    pos += 2;
                    while pos < text.len() && text[pos] != b'\n' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Comment, start..pos));
                }

                // Block comment
                b'/' if pos + 1 < text.len() && text[pos + 1] == b'*' => {
                    pos += 2;
                    let mut depth = 1;
                    while pos + 1 < text.len() && depth > 0 {
                        if text[pos] == b'/' && text[pos + 1] == b'*' {
                            depth += 1;
                            pos += 2;
                        } else if text[pos] == b'*' && text[pos + 1] == b'/' {
                            depth -= 1;
                            pos += 2;
                        } else {
                            pos += 1;
                        }
                    }
                    tokens.push(Token::new(TokenKind::Comment, start..pos));
                }

                // String literal
                b'"' => {
                    pos += 1;
                    let mut escaped = false;
                    while pos < text.len() {
                        if escaped {
                            escaped = false;
                        } else if text[pos] == b'\\' {
                            escaped = true;
                        } else if text[pos] == b'"' {
                            pos += 1;
                            break;
                        }
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::String, start..pos));
                }

                // Raw string literal
                b'r' if pos + 1 < text.len() && text[pos + 1] == b'"' => {
                    pos += 2;
                    while pos < text.len() && text[pos] != b'"' {
                        pos += 1;
                    }
                    if pos < text.len() {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::String, start..pos));
                }

                // Lifetime (must come before character literals)
                b'\'' if pos + 1 < text.len() && is_ident_start(text[pos + 1]) => {
                    pos += 1;
                    while pos < text.len() && is_ident_continue(text[pos]) {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::RustLifetime, start..pos));
                }

                // Character literal
                b'\'' => {
                    pos += 1;
                    if pos < text.len() && text[pos] == b'\\' {
                        pos += 1; // Skip escape character
                    }
                    if pos < text.len() {
                        pos += 1; // Skip character
                    }
                    if pos < text.len() && text[pos] == b'\'' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Char, start..pos));
                }

                // Numbers
                b'0'..=b'9' => {
                    pos += 1;
                    
                    // Binary
                    if start + 1 < text.len() && text[start] == b'0' && text[start + 1] == b'b' {
                        pos += 1;
                        while pos < text.len() && matches!(text[pos], b'0' | b'1' | b'_') {
                            pos += 1;
                        }
                    }
                    // Octal
                    else if start + 1 < text.len() && text[start] == b'0' && text[start + 1] == b'o' {
                        pos += 1;
                        while pos < text.len() && matches!(text[pos], b'0'..=b'7' | b'_') {
                            pos += 1;
                        }
                    }
                    // Hexadecimal
                    else if start + 1 < text.len() && text[start] == b'0' && text[start + 1] == b'x' {
                        pos += 1;
                        while pos < text.len() && (is_ascii_digit(text[pos]) || matches!(text[pos], b'a'..=b'f' | b'A'..=b'F' | b'_')) {
                            pos += 1;
                        }
                    }
                    // Decimal
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
                        if pos < text.len() && (text[pos] == b'e' || text[pos] == b'E') {
                            pos += 1;
                            if pos < text.len() && (text[pos] == b'+' || text[pos] == b'-') {
                                pos += 1;
                            }
                            while pos < text.len() && (is_ascii_digit(text[pos]) || text[pos] == b'_') {
                                pos += 1;
                            }
                        }
                    }
                    
                    // Type suffix (i32, u64, f32, etc.)
                    if pos < text.len() && is_ident_start(text[pos]) {
                        while pos < text.len() && is_ident_continue(text[pos]) {
                            pos += 1;
                        }
                    }
                    
                    tokens.push(Token::new(TokenKind::Number, start..pos));
                }

                // Attribute (before identifiers to avoid conflicts)
                b'#' if pos + 1 < text.len() && text[pos + 1] == b'[' => {
                    pos += 1;
                    let mut depth = 0;
                    while pos < text.len() {
                        if text[pos] == b'[' {
                            depth += 1;
                        } else if text[pos] == b']' {
                            if depth == 0 {
                                pos += 1;
                                break;
                            }
                            depth -= 1;
                        }
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::RustAttribute, start..pos));
                }

                // Identifiers and keywords
                _ if is_ident_start(b) => {
                    while pos < text.len() && is_ident_continue(text[pos]) {
                        pos += 1;
                    }
                    
                    let word = &text[start..pos];
                    let kind = match word {
                        b"as" | b"in" | b"is" => TokenKind::KeywordOperator,
                        b"break" | b"continue" | b"else" | b"for" | b"if" | b"loop" | b"match" | b"return" | b"while" => TokenKind::KeywordControl,
                        b"fn" | b"async" | b"await" => TokenKind::KeywordFunction,
                        b"use" | b"mod" | b"extern" | b"crate" => TokenKind::KeywordImport,
                        b"let" | b"const" | b"static" | b"mut" => TokenKind::KeywordStorage,
                        b"struct" | b"enum" | b"union" | b"trait" | b"type" | b"impl" => TokenKind::KeywordType,
                        b"pub" | b"priv" | b"super" | b"self" | b"Self" | b"where" | b"unsafe" | b"ref" | b"move" => TokenKind::Keyword,
                        b"true" | b"false" => TokenKind::Boolean,
                        _ => TokenKind::Identifier,
                    };
                    
                    tokens.push(Token::new(kind, start..pos));
                }

                // Operators and punctuation
                b'+' | b'-' | b'*' | b'/' | b'%' | b'&' | b'|' | b'^' | b'!' | b'=' | b'<' | b'>' => {
                    pos += 1;
                    // Handle multi-character operators
                    if pos < text.len() && matches!(text[pos], b'=' | b'&' | b'|' | b'<' | b'>') {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
                }

                b'{' | b'}' | b'[' | b']' | b'(' | b')' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Delimiter, start..pos));
                }

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
    fn test_rust_keywords() {
        let lexer = RustLexer;
        let text = b"fn main() { let x = 42; }";
        let tokens = lexer.tokenize(text);
        
        let has_fn = tokens.iter().any(|t| t.kind == TokenKind::KeywordFunction);
        let has_let = tokens.iter().any(|t| t.kind == TokenKind::KeywordStorage);
        
        assert!(has_fn);
        assert!(has_let);
    }

    #[test]
    fn test_rust_lifetime() {
        let lexer = RustLexer;
        let text = b"fn foo<'a>(x: &'a str) {}";
        let tokens = lexer.tokenize(text);
        
        let lifetimes: Vec<_> = tokens.iter()
            .filter(|t| t.kind == TokenKind::RustLifetime)
            .collect();
        
        assert_eq!(lifetimes.len(), 2);
    }

    #[test]
    fn test_rust_string() {
        let lexer = RustLexer;
        let text = br#""hello" r"raw string""#;
        let tokens = lexer.tokenize(text);
        
        let strings: Vec<_> = tokens.iter()
            .filter(|t| t.kind == TokenKind::String)
            .collect();
        
        assert_eq!(strings.len(), 2);
    }
}
