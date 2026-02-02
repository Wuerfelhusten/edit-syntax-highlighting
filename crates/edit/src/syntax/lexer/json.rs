// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! High-performance JSON lexer with JSONC (JSON with comments) support.

use crate::syntax::lexer::{Lexer, is_ascii_digit};
use crate::syntax::{Token, TokenKind};

pub struct JsonLexer;

#[inline]
fn is_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r')
}

impl Lexer for JsonLexer {
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

                // String
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

                // Numbers
                b'-' | b'0'..=b'9' => {
                    pos += 1;
                    
                    // Integer part
                    if text[start] == b'-' && pos < text.len() {
                        // negative number
                    }
                    
                    // Skip digits
                    while pos < text.len() && is_ascii_digit(text[pos]) {
                        pos += 1;
                    }
                    
                    // Decimal part
                    if pos < text.len() && text[pos] == b'.' {
                        pos += 1;
                        while pos < text.len() && is_ascii_digit(text[pos]) {
                            pos += 1;
                        }
                    }
                    
                    // Exponent
                    if pos < text.len() && (text[pos] == b'e' || text[pos] == b'E') {
                        pos += 1;
                        if pos < text.len() && (text[pos] == b'+' || text[pos] == b'-') {
                            pos += 1;
                        }
                        while pos < text.len() && is_ascii_digit(text[pos]) {
                            pos += 1;
                        }
                    }
                    
                    tokens.push(Token::new(TokenKind::Number, start..pos));
                }

                // Comments (JSONC extension)
                b'/' if pos + 1 < text.len() => {
                    match text[pos + 1] {
                        // Line comment
                        b'/' => {
                            pos += 2;
                            while pos < text.len() && text[pos] != b'\n' {
                                pos += 1;
                            }
                            tokens.push(Token::new(TokenKind::Comment, start..pos));
                        }
                        // Block comment
                        b'*' => {
                            pos += 2;
                            while pos + 1 < text.len() {
                                if text[pos] == b'*' && text[pos + 1] == b'/' {
                                    pos += 2;
                                    break;
                                }
                                pos += 1;
                            }
                            tokens.push(Token::new(TokenKind::Comment, start..pos));
                        }
                        _ => {
                            // Not a comment, treat as error
                            pos += 1;
                            tokens.push(Token::new(TokenKind::Error, start..pos));
                        }
                    }
                }

                // Keywords: true, false, null
                b't' if pos + 4 <= text.len() && &text[pos..pos + 4] == b"true" => {
                    pos += 4;
                    tokens.push(Token::new(TokenKind::Boolean, start..pos));
                }
                b'f' if pos + 5 <= text.len() && &text[pos..pos + 5] == b"false" => {
                    pos += 5;
                    tokens.push(Token::new(TokenKind::Boolean, start..pos));
                }
                b'n' if pos + 4 <= text.len() && &text[pos..pos + 4] == b"null" => {
                    pos += 4;
                    tokens.push(Token::new(TokenKind::Null, start..pos));
                }

                // Delimiters and operators
                b'{' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::JsonBrace, start..pos));
                }
                b'}' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::JsonBrace, start..pos));
                }
                b'[' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::JsonBracket, start..pos));
                }
                b']' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::JsonBracket, start..pos));
                }
                b':' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::JsonColon, start..pos));
                }
                b',' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::JsonComma, start..pos));
                }

                // Error: unexpected character
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
    fn test_json_simple() {
        let lexer = JsonLexer;
        let text = br#"{"key": "value"}"#;
        let tokens = lexer.tokenize(text);
        
        assert_eq!(tokens[0].kind, TokenKind::JsonBrace); // {
        assert_eq!(tokens[1].kind, TokenKind::String);    // "key"
        assert_eq!(tokens[2].kind, TokenKind::JsonColon); // :
    }

    #[test]
    fn test_json_numbers() {
        let lexer = JsonLexer;
        let text = b"[42, -3.14, 1.5e-10]";
        let tokens = lexer.tokenize(text);
        
        let numbers: Vec<_> = tokens.iter()
            .filter(|t| t.kind == TokenKind::Number)
            .collect();
        
        assert_eq!(numbers.len(), 3);
    }

    #[test]
    fn test_json_keywords() {
        let lexer = JsonLexer;
        let text = b"[true, false, null]";
        let tokens = lexer.tokenize(text);
        
        let has_bool = tokens.iter().any(|t| t.kind == TokenKind::Boolean);
        let has_null = tokens.iter().any(|t| t.kind == TokenKind::Null);
        
        assert!(has_bool);
        assert!(has_null);
    }

    #[test]
    fn test_jsonc_comments() {
        let lexer = JsonLexer;
        let text = b"// line comment\n/* block comment */ {}";
        let tokens = lexer.tokenize(text);
        
        let comments: Vec<_> = tokens.iter()
            .filter(|t| t.kind == TokenKind::Comment)
            .collect();
        
        assert_eq!(comments.len(), 2);
    }
}
