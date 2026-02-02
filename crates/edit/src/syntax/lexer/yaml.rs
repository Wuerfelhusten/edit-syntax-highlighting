// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! YAML configuration file lexer.

use crate::syntax::lexer::{Lexer, is_ident_start, is_ident_continue, is_ascii_digit};
use crate::syntax::{Token, TokenKind};

pub struct YamlLexer;

impl Lexer for YamlLexer {
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

                // Newline (significant in YAML)
                b'\n' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Whitespace, start..pos));
                }

                // Comments
                b'#' => {
                    while pos < text.len() && text[pos] != b'\n' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Comment, start..pos));
                }

                // Document markers (--- and ...)
                b'-' if pos + 2 < text.len() && text[pos + 1] == b'-' && text[pos + 2] == b'-' => {
                    pos += 3;
                    tokens.push(Token::new(TokenKind::Keyword, start..pos));
                }
                b'.' if pos + 2 < text.len() && text[pos + 1] == b'.' && text[pos + 2] == b'.' => {
                    pos += 3;
                    tokens.push(Token::new(TokenKind::Keyword, start..pos));
                }

                // Strings (quoted)
                b'"' | b'\'' => {
                    let quote = b;
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
                b'0'..=b'9' | b'+' | b'-' if (matches!(b, b'+' | b'-') && pos + 1 < text.len() && is_ascii_digit(text[pos + 1])) || is_ascii_digit(b) => {
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
                        while pos < text.len() && is_ascii_digit(text[pos]) {
                            pos += 1;
                        }
                    }
                    
                    tokens.push(Token::new(TokenKind::Number, start..pos));
                }

                // Special values
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
                b'~' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Null, start..pos));
                }

                // List item marker
                b'-' if pos + 1 < text.len() && matches!(text[pos + 1], b' ' | b'\n') => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
                }

                // Anchors and aliases
                b'&' | b'*' => {
                    pos += 1;
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'-') {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Label, start..pos));
                }

                // Tags
                b'!' => {
                    pos += 1;
                    while pos < text.len() && !matches!(text[pos], b' ' | b'\n' | b'\t') {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Attribute, start..pos));
                }

                // Keys and unquoted strings
                _ if is_ident_start(b) => {
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'-') {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Identifier, start..pos));
                }

                // Colon (key-value separator)
                b':' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Punctuation, start..pos));
                }

                // Other delimiters
                b'[' | b']' | b'{' | b'}' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Delimiter, start..pos));
                }
                b',' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Punctuation, start..pos));
                }

                // Pipe and fold markers
                b'|' | b'>' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
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
    fn test_yaml_key_value() {
        let lexer = YamlLexer;
        let text = b"key: value\nnumber: 42";
        let tokens = lexer.tokenize(text);
        
        let has_identifier = tokens.iter().any(|t| t.kind == TokenKind::Identifier);
        let has_number = tokens.iter().any(|t| t.kind == TokenKind::Number);
        
        assert!(has_identifier);
        assert!(has_number);
    }

    #[test]
    fn test_yaml_booleans() {
        let lexer = YamlLexer;
        let text = b"enabled: true\ndisabled: false";
        let tokens = lexer.tokenize(text);
        
        let bools: Vec<_> = tokens.iter().filter(|t| t.kind == TokenKind::Boolean).collect();
        assert_eq!(bools.len(), 2);
    }
}
