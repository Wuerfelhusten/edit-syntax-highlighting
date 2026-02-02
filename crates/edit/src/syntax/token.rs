// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Token types and structures for syntax highlighting.

use std::ops::Range;

/// A single token from the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The kind of token
    pub kind: TokenKind,
    /// The byte span in the source text
    pub span: TokenSpan,
}

/// A byte range in the source text.
pub type TokenSpan = Range<usize>;

/// The kind of token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Generic
    Whitespace,
    Comment,
    Error,

    // Literals
    String,
    Number,
    Boolean,
    Null,
    Char,

    // Keywords
    Keyword,
    KeywordControl,   // if, else, while, for, loop, match
    KeywordFunction,  // fn, function, def
    KeywordImport,    // import, use, require
    KeywordStorage,   // let, const, var, mut
    KeywordType,      // type, struct, enum, class, interface
    KeywordOperator,  // as, in, is, sizeof

    // Identifiers
    Identifier,
    TypeName,
    FunctionName,
    VariableName,
    PropertyName,
    ParameterName,

    // Operators and Punctuation
    Operator,
    Punctuation,
    Delimiter,       // {}, [], ()
    Separator,       // ,, ;

    // Special
    Attribute,       // #[derive(...)] in Rust
    Macro,           // macros
    Label,           // loop labels
    Escape,          // escape sequences in strings

    // JSON specific
    JsonKey,
    JsonBrace,
    JsonBracket,
    JsonColon,
    JsonComma,

    // Rust specific
    RustLifetime,
    RustMacro,
    RustAttribute,

    // Markdown specific
    MarkdownHeading,
    MarkdownBold,
    MarkdownItalic,
    MarkdownCode,
    MarkdownLink,
}

impl TokenKind {
    /// Returns true if this token is a whitespace or comment.
    pub fn is_trivia(self) -> bool {
        matches!(self, TokenKind::Whitespace | TokenKind::Comment)
    }

    /// Returns true if this token represents an error.
    pub fn is_error(self) -> bool {
        matches!(self, TokenKind::Error)
    }

    /// Returns true if this token is a keyword.
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            TokenKind::Keyword
                | TokenKind::KeywordControl
                | TokenKind::KeywordFunction
                | TokenKind::KeywordImport
                | TokenKind::KeywordStorage
                | TokenKind::KeywordType
                | TokenKind::KeywordOperator
        )
    }

    /// Returns true if this token is a literal.
    pub fn is_literal(self) -> bool {
        matches!(
            self,
            TokenKind::String
                | TokenKind::Number
                | TokenKind::Boolean
                | TokenKind::Null
                | TokenKind::Char
        )
    }
}

impl Token {
    /// Create a new token.
    pub fn new(kind: TokenKind, span: TokenSpan) -> Self {
        Self { kind, span }
    }

    /// Get the length of the token in bytes.
    pub fn len(&self) -> usize {
        self.span.end - self.span.start
    }

    /// Check if the token is empty.
    pub fn is_empty(&self) -> bool {
        self.span.start == self.span.end
    }
}
