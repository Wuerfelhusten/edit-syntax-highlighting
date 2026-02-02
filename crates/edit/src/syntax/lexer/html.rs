// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! High-performance HTML lexer with full language support.

use crate::syntax::lexer::{Lexer, is_whitespace};
use crate::syntax::{Token, TokenKind};

pub struct HtmlLexer;

impl Lexer for HtmlLexer {
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

                // HTML Comment
                b'<' if pos + 3 < text.len() && &text[pos..pos+4] == b"<!--" => {
                    pos += 4;
                    while pos + 2 < text.len() {
                        if &text[pos..pos+3] == b"-->" {
                            pos += 3;
                            break;
                        }
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Comment, start..pos));
                }

                // DOCTYPE declaration
                b'<' if pos + 1 < text.len() && text[pos + 1] == b'!' => {
                    pos += 2;
                    while pos < text.len() && text[pos] != b'>' {
                        pos += 1;
                    }
                    if pos < text.len() {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Keyword, start..pos));
                }

                // Closing tag
                b'<' if pos + 1 < text.len() && text[pos + 1] == b'/' => {
                    pos += 2;
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
                    
                    // Tag name
                    let tag_start = pos;
                    while pos < text.len() && !is_whitespace(text[pos]) && text[pos] != b'>' {
                        pos += 1;
                    }
                    if pos > tag_start {
                        tokens.push(Token::new(TokenKind::Keyword, tag_start..pos));
                    }
                    
                    // Skip whitespace
                    while pos < text.len() && is_whitespace(text[pos]) {
                        let ws_start = pos;
                        pos += 1;
                        while pos < text.len() && is_whitespace(text[pos]) {
                            pos += 1;
                        }
                        tokens.push(Token::new(TokenKind::Whitespace, ws_start..pos));
                    }
                    
                    // Closing >
                    if pos < text.len() && text[pos] == b'>' {
                        tokens.push(Token::new(TokenKind::Operator, pos..pos+1));
                        pos += 1;
                    }
                }

                // Opening tag or self-closing tag
                b'<' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
                    
                    // Tag name
                    let tag_start = pos;
                    while pos < text.len() && !is_whitespace(text[pos]) && text[pos] != b'>' && text[pos] != b'/' {
                        pos += 1;
                    }
                    if pos > tag_start {
                        tokens.push(Token::new(TokenKind::Keyword, tag_start..pos));
                    }
                    
                    // Attributes
                    loop {
                        // Skip whitespace
                        if pos < text.len() && is_whitespace(text[pos]) {
                            let ws_start = pos;
                            while pos < text.len() && is_whitespace(text[pos]) {
                                pos += 1;
                            }
                            tokens.push(Token::new(TokenKind::Whitespace, ws_start..pos));
                        }
                        
                        // Check for end of tag
                        if pos >= text.len() || text[pos] == b'>' || (text[pos] == b'/' && pos + 1 < text.len() && text[pos + 1] == b'>') {
                            break;
                        }
                        
                        // Attribute name
                        let attr_start = pos;
                        while pos < text.len() && !is_whitespace(text[pos]) && text[pos] != b'=' && text[pos] != b'>' && text[pos] != b'/' {
                            pos += 1;
                        }
                        if pos > attr_start {
                            tokens.push(Token::new(TokenKind::PropertyName, attr_start..pos));
                        }
                        
                        // Skip whitespace around =
                        while pos < text.len() && is_whitespace(text[pos]) {
                            let ws_start = pos;
                            pos += 1;
                            while pos < text.len() && is_whitespace(text[pos]) {
                                pos += 1;
                            }
                            tokens.push(Token::new(TokenKind::Whitespace, ws_start..pos));
                        }
                        
                        // Equals sign
                        if pos < text.len() && text[pos] == b'=' {
                            tokens.push(Token::new(TokenKind::Operator, pos..pos+1));
                            pos += 1;
                            
                            // Skip whitespace after =
                            while pos < text.len() && is_whitespace(text[pos]) {
                                let ws_start = pos;
                                pos += 1;
                                while pos < text.len() && is_whitespace(text[pos]) {
                                    pos += 1;
                                }
                                tokens.push(Token::new(TokenKind::Whitespace, ws_start..pos));
                            }
                            
                            // Attribute value
                            if pos < text.len() {
                                let quote = text[pos];
                                if quote == b'"' || quote == b'\'' {
                                    let value_start = pos;
                                    pos += 1;
                                    while pos < text.len() && text[pos] != quote {
                                        pos += 1;
                                    }
                                    if pos < text.len() {
                                        pos += 1;
                                    }
                                    tokens.push(Token::new(TokenKind::String, value_start..pos));
                                }
                            }
                        }
                    }
                    
                    // Self-closing />
                    if pos + 1 < text.len() && text[pos] == b'/' && text[pos + 1] == b'>' {
                        tokens.push(Token::new(TokenKind::Operator, pos..pos+2));
                        pos += 2;
                    }
                    // Closing >
                    else if pos < text.len() && text[pos] == b'>' {
                        tokens.push(Token::new(TokenKind::Operator, pos..pos+1));
                        pos += 1;
                    }
                }

                // Text content
                _ => {
                    while pos < text.len() && text[pos] != b'<' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Identifier, start..pos));
                }
            }
        }

        tokens
    }
}
