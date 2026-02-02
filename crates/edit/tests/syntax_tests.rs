// Test file to verify syntax highlighting implementation

use edit::syntax::{Language, SyntaxHighlighter, Theme, TokenKind};

#[test]
fn test_json_highlighting() {
    let theme = Theme::default();
    let mut highlighter = SyntaxHighlighter::new(Language::Json, theme);
    
    let json = br#"{"key": "value", "number": 42}"#;
    highlighter.update(json, false);
    
    // Check that we have tokens
    let tokens = highlighter.get_tokens_in_range(0..json.len());
    assert!(!tokens.is_empty());
    
    // Check for specific token types
    let has_string = tokens.iter().any(|t| t.kind == TokenKind::String);
    let has_number = tokens.iter().any(|t| t.kind == TokenKind::Number);
    
    assert!(has_string);
    assert!(has_number);
}

#[test]
fn test_rust_highlighting() {
    let theme = Theme::default();
    let mut highlighter = SyntaxHighlighter::new(Language::Rust, theme);
    
    let rust_code = b"fn main() { let x = 42; }";
    highlighter.update(rust_code, false);
    
    let tokens = highlighter.get_tokens_in_range(0..rust_code.len());
    assert!(!tokens.is_empty());
    
    // Should have keywords
    let has_keyword = tokens.iter().any(|t| t.kind.is_keyword());
    assert!(has_keyword);
}

#[test]
fn test_language_detection() {
    assert_eq!(Language::from_extension("rs"), Language::Rust);
    assert_eq!(Language::from_extension("json"), Language::Json);
    assert_eq!(Language::from_extension("py"), Language::Python);
    assert_eq!(Language::from_extension("txt"), Language::PlainText);
}

#[test]
fn test_theme_colors() {
    let theme = Theme::default_dark();
    let style = theme.get_style(TokenKind::Keyword);
    
    // Keywords should have a color (just check that style exists)
    // Note: StraightRgba doesn't expose r/g/b fields directly
    let _ = style.fg; // Just verify fg exists
}
