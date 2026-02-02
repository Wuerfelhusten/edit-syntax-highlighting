// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! TOML configuration file lexer.

use crate::syntax::lexer::{Lexer, is_ident_start, is_ident_continue, is_ascii_digit};
use crate::syntax::{Token, TokenKind};

pub struct TomlLexer;

impl Lexer for TomlLexer {
    fn tokenize(&self, text: &[u8]) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(text.len() / 8);
        let mut pos = 0;

        while pos < text.len() {
            let start = pos;
            let b = text[pos];

            match b {
                // Whitespace
                b' ' | b'\t' | b'\n' | b'\r' => {
                    while pos < text.len() && matches!(text[pos], b' ' | b'\t' | b'\n' | b'\r') {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Whitespace, start..pos));
                }

                // Comments
                b'#' => {
                    while pos < text.len() && text[pos] != b'\n' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Comment, start..pos));
                }

                // Section headers [section] or [[array]]
                b'[' => {
                    pos += 1;
                    let is_array = pos < text.len() && text[pos] == b'[';
                    if is_array {
                        pos += 1;
                    }
                    
                    while pos < text.len() && text[pos] != b']' {
                        pos += 1;
                    }
                    
                    if pos < text.len() {
                        pos += 1;
                        if is_array && pos < text.len() && text[pos] == b']' {
                            pos += 1;
                        }
                    }
                    tokens.push(Token::new(TokenKind::KeywordType, start..pos));
                }

                // Strings
                b'"' | b'\'' => {
                    let quote = b;
                    pos += 1;
                    
                    // Check for multi-line strings (""" or ''')
                    let multiline = pos + 1 < text.len() && text[pos] == quote && text[pos + 1] == quote;
                    if multiline {
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
                                break;
                            }
                            pos += 1;
                        }
                    }
                    tokens.push(Token::new(TokenKind::String, start..pos));
                }

                // Numbers (integers and floats)
                b'0'..=b'9' | b'+' | b'-' if matches!(b, b'+' | b'-') && pos + 1 < text.len() && is_ascii_digit(text[pos + 1]) || is_ascii_digit(b) => {
                    if matches!(b, b'+' | b'-') {
                        pos += 1;
                    }
                    
                    while pos < text.len() && (is_ascii_digit(text[pos]) || text[pos] == b'_') {
                        pos += 1;
                    }
                    
                    // Float
                    if pos < text.len() && text[pos] == b'.' {
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
                    
                    tokens.push(Token::new(TokenKind::Number, start..pos));
                }

                // Keywords (true, false)
                b't' if pos + 4 <= text.len() && &text[pos..pos + 4] == b"true" => {
                    pos += 4;
                    tokens.push(Token::new(TokenKind::Boolean, start..pos));
                }
                b'f' if pos + 5 <= text.len() && &text[pos..pos + 5] == b"false" => {
                    pos += 5;
                    tokens.push(Token::new(TokenKind::Boolean, start..pos));
                }

                // Keys (identifiers)
                _ if is_ident_start(b) => {
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'-') {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Identifier, start..pos));
                }

                // Operators and delimiters
                b'=' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
                }
                b'.' | b',' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Punctuation, start..pos));
                }
                b'{' | b'}' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Delimiter, start..pos));
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
    fn test_toml_section() {
        let lexer = TomlLexer;
        let text = b"[package]\nname = \"test\"";
        let tokens = lexer.tokenize(text);
        
        let has_section = tokens.iter().any(|t| t.kind == TokenKind::KeywordType);
        assert!(has_section);
    }

    #[test]
    fn test_toml_values() {
        let lexer = TomlLexer;
        let text = b"enabled = true\ncount = 42";
        let tokens = lexer.tokenize(text);
        
        let has_bool = tokens.iter().any(|t| t.kind == TokenKind::Boolean);
        let has_number = tokens.iter().any(|t| t.kind == TokenKind::Number);
        
        assert!(has_bool);
        assert!(has_number);
    }
}
