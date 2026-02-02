// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! High-performance CSS lexer with full language support.

use crate::syntax::lexer::{Lexer, is_whitespace, is_ident_start, is_ident_continue};
use crate::syntax::{Token, TokenKind};

pub struct CssLexer;

impl Lexer for CssLexer {
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

                // Block comment
                b'/' if pos + 1 < text.len() && text[pos + 1] == b'*' => {
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

                // At-rule (@media, @import, @keyframes, etc.)
                b'@' => {
                    pos += 1;
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'-') {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Keyword, start..pos));
                }

                // Hash (color or id selector)
                b'#' => {
                    pos += 1;
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'-') {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Number, start..pos));
                }

                // Class selector
                b'.' if pos + 1 < text.len() && (is_ident_start(text[pos + 1]) || text[pos + 1] == b'-') => {
                    pos += 1;
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'-') {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Keyword, start..pos));
                }

                // String literal (single or double quotes)
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

                // Number (including units like px, em, %, etc.)
                b'0'..=b'9' | b'-' if pos + 1 < text.len() && text[pos + 1].is_ascii_digit() => {
                    if b == b'-' {
                        pos += 1;
                    }
                    while pos < text.len() && (text[pos].is_ascii_digit() || text[pos] == b'.') {
                        pos += 1;
                    }
                    // Unit (px, em, rem, %, etc.)
                    if pos < text.len() && is_ident_start(text[pos]) {
                        while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'%') {
                            pos += 1;
                        }
                    } else if pos < text.len() && text[pos] == b'%' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Number, start..pos));
                }

                // Identifier (properties, values, selectors)
                _ if is_ident_start(b) || b == b'-' => {
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'-') {
                        pos += 1;
                    }
                    
                    let word = &text[start..pos];
                    let kind = match word {
                        // CSS keywords and values
                        b"important" | b"inherit" | b"initial" | b"unset" | b"auto" |
                        b"none" | b"normal" | b"bold" | b"italic" | b"block" | b"inline" |
                        b"flex" | b"grid" | b"absolute" | b"relative" | b"fixed" | b"sticky" |
                        b"left" | b"right" | b"center" | b"top" | b"bottom" |
                        b"hidden" | b"visible" | b"scroll" | b"solid" | b"dashed" | b"dotted" |
                        b"transparent" | b"currentColor" => TokenKind::Keyword,
                        
                        _ => TokenKind::Identifier,
                    };
                    tokens.push(Token::new(kind, start..pos));
                }

                // Operators and punctuation
                b'{' | b'}' | b'(' | b')' | b'[' | b']' | b':' | b';' | b',' |
                b'>' | b'+' | b'~' | b'*' | b'=' | b'^' | b'$' | b'|' => {
                    pos += 1;
                    // Handle multi-character operators
                    if pos < text.len() {
                        match (b, text[pos]) {
                            (b':', b':') | (b'*', b'=') | (b'^', b'=') | (b'$', b'=') | (b'|', b'=') => {
                                pos += 1;
                            }
                            _ => {}
                        }
                    }
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
                }

                // Other characters
                _ => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
                }
            }
        }

        tokens
    }
}
