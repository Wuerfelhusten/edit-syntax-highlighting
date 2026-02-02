// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! High-performance C++ lexer with full language support.

use crate::syntax::lexer::{Lexer, is_whitespace, is_ident_start, is_ident_continue, is_ascii_digit};
use crate::syntax::{Token, TokenKind};

pub struct CppLexer;

impl Lexer for CppLexer {
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

                // Raw string literal (C++11)
                b'R' if pos + 2 < text.len() && text[pos + 1] == b'"' && text[pos + 2] == b'(' => {
                    pos += 3;
                    // Find delimiter
                    let delim_start = pos;
                    while pos < text.len() && text[pos] != b')' {
                        pos += 1;
                    }
                    let delimiter = &text[delim_start..pos];
                    if pos < text.len() {
                        pos += 1; // Skip ')'
                    }
                    
                    // Find closing sequence: )delimiter"
                    while pos < text.len() {
                        if text[pos] == b')' {
                            let mut match_pos = 0;
                            let mut temp_pos = pos + 1;
                            while match_pos < delimiter.len() && temp_pos < text.len() 
                                  && text[temp_pos] == delimiter[match_pos] {
                                match_pos += 1;
                                temp_pos += 1;
                            }
                            if match_pos == delimiter.len() && temp_pos < text.len() && text[temp_pos] == b'"' {
                                pos = temp_pos + 1;
                                break;
                            }
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
                        while pos < text.len() && (is_ascii_digit(text[pos]) || matches!(text[pos], b'a'..=b'f' | b'A'..=b'F' | b'_' | b'\'')) {
                            pos += 1;
                        }
                    }
                    // Binary literal (C++14)
                    else if b == b'0' && pos + 1 < text.len() && (text[pos + 1] == b'b' || text[pos + 1] == b'B') {
                        pos += 2;
                        while pos < text.len() && (text[pos] == b'0' || text[pos] == b'1' || text[pos] == b'_' || text[pos] == b'\'') {
                            pos += 1;
                        }
                    }
                    // Octal literal
                    else if b == b'0' && pos + 1 < text.len() && matches!(text[pos + 1], b'0'..=b'7') {
                        pos += 1;
                        while pos < text.len() && (matches!(text[pos], b'0'..=b'7') || text[pos] == b'_' || text[pos] == b'\'') {
                            pos += 1;
                        }
                    }
                    // Decimal literal
                    else {
                        while pos < text.len() && (is_ascii_digit(text[pos]) || text[pos] == b'_' || text[pos] == b'\'') {
                            pos += 1;
                        }
                        // Float
                        if pos < text.len() && text[pos] == b'.' {
                            pos += 1;
                            while pos < text.len() && (is_ascii_digit(text[pos]) || text[pos] == b'_' || text[pos] == b'\'') {
                                pos += 1;
                            }
                        }
                        // Exponent
                        if pos < text.len() && (text[pos] == b'e' || text[pos] == b'E') {
                            pos += 1;
                            if pos < text.len() && (text[pos] == b'+' || text[pos] == b'-') {
                                pos += 1;
                            }
                            while pos < text.len() && (is_ascii_digit(text[pos]) || text[pos] == b'\'') {
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
                        // C++ keywords (includes all C keywords plus C++-specific)
                        b"alignas" | b"alignof" | b"and" | b"and_eq" | b"asm" | b"auto" |
                        b"bitand" | b"bitor" | b"bool" | b"break" | b"case" | b"catch" |
                        b"char" | b"char8_t" | b"char16_t" | b"char32_t" | b"class" |
                        b"compl" | b"concept" | b"const" | b"const_cast" | b"consteval" |
                        b"constexpr" | b"constinit" | b"continue" | b"co_await" | b"co_return" |
                        b"co_yield" | b"decltype" | b"default" | b"delete" | b"do" | b"double" |
                        b"dynamic_cast" | b"else" | b"enum" | b"explicit" | b"export" |
                        b"extern" | b"float" | b"for" | b"friend" | b"goto" |
                        b"if" | b"inline" | b"int" | b"long" | b"mutable" | b"namespace" |
                        b"new" | b"noexcept" | b"not" | b"not_eq" | b"operator" |
                        b"or" | b"or_eq" | b"private" | b"protected" | b"public" | b"register" |
                        b"reinterpret_cast" | b"requires" | b"return" | b"short" | b"signed" |
                        b"sizeof" | b"static" | b"static_assert" | b"static_cast" | b"struct" |
                        b"switch" | b"template" | b"this" | b"thread_local" | b"throw" |
                        b"try" | b"typedef" | b"typeid" | b"typename" | b"union" |
                        b"unsigned" | b"using" | b"virtual" | b"void" | b"volatile" |
                        b"wchar_t" | b"while" | b"xor" | b"xor_eq" => TokenKind::Keyword,
                        
                        // Boolean literals
                        b"true" | b"false" | b"TRUE" | b"FALSE" => TokenKind::Boolean,
                        
                        // nullptr
                        b"nullptr" | b"NULL" => TokenKind::Boolean,
                        
                        // Common STL types
                        b"string" | b"vector" | b"map" | b"set" | b"list" | b"deque" |
                        b"queue" | b"stack" | b"array" | b"pair" | b"tuple" | b"optional" |
                        b"variant" | b"any" | b"function" | b"shared_ptr" | b"unique_ptr" |
                        b"weak_ptr" | b"size_t" | b"ptrdiff_t" | b"nullptr_t" => TokenKind::TypeName,
                        
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
                                        (b'-', b'>', b'*') | (b'.', b'.', b'.') => {
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
