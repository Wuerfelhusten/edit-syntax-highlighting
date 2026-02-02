// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Markdown lexer with support for common formatting.

use crate::syntax::lexer::Lexer;
use crate::syntax::{Token, TokenKind};

pub struct MarkdownLexer;

impl Lexer for MarkdownLexer {
    fn tokenize(&self, text: &[u8]) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(text.len() / 16);
        let mut pos = 0;

        while pos < text.len() {
            let start = pos;
            let b = text[pos];

            // Check if we're at the start of a line for headings
            let at_line_start = pos == 0 || (pos > 0 && text[pos - 1] == b'\n');

            match b {
                // Headings (must be at line start)
                b'#' if at_line_start => {
                    let mut level = 0;
                    while pos < text.len() && text[pos] == b'#' && level < 6 {
                        pos += 1;
                        level += 1;
                    }
                    // Consume the rest of the line
                    while pos < text.len() && text[pos] != b'\n' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::MarkdownHeading, start..pos));
                }

                // Code blocks with backticks
                b'`' if pos + 2 < text.len() && text[pos + 1] == b'`' && text[pos + 2] == b'`' => {
                    pos += 3;
                    // Find the closing ```
                    while pos + 2 < text.len() {
                        if text[pos] == b'`' && text[pos + 1] == b'`' && text[pos + 2] == b'`' {
                            pos += 3;
                            break;
                        }
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::MarkdownCode, start..pos));
                }

                // Inline code
                b'`' => {
                    pos += 1;
                    while pos < text.len() && text[pos] != b'`' {
                        pos += 1;
                    }
                    if pos < text.len() {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::MarkdownCode, start..pos));
                }

                // Bold with **
                b'*' if pos + 1 < text.len() && text[pos + 1] == b'*' => {
                    pos += 2;
                    while pos + 1 < text.len() {
                        if text[pos] == b'*' && text[pos + 1] == b'*' {
                            pos += 2;
                            break;
                        }
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::MarkdownBold, start..pos));
                }

                // Italic with *
                b'*' => {
                    pos += 1;
                    while pos < text.len() && text[pos] != b'*' {
                        pos += 1;
                    }
                    if pos < text.len() {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::MarkdownItalic, start..pos));
                }

                // Bold with __
                b'_' if pos + 1 < text.len() && text[pos + 1] == b'_' => {
                    pos += 2;
                    while pos + 1 < text.len() {
                        if text[pos] == b'_' && text[pos + 1] == b'_' {
                            pos += 2;
                            break;
                        }
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::MarkdownBold, start..pos));
                }

                // Italic with _
                b'_' => {
                    pos += 1;
                    while pos < text.len() && text[pos] != b'_' {
                        pos += 1;
                    }
                    if pos < text.len() {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::MarkdownItalic, start..pos));
                }

                // Links [text](url)
                b'[' => {
                    pos += 1;
                    // Find closing ]
                    while pos < text.len() && text[pos] != b']' {
                        pos += 1;
                    }
                    if pos < text.len() {
                        pos += 1;
                        // Check for (url)
                        if pos < text.len() && text[pos] == b'(' {
                            while pos < text.len() && text[pos] != b')' {
                                pos += 1;
                            }
                            if pos < text.len() {
                                pos += 1;
                            }
                        }
                    }
                    tokens.push(Token::new(TokenKind::MarkdownLink, start..pos));
                }

                // Regular text - consume until next special character
                _ => {
                    while pos < text.len() {
                        let ch = text[pos];
                        if matches!(ch, b'#' | b'`' | b'*' | b'_' | b'[' | b'\n') {
                            break;
                        }
                        pos += 1;
                    }
                    // Don't create empty tokens
                    if pos > start {
                        tokens.push(Token::new(TokenKind::Identifier, start..pos));
                    } else {
                        pos += 1;
                    }
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
    fn test_markdown_headings() {
        let lexer = MarkdownLexer;
        let text = b"# Heading 1\n## Heading 2";
        let tokens = lexer.tokenize(text);
        
        let headings: Vec<_> = tokens.iter()
            .filter(|t| t.kind == TokenKind::MarkdownHeading)
            .collect();
        
        assert_eq!(headings.len(), 2);
    }

    #[test]
    fn test_markdown_code() {
        let lexer = MarkdownLexer;
        let text = b"`inline code` and ```block code```";
        let tokens = lexer.tokenize(text);
        
        let code: Vec<_> = tokens.iter()
            .filter(|t| t.kind == TokenKind::MarkdownCode)
            .collect();
        
        assert_eq!(code.len(), 2);
    }

    #[test]
    fn test_markdown_formatting() {
        let lexer = MarkdownLexer;
        let text = b"**bold** and *italic*";
        let tokens = lexer.tokenize(text);
        
        let has_bold = tokens.iter().any(|t| t.kind == TokenKind::MarkdownBold);
        let has_italic = tokens.iter().any(|t| t.kind == TokenKind::MarkdownItalic);
        
        assert!(has_bold);
        assert!(has_italic);
    }
}
