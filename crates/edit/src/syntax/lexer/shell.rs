// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! High-performance Shell/Bash lexer with full language support.

use crate::syntax::lexer::{Lexer, is_whitespace, is_ident_start, is_ident_continue, is_ascii_digit};
use crate::syntax::{Token, TokenKind};

pub struct ShellLexer;

impl Lexer for ShellLexer {
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

                // Comment
                b'#' => {
                    while pos < text.len() && text[pos] != b'\n' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Comment, start..pos));
                }

                // Variable expansion ($VAR, ${VAR}, $(...), $((...)))
                b'$' => {
                    pos += 1;
                    
                    // Special variables ($?, $!, $$, $@, $*, $#, etc.)
                    if pos < text.len() && matches!(text[pos], b'?' | b'!' | b'$' | b'@' | b'*' | b'#' | b'-' | b'0'..=b'9') {
                        pos += 1;
                        tokens.push(Token::new(TokenKind::VariableName, start..pos));
                    }
                    // Brace expansion ${VAR}
                    else if pos < text.len() && text[pos] == b'{' {
                        pos += 1;
                        while pos < text.len() && text[pos] != b'}' {
                            pos += 1;
                        }
                        if pos < text.len() {
                            pos += 1;
                        }
                        tokens.push(Token::new(TokenKind::VariableName, start..pos));
                    }
                    // Command substitution $(...)
                    else if pos < text.len() && text[pos] == b'(' {
                        pos += 1;
                        let mut depth = 1;
                        while pos < text.len() && depth > 0 {
                            if text[pos] == b'(' {
                                depth += 1;
                            } else if text[pos] == b')' {
                                depth -= 1;
                            }
                            pos += 1;
                        }
                        tokens.push(Token::new(TokenKind::VariableName, start..pos));
                    }
                    // Regular variable $VAR
                    else if pos < text.len() && (is_ident_start(text[pos]) || text[pos] == b'_') {
                        while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'_') {
                            pos += 1;
                        }
                        tokens.push(Token::new(TokenKind::VariableName, start..pos));
                    }
                    else {
                        tokens.push(Token::new(TokenKind::Operator, start..pos));
                    }
                }

                // Single-quoted string (no expansion)
                b'\'' => {
                    pos += 1;
                    while pos < text.len() && text[pos] != b'\'' {
                        pos += 1;
                    }
                    if pos < text.len() {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::String, start..pos));
                }

                // Double-quoted string (with expansion)
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

                // Backtick command substitution `...`
                b'`' => {
                    pos += 1;
                    let mut escaped = false;
                    while pos < text.len() {
                        if escaped {
                            escaped = false;
                        } else if text[pos] == b'\\' {
                            escaped = true;
                        } else if text[pos] == b'`' {
                            pos += 1;
                            break;
                        }
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::String, start..pos));
                }

                // Number
                b'0'..=b'9' => {
                    while pos < text.len() && is_ascii_digit(text[pos]) {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Number, start..pos));
                }

                // Operators and redirections
                b'>' | b'<' | b'|' | b'&' | b';' => {
                    pos += 1;
                    // Handle multi-character operators
                    if pos < text.len() {
                        match (b, text[pos]) {
                            (b'>', b'>') | (b'<', b'<') | (b'&', b'&') | (b'|', b'|') |
                            (b'>', b'&') | (b'<', b'&') | (b';', b';') => {
                                pos += 1;
                            }
                            _ => {}
                        }
                    }
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
                }

                // Other operators
                b'=' | b'+' | b'-' | b'*' | b'/' | b'%' | b'!' | b'~' | b'?' |
                b'(' | b')' | b'{' | b'}' | b'[' | b']' | b',' | b':' | b'.' => {
                    pos += 1;
                    // Handle multi-character operators
                    if pos < text.len() {
                        match (b, text[pos]) {
                            (b'=', b'=') | (b'!', b'=') | (b'+', b'=') | (b'-', b'=') |
                            (b'*', b'=') | (b'/', b'=') | (b'%', b'=') | (b'+', b'+') |
                            (b'-', b'-') | (b'=', b'~') => {
                                pos += 1;
                            }
                            _ => {}
                        }
                    }
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
                }

                // Identifier or keyword
                _ if is_ident_start(b) || b == b'_' => {
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'_' || text[pos] == b'-') {
                        pos += 1;
                    }
                    let word = &text[start..pos];
                    let kind = match word {
                        // Bash keywords
                        b"if" | b"then" | b"else" | b"elif" | b"fi" |
                        b"case" | b"esac" | b"for" | b"while" | b"until" | b"do" | b"done" |
                        b"in" | b"select" | b"time" | b"function" |
                        b"declare" | b"typeset" | b"local" | b"readonly" | b"export" |
                        b"unset" | b"return" | b"break" | b"continue" | b"exit" |
                        b"shift" | b"eval" | b"exec" | b"source" | b"alias" | b"unalias" => TokenKind::Keyword,
                        
                        // Conditional expressions
                        b"test" => TokenKind::Keyword,
                        
                        // Boolean values
                        b"true" | b"false" => TokenKind::Boolean,
                        
                        // Common commands (builtins)
                        b"echo" | b"printf" | b"read" | b"cd" | b"pwd" | b"pushd" | b"popd" |
                        b"ls" | b"cat" | b"grep" | b"sed" | b"awk" | b"find" | b"sort" | b"uniq" |
                        b"head" | b"tail" | b"cut" | b"paste" | b"tr" | b"wc" |
                        b"chmod" | b"chown" | b"chgrp" | b"mkdir" | b"rm" | b"cp" | b"mv" |
                        b"touch" | b"ln" | b"dirname" | b"basename" |
                        b"tar" | b"gzip" | b"gunzip" | b"zip" | b"unzip" |
                        b"ps" | b"top" | b"kill" | b"killall" | b"jobs" | b"bg" | b"fg" |
                        b"man" | b"which" | b"whereis" | b"type" | b"command" |
                        b"set" | b"shopt" | b"let" | b"wait" | b"sleep" | b"trap" => TokenKind::FunctionName,
                        
                        _ => TokenKind::Identifier,
                    };
                    tokens.push(Token::new(kind, start..pos));
                }

                // Everything else
                _ => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
                }
            }
        }

        tokens
    }
}
