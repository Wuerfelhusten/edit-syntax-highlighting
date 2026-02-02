// Integration tests for newly implemented languages
// This verifies that C, C++, C#, Java, Go, Shell, PowerShell, HTML, CSS, SQL, and XML
// all work correctly with the SyntaxHighlighter

use edit::syntax::{Language, SyntaxHighlighter, Theme, TokenKind};

#[test]
fn test_c_highlighting() {
    let theme = Theme::default();
    let mut highlighter = SyntaxHighlighter::new(Language::C, theme);
    
    let code = b"#include <stdio.h>\nint main() { printf(\"Hello\"); return 0; }";
    highlighter.update(code, false);
    
    let tokens = highlighter.get_tokens_in_range(0..code.len());
    println!("C tokens count: {}", tokens.len());
    for token in tokens.iter() {
        println!("  Token: {:?} at {:?}", token.kind, token.span);
    }
    
    assert!(!tokens.is_empty(), "C lexer should produce tokens");
    let has_keyword = tokens.iter().any(|t| matches!(t.kind, TokenKind::Keyword));
    assert!(has_keyword, "C code should have keywords (int, return)");
}

#[test]
fn test_cpp_highlighting() {
    let theme = Theme::default();
    let mut highlighter = SyntaxHighlighter::new(Language::Cpp, theme);
    
    let code = b"class Foo { public: void bar() { std::cout << \"test\"; } };";
    highlighter.update(code, false);
    
    let tokens = highlighter.get_tokens_in_range(0..code.len());
    println!("C++ tokens count: {}", tokens.len());
    for token in tokens.iter() {
        println!("  Token: {:?} at {:?}", token.kind, token.span);
    }
    
    assert!(!tokens.is_empty(), "C++ lexer should produce tokens");
    let has_keyword = tokens.iter().any(|t| matches!(t.kind, TokenKind::Keyword));
    assert!(has_keyword, "C++ code should have keywords (class, public, void)");
}

#[test]
fn test_csharp_highlighting() {
    let theme = Theme::default();
    let mut highlighter = SyntaxHighlighter::new(Language::CSharp, theme);
    
    let code = b"class Program { public static void Main() { Console.WriteLine(\"test\"); } }";
    highlighter.update(code, false);
    
    let tokens = highlighter.get_tokens_in_range(0..code.len());
    println!("C# tokens count: {}", tokens.len());
    for token in tokens.iter() {
        println!("  Token: {:?} at {:?}", token.kind, token.span);
    }
    
    assert!(!tokens.is_empty(), "C# lexer should produce tokens");
    let has_keyword = tokens.iter().any(|t| matches!(t.kind, TokenKind::Keyword));
    assert!(has_keyword, "C# code should have keywords (class, public, static, void)");
}

#[test]
fn test_java_highlighting() {
    let theme = Theme::default();
    let mut highlighter = SyntaxHighlighter::new(Language::Java, theme);
    
    let code = b"public class Main { public static void main(String[] args) { System.out.println(\"test\"); } }";
    highlighter.update(code, false);
    
    let tokens = highlighter.get_tokens_in_range(0..code.len());
    println!("Java tokens count: {}", tokens.len());
    for token in tokens.iter() {
        println!("  Token: {:?} at {:?}", token.kind, token.span);
    }
    
    assert!(!tokens.is_empty(), "Java lexer should produce tokens");
    let has_keyword = tokens.iter().any(|t| matches!(t.kind, TokenKind::Keyword));
    assert!(has_keyword, "Java code should have keywords (public, class, static, void)");
}

#[test]
fn test_go_highlighting() {
    let theme = Theme::default();
    let mut highlighter = SyntaxHighlighter::new(Language::Go, theme);
    
    let code = b"package main\nfunc main() { fmt.Println(\"test\") }";
    highlighter.update(code, false);
    
    let tokens = highlighter.get_tokens_in_range(0..code.len());
    assert!(!tokens.is_empty(), "Go lexer should produce tokens");
    let has_keyword = tokens.iter().any(|t| matches!(t.kind, TokenKind::Keyword));
    assert!(has_keyword, "Go code should have keywords (package, func)");
}

#[test]
fn test_shell_highlighting() {
    let theme = Theme::default();
    let mut highlighter = SyntaxHighlighter::new(Language::Shell, theme);
    
    let code = b"#!/bin/bash\nfor i in 1 2 3; do echo $i; done";
    highlighter.update(code, false);
    
    let tokens = highlighter.get_tokens_in_range(0..code.len());
    assert!(!tokens.is_empty(), "Shell lexer should produce tokens");
}

#[test]
fn test_powershell_highlighting() {
    let theme = Theme::default();
    let mut highlighter = SyntaxHighlighter::new(Language::Shell, theme);
    
    let code = b"foreach ($item in $list) { Write-Host $item }";
    highlighter.update(code, false);
    
    let tokens = highlighter.get_tokens_in_range(0..code.len());
    assert!(!tokens.is_empty(), "PowerShell lexer should produce tokens");
}

#[test]
fn test_html_highlighting() {
    let theme = Theme::default();
    let mut highlighter = SyntaxHighlighter::new(Language::Html, theme);
    
    let code = b"<html><body><h1>Test</h1></body></html>";
    highlighter.update(code, false);
    
    let tokens = highlighter.get_tokens_in_range(0..code.len());
    assert!(!tokens.is_empty(), "HTML lexer should produce tokens");
}

#[test]
fn test_css_highlighting() {
    let theme = Theme::default();
    let mut highlighter = SyntaxHighlighter::new(Language::Css, theme);
    
    let code = b".class { color: red; font-size: 14px; }";
    highlighter.update(code, false);
    
    let tokens = highlighter.get_tokens_in_range(0..code.len());
    assert!(!tokens.is_empty(), "CSS lexer should produce tokens");
}

#[test]
fn test_sql_highlighting() {
    let theme = Theme::default();
    let mut highlighter = SyntaxHighlighter::new(Language::Sql, theme);
    
    let code = b"SELECT * FROM users WHERE age > 18;";
    highlighter.update(code, false);
    
    let tokens = highlighter.get_tokens_in_range(0..code.len());
    assert!(!tokens.is_empty(), "SQL lexer should produce tokens");
}

#[test]
fn test_xml_highlighting() {
    let theme = Theme::default();
    let mut highlighter = SyntaxHighlighter::new(Language::Xml, theme);
    
    let code = b"<?xml version=\"1.0\"?><root><item>Test</item></root>";
    highlighter.update(code, false);
    
    let tokens = highlighter.get_tokens_in_range(0..code.len());
    assert!(!tokens.is_empty(), "XML lexer should produce tokens");
}

#[test]
fn test_language_extension_mapping() {
    // Test all new language extensions
    assert_eq!(Language::from_extension("c"), Language::C);
    assert_eq!(Language::from_extension("h"), Language::C);
    assert_eq!(Language::from_extension("cpp"), Language::Cpp);
    assert_eq!(Language::from_extension("hpp"), Language::Cpp);
    assert_eq!(Language::from_extension("cs"), Language::CSharp);
    assert_eq!(Language::from_extension("java"), Language::Java);
    assert_eq!(Language::from_extension("go"), Language::Go);
    assert_eq!(Language::from_extension("sh"), Language::Shell);
    assert_eq!(Language::from_extension("bash"), Language::Shell);
    assert_eq!(Language::from_extension("html"), Language::Html);
    assert_eq!(Language::from_extension("css"), Language::Css);
    assert_eq!(Language::from_extension("sql"), Language::Sql);
    assert_eq!(Language::from_extension("xml"), Language::Xml);
}
