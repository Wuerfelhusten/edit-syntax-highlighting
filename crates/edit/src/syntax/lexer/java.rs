// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! High-performance Java lexer with full language support.

use crate::syntax::lexer::{Lexer, is_whitespace, is_ident_start, is_ident_continue, is_ascii_digit};
use crate::syntax::{Token, TokenKind};

pub struct JavaLexer;

impl Lexer for JavaLexer {
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

                // Block comment or Javadoc
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

                // Annotation
                b'@' => {
                    pos += 1;
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'_') {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Attribute, start..pos));
                }

                // Text block (Java 15+) - must come before regular string
                b'"' if pos + 2 < text.len() && text[pos + 1] == b'"' && text[pos + 2] == b'"' => {
                    pos += 3;
                    while pos + 2 < text.len() {
                        if text[pos] == b'"' && text[pos + 1] == b'"' && text[pos + 2] == b'"' {
                            pos += 3;
                            break;
                        }
                        pos += 1;
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
                    // Binary literal
                    else if b == b'0' && pos + 1 < text.len() && (text[pos + 1] == b'b' || text[pos + 1] == b'B') {
                        pos += 2;
                        while pos < text.len() && (text[pos] == b'0' || text[pos] == b'1' || text[pos] == b'_') {
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
                    // Suffix (f, F, d, D, l, L)
                    if pos < text.len() && matches!(text[pos], b'f' | b'F' | b'd' | b'D' | b'l' | b'L') {
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
                        // Java keywords
                        b"abstract" | b"assert" | b"break" | b"case" | b"catch" |
                        b"class" | b"const" | b"continue" | b"default" | b"do" |
                        b"else" | b"enum" | b"extends" | b"final" | b"finally" |
                        b"for" | b"goto" | b"if" | b"implements" | b"import" |
                        b"instanceof" | b"interface" | b"native" | b"new" | b"package" |
                        b"private" | b"protected" | b"public" | b"return" | b"static" |
                        b"strictfp" | b"super" | b"switch" | b"synchronized" | b"this" |
                        b"throw" | b"throws" | b"transient" | b"try" | b"volatile" |
                        b"while" => TokenKind::Keyword,
                        
                        // Java 14+ keywords
                        b"record" | b"sealed" | b"non-sealed" | b"permits" | b"var" | b"yield" => TokenKind::Keyword,
                        
                        // Primitive types
                        b"boolean" | b"byte" | b"char" | b"short" | b"int" | b"long" |
                        b"float" | b"double" | b"void" => TokenKind::TypeName,
                        
                        // Boolean literals
                        b"true" | b"false" => TokenKind::Boolean,
                        
                        // Null
                        b"null" => TokenKind::Boolean,
                        
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
                            (b'|', b'=') | (b'^', b'=') | (b'-', b'>') | (b':', b':') => {
                                pos += 1;
                                // Handle three-character operators
                                if pos < text.len() {
                                    match (b, text[pos - 1], text[pos]) {
                                        (b'<', b'<', b'=') | (b'>', b'>', b'=') |
                                        (b'>', b'>', b'>') => {
                                            pos += 1;
                                            // Handle >>>= (four-character)
                                            if pos < text.len() && text[pos] == b'=' {
                                                pos += 1;
                                            }
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
