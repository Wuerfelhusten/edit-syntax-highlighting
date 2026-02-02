// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! High-performance C# lexer with full language support.

use crate::syntax::lexer::{Lexer, is_whitespace, is_ident_start, is_ident_continue, is_ascii_digit};
use crate::syntax::{Token, TokenKind};

pub struct CSharpLexer;

impl Lexer for CSharpLexer {
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
                    while pos < text.len() && text[pos] != b'\n' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Macro, start..pos));
                }

                // Attribute
                b'[' if pos + 1 < text.len() && is_ident_start(text[pos + 1]) => {
                    pos += 1;
                    let _attr_start = pos;
                    while pos < text.len() && text[pos] != b']' {
                        pos += 1;
                    }
                    if pos < text.len() {
                        pos += 1; // Skip closing ]
                    }
                    tokens.push(Token::new(TokenKind::Attribute, start..pos));
                }

                // Verbatim string (@"...")
                b'@' if pos + 1 < text.len() && text[pos + 1] == b'"' => {
                    pos += 2;
                    while pos < text.len() {
                        if text[pos] == b'"' {
                            pos += 1;
                            // Check for escaped quote ("")
                            if pos < text.len() && text[pos] == b'"' {
                                pos += 1;
                            } else {
                                break;
                            }
                        } else {
                            pos += 1;
                        }
                    }
                    tokens.push(Token::new(TokenKind::String, start..pos));
                }

                // Interpolated string ($"..." or $@"...")
                b'$' if pos + 1 < text.len() && (text[pos + 1] == b'"' || 
                        (pos + 2 < text.len() && text[pos + 1] == b'@' && text[pos + 2] == b'"')) => {
                    pos += 1;
                    let verbatim = if text[pos] == b'@' {
                        pos += 1;
                        true
                    } else {
                        false
                    };
                    pos += 1; // Skip opening "
                    
                    let mut brace_depth = 0;
                    let mut escaped = false;
                    while pos < text.len() {
                        if verbatim {
                            if text[pos] == b'"' {
                                pos += 1;
                                if pos < text.len() && text[pos] == b'"' {
                                    pos += 1; // Escaped quote
                                } else {
                                    break;
                                }
                            } else if text[pos] == b'{' {
                                if pos + 1 < text.len() && text[pos + 1] == b'{' {
                                    pos += 2; // Escaped brace
                                } else {
                                    brace_depth += 1;
                                    pos += 1;
                                }
                            } else if text[pos] == b'}' {
                                if brace_depth > 0 {
                                    brace_depth -= 1;
                                }
                                pos += 1;
                            } else {
                                pos += 1;
                            }
                        } else {
                            if escaped {
                                escaped = false;
                                pos += 1;
                            } else if text[pos] == b'\\' {
                                escaped = true;
                                pos += 1;
                            } else if text[pos] == b'"' && brace_depth == 0 {
                                pos += 1;
                                break;
                            } else if text[pos] == b'{' {
                                if pos + 1 < text.len() && text[pos + 1] == b'{' {
                                    pos += 2; // Escaped brace
                                } else {
                                    brace_depth += 1;
                                    pos += 1;
                                }
                            } else if text[pos] == b'}' {
                                if brace_depth > 0 {
                                    brace_depth -= 1;
                                }
                                pos += 1;
                            } else {
                                pos += 1;
                            }
                        }
                    }
                    tokens.push(Token::new(TokenKind::String, start..pos));
                }

                // Regular string literal
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
                            while pos < text.len() && is_ascii_digit(text[pos]) {
                                pos += 1;
                            }
                        }
                    }
                    // Suffix (f, F, d, D, m, M, l, L, u, U, ul, UL, etc.)
                    while pos < text.len() && matches!(text[pos], b'f' | b'F' | b'd' | b'D' | b'm' | b'M' | b'l' | b'L' | b'u' | b'U') {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Number, start..pos));
                }

                // Identifier or keyword
                _ if is_ident_start(b) || b == b'@' => {
                    if b == b'@' {
                        pos += 1; // Skip @ for verbatim identifier
                    }
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'_') {
                        pos += 1;
                    }
                    let word = &text[start..pos];
                    let kind = match word {
                        // C# keywords
                        b"abstract" | b"as" | b"base" | b"bool" | b"break" | b"byte" |
                        b"case" | b"catch" | b"char" | b"checked" | b"class" | b"const" |
                        b"continue" | b"decimal" | b"default" | b"delegate" | b"do" |
                        b"double" | b"else" | b"enum" | b"event" | b"explicit" | b"extern" |
                        b"finally" | b"fixed" | b"float" | b"for" | b"foreach" |
                        b"goto" | b"if" | b"implicit" | b"in" | b"int" | b"interface" |
                        b"internal" | b"is" | b"lock" | b"long" | b"namespace" | b"new" |
                        b"object" | b"operator" | b"out" | b"override" | b"params" |
                        b"private" | b"protected" | b"public" | b"readonly" | b"ref" |
                        b"return" | b"sbyte" | b"sealed" | b"short" | b"sizeof" | b"stackalloc" |
                        b"static" | b"string" | b"struct" | b"switch" | b"this" | b"throw" |
                        b"try" | b"typeof" | b"uint" | b"ulong" | b"unchecked" |
                        b"unsafe" | b"ushort" | b"using" | b"virtual" | b"void" | b"volatile" |
                        b"while" => TokenKind::Keyword,
                        
                        // Contextual keywords
                        b"add" | b"alias" | b"ascending" | b"async" | b"await" | b"by" |
                        b"descending" | b"dynamic" | b"equals" | b"from" | b"get" | b"global" |
                        b"group" | b"into" | b"join" | b"let" | b"nameof" | b"on" | b"orderby" |
                        b"partial" | b"remove" | b"select" | b"set" | b"value" | b"var" |
                        b"when" | b"where" | b"yield" => TokenKind::Keyword,
                        
                        // C# 9.0+ keywords
                        b"record" | b"init" | b"with" | b"nint" | b"nuint" => TokenKind::Keyword,
                        
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
                            (b'|', b'=') | (b'^', b'=') | (b'=', b'>') | (b'?', b'?') |
                            (b'?', b'.') => {
                                pos += 1;
                                // Handle three-character operators
                                if pos < text.len() {
                                    match (b, text[pos - 1], text[pos]) {
                                        (b'<', b'<', b'=') | (b'>', b'>', b'=') |
                                        (b'?', b'?', b'=') => {
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
