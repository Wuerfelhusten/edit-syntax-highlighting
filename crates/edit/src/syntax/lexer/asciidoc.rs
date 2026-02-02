// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! High-performance AsciiDoc lexer with full language support.

use crate::syntax::lexer::{Lexer, is_whitespace, is_ident_continue, is_ascii_digit};
use crate::syntax::{Token, TokenKind};

pub struct AsciiDocLexer;

impl Lexer for AsciiDocLexer {
    fn tokenize(&self, text: &[u8]) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(text.len() / 8);
        let mut pos = 0;
        let mut line_start = true;

        while pos < text.len() {
            let start = pos;
            let b = text[pos];

            // Check for line-start constructs (headings, lists, blocks)
            if line_start {
                // Document title (= Title)
                if b == b'=' {
                    let heading_start = pos;
                    while pos < text.len() && text[pos] == b'=' {
                        pos += 1;
                    }
                    // Skip optional space
                    if pos < text.len() && text[pos] == b' ' {
                        pos += 1;
                    }
                    // Rest of line is heading
                    while pos < text.len() && text[pos] != b'\n' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Keyword, heading_start..pos));
                    line_start = text.get(pos) == Some(&b'\n');
                    if line_start {
                        pos += 1;
                    }
                    continue;
                }

                // Block delimiters (----, ****, ====, etc.)
                if matches!(b, b'-' | b'*' | b'=' | b'_' | b'.' | b'/') {
                    let delimiter_start = pos;
                    let delimiter_char = b;
                    let mut count = 0;
                    while pos < text.len() && text[pos] == delimiter_char {
                        count += 1;
                        pos += 1;
                    }
                    // Block delimiters are 4+ repeated characters
                    if count >= 4 && (pos >= text.len() || text[pos] == b'\n' || is_whitespace(text[pos])) {
                        while pos < text.len() && text[pos] != b'\n' {
                            pos += 1;
                        }
                        tokens.push(Token::new(TokenKind::Operator, delimiter_start..pos));
                        line_start = text.get(pos) == Some(&b'\n');
                        if line_start {
                            pos += 1;
                        }
                        continue;
                    } else {
                        // Not a block delimiter, reset
                        pos = start;
                    }
                }

                // Attribute entry (":name: value")
                if b == b':' && pos + 1 < text.len() && text[pos + 1] != b':' {
                    let attr_start = pos;
                    pos += 1;
                    // Attribute name
                    while pos < text.len() && text[pos] != b':' && text[pos] != b'\n' {
                        pos += 1;
                    }
                    if pos < text.len() && text[pos] == b':' {
                        pos += 1;
                        // Attribute value
                        while pos < text.len() && text[pos] != b'\n' {
                            pos += 1;
                        }
                        tokens.push(Token::new(TokenKind::Attribute, attr_start..pos));
                        line_start = text.get(pos) == Some(&b'\n');
                        if line_start {
                            pos += 1;
                        }
                        continue;
                    } else {
                        pos = start;
                    }
                }

