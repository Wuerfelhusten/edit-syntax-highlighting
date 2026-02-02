// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! High-performance Go lexer with full language support.

use crate::syntax::lexer::{Lexer, is_whitespace, is_ident_start, is_ident_continue, is_ascii_digit};
use crate::syntax::{Token, TokenKind};

pub struct GoLexer;

impl Lexer for GoLexer {
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
                    while pos + 1 < text.len() {
                        if text[pos] == b'*' && text[pos + 1] == b'/' {
                            pos += 2;
                            break;
                        }
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Comment, start..pos));
                }

                // Raw string literal (`...`)
                b'`' => {
                    pos += 1;
                    while pos < text.len() && text[pos] != b'`' {
                        pos += 1;
                    }
                    if pos < text.len() {
                        pos += 1; // Skip closing backtick
                    }
                    tokens.push(Token::new(TokenKind::String, start..pos));
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

                // Rune literal (character)
                b'\'' => {
                    pos += 1;
                    let mut escaped = false;
                    while pos < text.len() {
                        if escaped {
                            escaped = false;
                        } else if text[pos] == b'\\' {
                            escaped = true;
                        } else if text[pos] == b'\'' {
                            pos += 1;
                            break;
                        }
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Char, start..pos));
                }

                // Number
                b'0'..=b'9' => {
                    // Hex literal
                    if b == b'0' && pos + 1 < text.len() && (text[pos + 1] == b'x' || text[pos + 1] == b'X') {
                        pos += 2;
                        while pos < text.len() && (is_ascii_digit(text[pos]) || matches!(text[pos], b'a'..=b'f' | b'A'..=b'F' | b'_')) {
                            pos += 1;
                        }
                    }
                    // Octal literal (0o prefix)
                    else if b == b'0' && pos + 1 < text.len() && (text[pos + 1] == b'o' || text[pos + 1] == b'O') {
                        pos += 2;
                        while pos < text.len() && (matches!(text[pos], b'0'..=b'7') || text[pos] == b'_') {
                            pos += 1;
                        }
                    }
                    // Binary literal
                    else if b == b'0' && pos + 1 < text.len() && (text[pos + 1] == b'b' || text[pos + 1] == b'B') {
                        pos += 2;
                        while pos < text.len() && (text[pos] == b'0' || text[pos] == b'1' || text[pos] == b'_') {
                            pos += 1;
                        }
                    }
                    // Decimal literal
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
                    // Imaginary suffix (i)
                    if pos < text.len() && text[pos] == b'i' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Number, start..pos));
                }

                // Identifier or keyword
                _ if is_ident_start(b) => {
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'_') {
                        pos += 1;
                    }
                    let word = &text[start..pos];
                    let kind = match word {
                        // Go keywords
                        b"break" | b"case" | b"chan" | b"const" | b"continue" |
                        b"default" | b"defer" | b"else" | b"fallthrough" | b"for" |
                        b"func" | b"go" | b"goto" | b"if" | b"import" | b"interface" |
                        b"map" | b"package" | b"range" | b"return" | b"select" |
                        b"struct" | b"switch" | b"type" | b"var" => TokenKind::Keyword,
                        
                        // Boolean literals
                        b"true" | b"false" => TokenKind::Boolean,
                        
                        // Nil
                        b"nil" => TokenKind::Boolean,
                        
                        // Built-in types
                        b"bool" | b"byte" | b"complex64" | b"complex128" | b"error" |
                        b"float32" | b"float64" | b"int" | b"int8" | b"int16" |
                        b"int32" | b"int64" | b"rune" | b"string" | b"uint" |
                        b"uint8" | b"uint16" | b"uint32" | b"uint64" | b"uintptr" => TokenKind::TypeName,
                        
                        // Built-in functions
                        b"append" | b"cap" | b"close" | b"complex" | b"copy" |
                        b"delete" | b"imag" | b"len" | b"make" | b"new" |
                        b"panic" | b"print" | b"println" | b"real" | b"recover" => TokenKind::FunctionName,
                        
                        // Special identifiers
                        b"iota" => TokenKind::Keyword,
                        
                        _ => TokenKind::Identifier,
                    };
                    tokens.push(Token::new(kind, start..pos));
                }

                // Operators and punctuation
                b'+' | b'-' | b'*' | b'/' | b'%' | b'=' | b'!' | b'<' | b'>' |
                b'&' | b'|' | b'^' | b'~' | b'?' | b':' | b'.' | b',' | b';' |
                b'(' | b')' | b'{' | b'}' | b'[' | b']' => {
                    pos += 1;
                    // Handle multi-character operators
                    if pos < text.len() {
                        match (b, text[pos]) {
                            (b'+', b'+') | (b'-', b'-') | (b'+', b'=') | (b'-', b'=') |
                            (b'*', b'=') | (b'/', b'=') | (b'%', b'=') | (b'=', b'=') |
                            (b'!', b'=') | (b'<', b'<') | (b'>', b'>') | (b'<', b'=') |
                            (b'>', b'=') | (b'&', b'&') | (b'|', b'|') | (b'&', b'=') |
                            (b'|', b'=') | (b'^', b'=') | (b'<', b'-') | (b':', b'=') |
                            (b'.', b'.') => {
                                pos += 1;
                                // Handle three-character operators
                                if pos < text.len() {
                                    match (b, text[pos - 1], text[pos]) {
                                        (b'<', b'<', b'=') | (b'>', b'>', b'=') |
                                        (b'.', b'.', b'.') | (b'&', b'^', b'=') => {
                                            pos += 1;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
                }

                // Unknown character
                _ => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Error, start..pos));
                }
            }
        }

        tokens
    }
}
