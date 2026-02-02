// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Lexer implementations for various programming languages.

mod json;
mod rust;
mod python;
mod markdown;
mod javascript;
mod toml;
mod yaml;
mod c;
mod cpp;
mod csharp;
mod go;
mod html;
mod css;
mod java;
mod xml;
mod shell;
mod sql;
mod asciidoc;

use crate::syntax::{Token, TokenKind};

/// Supported programming languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    PlainText,
    Json,
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Markdown,
    Toml,
    Yaml,
    C,
    Cpp,
    CSharp,
    Go,
    Html,
    Css,
    Java,
    Xml,
    Shell,
    Sql,
    AsciiDoc,
}

impl Language {
    /// Try to detect the language from a file extension.
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "json" | "jsonc" => Language::Json,
            "rs" => Language::Rust,
            "py" | "pyw" | "pyi" => Language::Python,
            "js" | "mjs" | "cjs" => Language::JavaScript,
            "ts" | "mts" | "cts" => Language::TypeScript,
            "md" | "markdown" => Language::Markdown,
            "toml" => Language::Toml,
            "yaml" | "yml" => Language::Yaml,
            "c" | "h" => Language::C,
            "cpp" | "cc" | "cxx" | "hpp" | "hxx" => Language::Cpp,
            "cs" => Language::CSharp,
            "go" => Language::Go,
            "html" | "htm" => Language::Html,
            "css" => Language::Css,
            "java" => Language::Java,
            "xml" | "svg" | "xhtml" | "xsd" | "wsdl" => Language::Xml,
            "sh" | "bash" | "zsh" => Language::Shell,
            "sql" => Language::Sql,
            "adoc" | "asciidoc" | "asc" => Language::AsciiDoc,
            _ => Language::PlainText,
        }
    }

    /// Get the display name for the language.
    pub fn name(self) -> &'static str {
        match self {
            Language::PlainText => "Plain Text",
            Language::Json => "JSON",
            Language::Rust => "Rust",
            Language::Python => "Python",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Markdown => "Markdown",
            Language::Toml => "TOML",
            Language::Yaml => "YAML",
            Language::C => "C",
            Language::Cpp => "C++",
            Language::CSharp => "C#",
            Language::Go => "Go",
            Language::Html => "HTML",
            Language::Css => "CSS",
            Language::Java => "Java",
            Language::Xml => "XML",
            Language::Shell => "Shell",
            Language::Sql => "SQL",
            Language::AsciiDoc => "AsciiDoc",
        }
    }
}

/// A trait for language lexers.
pub trait Lexer: Send + Sync {
    /// Tokenize the given text into a sequence of tokens.
    fn tokenize(&self, text: &[u8]) -> Vec<Token>;
}

/// Registry for language lexers.
pub struct LexerRegistry;

impl LexerRegistry {
    /// Get a lexer for the given language.
    pub fn get_lexer(language: Language) -> Box<dyn Lexer> {
        match language {
            Language::Json => Box::new(json::JsonLexer),
            Language::Rust => Box::new(rust::RustLexer),
            Language::Python => Box::new(python::PythonLexer),
            Language::Markdown => Box::new(markdown::MarkdownLexer),
            Language::JavaScript => Box::new(javascript::JavaScriptLexer),
            Language::TypeScript => Box::new(javascript::JavaScriptLexer), // Use same lexer
            Language::Toml => Box::new(toml::TomlLexer),
            Language::Yaml => Box::new(yaml::YamlLexer),
            Language::C => Box::new(c::CLexer),
            Language::Cpp => Box::new(cpp::CppLexer),
            Language::CSharp => Box::new(csharp::CSharpLexer),
            Language::Go => Box::new(go::GoLexer),
            Language::Html => Box::new(html::HtmlLexer),
            Language::Css => Box::new(css::CssLexer),
            Language::Java => Box::new(java::JavaLexer),
            Language::Xml => Box::new(xml::XmlLexer),
            Language::Shell => Box::new(shell::ShellLexer),
            Language::Sql => Box::new(sql::SqlLexer),
            Language::AsciiDoc => Box::new(asciidoc::AsciiDocLexer),
            Language::PlainText => Box::new(PlainTextLexer),
        }
    }
}

/// A simple plain text lexer that doesn't do any highlighting.
struct PlainTextLexer;

impl Lexer for PlainTextLexer {
    fn tokenize(&self, text: &[u8]) -> Vec<Token> {
        if text.is_empty() {
            return Vec::new();
        }
        vec![Token::new(TokenKind::Identifier, 0..text.len())]
    }
}

/// Helper function to check if a byte is a whitespace character.
#[inline]
pub(crate) fn is_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r')
}

/// Helper function to check if a byte is an ASCII alphabetic character.
#[inline]
pub(crate) fn is_ascii_alpha(b: u8) -> bool {
    b.is_ascii_alphabetic()
}

/// Helper function to check if a byte is an ASCII digit.
#[inline]
pub(crate) fn is_ascii_digit(b: u8) -> bool {
    b.is_ascii_digit()
}

/// Helper function to check if a byte is an ASCII alphanumeric character.
#[inline]
pub(crate) fn is_ascii_alphanumeric(b: u8) -> bool {
    b.is_ascii_alphanumeric()
}

/// Helper function to check if a byte can start an identifier.
#[inline]
pub(crate) fn is_ident_start(b: u8) -> bool {
    is_ascii_alpha(b) || b == b'_'
}

/// Helper function to check if a byte can continue an identifier.
#[inline]
pub(crate) fn is_ident_continue(b: u8) -> bool {
    is_ascii_alphanumeric(b) || b == b'_'
}