                // Block title (.Title)
                if b == b'.' && pos + 1 < text.len() && !is_ascii_digit(text[pos + 1]) && text[pos + 1] != b' ' {
                    pos += 1;
                    while pos < text.len() && text[pos] != b'\n' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::PropertyName, start..pos));
                    line_start = text.get(pos) == Some(&b'\n');
                    if line_start {
                        pos += 1;
                    }
                    continue;
                }

                // Unordered list (* or **)
                if b == b'*' && pos + 1 < text.len() && (text[pos + 1] == b' ' || text[pos + 1] == b'*') {
                    while pos < text.len() && text[pos] == b'*' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
                    line_start = false;
                    continue;
                }

                // Ordered list (. or ..)
                if b == b'.' && pos + 1 < text.len() && (text[pos + 1] == b' ' || text[pos + 1] == b'.') {
                    let list_start = pos;
                    while pos < text.len() && text[pos] == b'.' {
                        pos += 1;
                    }
                    if pos < text.len() && text[pos] == b' ' {
                        tokens.push(Token::new(TokenKind::Operator, list_start..pos));
                        line_start = false;
                        continue;
                    } else {
                        pos = start;
                    }
                }

                // Line comment (//)
                if b == b'/' && pos + 1 < text.len() && text[pos + 1] == b'/' && 
                   (pos + 2 >= text.len() || text[pos + 2] != b'/') {
                    while pos < text.len() && text[pos] != b'\n' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Comment, start..pos));
                    line_start = text.get(pos) == Some(&b'\n');
                    if line_start {
                        pos += 1;
                    }
                    continue;
                }

                line_start = false;
            }

            match b {
                // Whitespace
                b' ' | b'\t' | b'\r' => {
                    while pos < text.len() && matches!(text[pos], b' ' | b'\t' | b'\r') {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Whitespace, start..pos));
                }

                // Newline
                b'\n' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Whitespace, start..pos));
                    line_start = true;
                }

                // Inline formatting (bold, italic, monospace, etc.)
                b'*' | b'_' | b'`' | b'+' | b'#' | b'^' | b'~' => {
                    // Check for constrained or unconstrained formatting
                    let format_char = b;
                    let is_double = pos + 1 < text.len() && text[pos + 1] == format_char;
                    
                    if is_double {
                        pos += 2;
                    } else {
                        pos += 1;
                    }
                    
                    // Find matching delimiter
                    let mut found_end = false;
                    while pos < text.len() {
                        if text[pos] == format_char {
                            if is_double {
                                if pos + 1 < text.len() && text[pos + 1] == format_char {
                                    pos += 2;
                                    found_end = true;
                                    break;
                                }
                            } else {
                                pos += 1;
                                found_end = true;
                                break;
                            }
                        }
                        if text[pos] == b'\n' {
                            break;
                        }
                        pos += 1;
                    }
                    
                    tokens.push(Token::new(
                        if found_end { TokenKind::String } else { TokenKind::Identifier },
                        start..pos
                    ));
                }

                // Link syntax (https://example.com or link:url[text])
                b'h' if pos + 7 < text.len() && &text[pos..pos + 7] == b"http://" ||
                        pos + 8 < text.len() && &text[pos..pos + 8] == b"https://" => {
                    while pos < text.len() && !is_whitespace(text[pos]) && text[pos] != b'[' {
                        pos += 1;
                    }
                    // Optional [text] after URL
                    if pos < text.len() && text[pos] == b'[' {
                        pos += 1;
                        while pos < text.len() && text[pos] != b']' {
                            pos += 1;
                        }
                        if pos < text.len() {
                            pos += 1;
                        }
                    }
                    tokens.push(Token::new(TokenKind::String, start..pos));
                }

                // Macro (image::url[] or include::file[])
                b'a'..=b'z' | b'A'..=b'Z' => {
                    let macro_start = pos;
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'-') {
                        pos += 1;
                    }
                    
                    // Check for :: (macro syntax)
                    if pos + 1 < text.len() && text[pos] == b':' && text[pos + 1] == b':' {
                        pos += 2;
                        // Target
                        while pos < text.len() && text[pos] != b'[' && text[pos] != b'\n' {
                            pos += 1;
                        }
                        // Attributes in []
                        if pos < text.len() && text[pos] == b'[' {
                            pos += 1;
                            let mut depth = 1;
                            while pos < text.len() && depth > 0 {
                                if text[pos] == b'[' {
                                    depth += 1;
                                } else if text[pos] == b']' {
                                    depth -= 1;
                                }
                                pos += 1;
                            }
                        }
                        tokens.push(Token::new(TokenKind::Macro, macro_start..pos));
                    } else {
                        tokens.push(Token::new(TokenKind::Identifier, macro_start..pos));
                    }
                }

                // Attribute reference {name}
                b'{' => {
                    pos += 1;
                    let attr_ref_start = pos;
                    while pos < text.len() && text[pos] != b'}' && text[pos] != b'\n' {
                        pos += 1;
                    }
                    if pos < text.len() && text[pos] == b'}' {
                        pos += 1;
                        tokens.push(Token::new(TokenKind::VariableName, start..pos));
                    } else {
                        tokens.push(Token::new(TokenKind::Identifier, start..attr_ref_start));
                    }
                }

                // Everything else
                _ => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Identifier, start..pos));
                }
            }
        }

        tokens
    }
}
