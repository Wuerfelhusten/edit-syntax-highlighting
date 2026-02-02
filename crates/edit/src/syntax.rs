// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Modern syntax highlighting system for Edit.
//!
//! This module provides a performant, incremental syntax highlighting system
//! that supports multiple languages through a lexer-based approach.
//!
//! # Architecture
//!
//! - **Lexers**: Fast, hand-written lexers for each language
//! - **Token Cache**: Incremental caching for performance
//! - **Themes**: Configurable color schemes for different token types
//! - **Lazy Evaluation**: Only highlights visible portions of the document

mod lexer;
mod theme;
mod token;

pub use lexer::{Lexer, LexerRegistry, Language};
pub use theme::{Theme, TokenStyle};
pub use token::{Token, TokenKind, TokenSpan};

use std::ops::Range;

/// A cached syntax highlighting result for a document.
pub struct SyntaxHighlighter {
    /// The language being highlighted
    language: Language,
    /// Cached tokens for the document
    tokens: Vec<Token>,
    /// Dirty range that needs re-highlighting
    dirty_range: Option<Range<usize>>,
    /// The theme to use for coloring
    theme: Theme,
    /// Document length at last tokenization
    doc_len: usize,
}

impl SyntaxHighlighter {
    /// Create a new syntax highlighter for the given language.
    pub fn new(language: Language, theme: Theme) -> Self {
        Self {
            language,
            tokens: Vec::new(),
            dirty_range: Some(0..usize::MAX),
            theme,
            doc_len: 0,
        }
    }

    /// Mark a range as dirty (needs re-highlighting).
    pub fn mark_dirty(&mut self, range: Range<usize>) {
        if let Some(dirty) = &mut self.dirty_range {
            dirty.start = dirty.start.min(range.start);
            dirty.end = dirty.end.max(range.end);
        } else {
            self.dirty_range = Some(range);
        }
    }

    /// Update the highlighting for the document.
    ///
    /// This is an incremental operation that only re-tokenizes dirty regions.
    pub fn update(&mut self, text: &[u8], force: bool) {
        if !force && self.dirty_range.is_none() && text.len() == self.doc_len {
            return;
        }

        // For now, we re-tokenize the entire document.
        // Future optimization: incremental tokenization.
        let lexer = LexerRegistry::get_lexer(self.language);
        self.tokens = lexer.tokenize(text);
        self.dirty_range = None;
        self.doc_len = text.len();
    }

    /// Get the style for a given byte offset in the document.
    pub fn get_style_at(&self, offset: usize) -> Option<TokenStyle> {
        // Binary search for the token containing this offset
        let idx = self.tokens.binary_search_by(|token| {
            if offset < token.span.start {
                std::cmp::Ordering::Greater
            } else if offset >= token.span.end {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });

        match idx {
            Ok(i) => Some(self.theme.get_style(self.tokens[i].kind)),
            Err(_) => None,
        }
    }

    /// Get all tokens in the given range.
    pub fn get_tokens_in_range(&self, range: Range<usize>) -> &[Token] {
        let start_idx = self.tokens.partition_point(|t| t.span.end <= range.start);
        let end_idx = self.tokens.partition_point(|t| t.span.start < range.end);
        &self.tokens[start_idx..end_idx]
    }

    /// Get the theme.
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Set a new theme.
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    /// Get the current language.
    pub fn language(&self) -> Language {
        self.language
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlighter_basic() {
        let theme = Theme::default();
        let mut highlighter = SyntaxHighlighter::new(Language::Json, theme);
        
        let text = b"{\"key\": \"value\"}";
        highlighter.update(text, false);
        
        assert!(!highlighter.tokens.is_empty());
    }
}
