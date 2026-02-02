// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! High-performance C lexer with full language support.

use crate::syntax::lexer::{Lexer, is_whitespace, is_ident_start, is_ident_continue, is_ascii_digit};
use crate::syntax::{Token, TokenKind};

pub struct CLexer;

impl Lexer for CLexer {
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

                // Preprocessor directive
                b'#' => {
                    pos += 1;
                    while pos < text.len() && is_whitespace(text[pos]) {
                        pos += 1;
                    }
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'_') {
                        pos += 1;
                    }
                    // Continue to end of logical line (handles line continuation with \)
                    while pos < text.len() {
                        if text[pos] == b'\n' {
                            if pos > 0 && text[pos - 1] != b'\\' {
                                break;
                            }
                        }
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Macro, start..pos));
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

                // Character literal
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
                    // Octal literal
                    else if b == b'0' && pos + 1 < text.len() && matches!(text[pos + 1], b'0'..=b'7') {
                        pos += 1;
                        while pos < text.len() && (matches!(text[pos], b'0'..=b'7') || text[pos] == b'_') {
                            pos += 1;
                        }
                    }
                    // Binary literal (C23)
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
                        if pos < text.len() && text[pos] == b'.' {
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
                            while pos < text.len() && is_ascii_digit(text[pos]) {
                                pos += 1;
                            }
                        }
                    }
                    // Suffix (f, F, l, L, u, U, ll, LL, ul, UL, etc.)
                    while pos < text.len() && matches!(text[pos], b'f' | b'F' | b'l' | b'L' | b'u' | b'U') {
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
                        // C keywords
                        b"auto" | b"break" | b"case" | b"char" | b"const" | b"continue" |
                        b"default" | b"do" | b"double" | b"else" | b"enum" | b"extern" |
                        b"float" | b"for" | b"goto" | b"if" | b"inline" | b"int" | b"long" |
                        b"register" | b"restrict" | b"return" | b"short" | b"signed" |
                        b"sizeof" | b"static" | b"struct" | b"switch" | b"typedef" |
                        b"union" | b"unsigned" | b"void" | b"volatile" | b"while" |
                        b"_Alignas" | b"_Alignof" | b"_Atomic" | b"_Bool" | b"_Complex" |
                        b"_Generic" | b"_Imaginary" | b"_Noreturn" | b"_Static_assert" |
                        b"_Thread_local" => TokenKind::Keyword,
                        
                        // C23 keywords
                        b"_BitInt" | b"typeof" | b"typeof_unqual" |
                        b"_Decimal128" | b"_Decimal32" | b"_Decimal64" => TokenKind::Keyword,
                        
                        // Common constants
                        b"NULL" | b"true" | b"false" | b"TRUE" | b"FALSE" => TokenKind::Boolean,
                        
                        // Type names (common standard types)
                        b"size_t" | b"ssize_t" | b"ptrdiff_t" | b"intptr_t" | b"uintptr_t" |
                        b"int8_t" | b"int16_t" | b"int32_t" | b"int64_t" |
                        b"uint8_t" | b"uint16_t" | b"uint32_t" | b"uint64_t" |
                        b"FILE" | b"DIR" | b"time_t" | b"clock_t" | b"pid_t" => TokenKind::TypeName,
                        
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
                            (b'|', b'=') | (b'^', b'=') | (b'-', b'>') => {
                                pos += 1;
                                // Handle three-character operators
                                if pos < text.len() {
                                    match (b, text[pos - 1], text[pos]) {
                                        (b'<', b'<', b'=') | (b'>', b'>', b'=') => {
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
