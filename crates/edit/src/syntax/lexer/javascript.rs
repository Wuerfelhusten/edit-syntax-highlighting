// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! JavaScript/TypeScript lexer with modern syntax support.

use crate::syntax::lexer::{Lexer, is_ident_start, is_ident_continue, is_ascii_digit};
use crate::syntax::{Token, TokenKind};

pub struct JavaScriptLexer;

impl Lexer for JavaScriptLexer {
    fn tokenize(&self, text: &[u8]) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(text.len() / 8);
        let mut pos = 0;

        while pos < text.len() {
            let start = pos;
            let b = text[pos];

            match b {
                // Whitespace
                b' ' | b'\t' | b'\n' | b'\r' => {
                    while pos < text.len() && matches!(text[pos], b' ' | b'\t' | b'\n' | b'\r') {
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

                // Template literals
                b'`' => {
                    pos += 1;
                    while pos < text.len() {
                        if text[pos] == b'\\' {
                            pos += 2;
                        } else if text[pos] == b'`' {
                            pos += 1;
                            break;
                        } else {
                            pos += 1;
                        }
                    }
                    tokens.push(Token::new(TokenKind::String, start..pos));
                }

                // String literals
                b'"' | b'\'' => {
                    let quote = b;
                    pos += 1;
                    let mut escaped = false;
                    while pos < text.len() {
                        if escaped {
                            escaped = false;
                        } else if text[pos] == b'\\' {
                            escaped = true;
                        } else if text[pos] == quote {
                            pos += 1;
                            break;
                        }
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::String, start..pos));
                }

                // Numbers
                b'0'..=b'9' => {
                    pos += 1;
                    
                    // Hex
                    if start + 1 < text.len() && text[start] == b'0' && matches!(text[start + 1], b'x' | b'X') {
                        pos += 1;
                        while pos < text.len() && (is_ascii_digit(text[pos]) || matches!(text[pos], b'a'..=b'f' | b'A'..=b'F')) {
                            pos += 1;
                        }
                    }
                    // Binary
                    else if start + 1 < text.len() && text[start] == b'0' && matches!(text[start + 1], b'b' | b'B') {
                        pos += 1;
                        while pos < text.len() && matches!(text[pos], b'0' | b'1') {
                            pos += 1;
                        }
                    }
                    // Octal
                    else if start + 1 < text.len() && text[start] == b'0' && matches!(text[start + 1], b'o' | b'O') {
                        pos += 1;
                        while pos < text.len() && matches!(text[pos], b'0'..=b'7') {
                            pos += 1;
                        }
                    }
                    // Decimal/Float
                    else {
                        while pos < text.len() && is_ascii_digit(text[pos]) {
                            pos += 1;
                        }
                        
                        // Float
                        if pos < text.len() && text[pos] == b'.' && pos + 1 < text.len() && is_ascii_digit(text[pos + 1]) {
                            pos += 1;
                            while pos < text.len() && is_ascii_digit(text[pos]) {
                                pos += 1;
                            }
                        }
                        
                        // Exponent
                        if pos < text.len() && matches!(text[pos], b'e' | b'E') {
                            pos += 1;
                            if pos < text.len() && matches!(text[pos], b'+' | b'-') {
                                pos += 1;
                            }
                            while pos < text.len() && is_ascii_digit(text[pos]) {
                                pos += 1;
                            }
                        }
                    }
                    
                    tokens.push(Token::new(TokenKind::Number, start..pos));
                }

                // Identifiers and keywords
                _ if is_ident_start(b) || b == b'$' => {
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'$') {
                        pos += 1;
                    }
                    
                    let word = &text[start..pos];
                    let kind = match word {
                        b"in" | b"of" | b"instanceof" | b"typeof" | b"delete" | b"void" => TokenKind::KeywordOperator,
                        b"if" | b"else" | b"switch" | b"case" | b"default" | b"for" | b"while" | b"do" | b"break" | b"continue" | b"return" | b"throw" | b"try" | b"catch" | b"finally" => TokenKind::KeywordControl,
                        b"function" | b"async" | b"await" | b"yield" => TokenKind::KeywordFunction,
                        b"import" | b"export" | b"from" | b"as" => TokenKind::KeywordImport,
                        b"let" | b"const" | b"var" => TokenKind::KeywordStorage,
                        b"class" | b"interface" | b"extends" | b"implements" | b"enum" | b"type" => TokenKind::KeywordType,
                        b"new" | b"this" | b"super" | b"static" | b"public" | b"private" | b"protected" | b"readonly" => TokenKind::Keyword,
                        b"true" | b"false" => TokenKind::Boolean,
                        b"null" | b"undefined" => TokenKind::Null,
                        _ => TokenKind::Identifier,
                    };
                    
                    tokens.push(Token::new(kind, start..pos));
                }

                // Operators
                b'+' | b'-' | b'*' | b'/' | b'%' | b'&' | b'|' | b'^' | b'!' | b'=' | b'<' | b'>' | b'?' | b':' | b'~' => {
                    pos += 1;
                    // Handle multi-character operators (==, ===, <=, >=, etc.)
                    while pos < text.len() && matches!(text[pos], b'=' | b'&' | b'|' | b'<' | b'>') {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
                }

                // Delimiters
                b'{' | b'}' | b'[' | b']' | b'(' | b')' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Delimiter, start..pos));
                }

                // Punctuation
                b',' | b';' | b'.' => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Punctuation, start..pos));
                }

                // Unknown
                _ => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Error, start..pos));
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
    fn test_js_keywords() {
        let lexer = JavaScriptLexer;
        let text = b"const x = async () => { return await fetch(); }";
        let tokens = lexer.tokenize(text);
        
        let has_const = tokens.iter().any(|t| t.kind == TokenKind::KeywordStorage);
        let has_async = tokens.iter().any(|t| t.kind == TokenKind::KeywordFunction);
        
        assert!(has_const);
        assert!(has_async);
    }

    #[test]
    fn test_js_template_literal() {
        let lexer = JavaScriptLexer;
        let text = b"`Hello ${name}`";
        let tokens = lexer.tokenize(text);
        
        let has_string = tokens.iter().any(|t| t.kind == TokenKind::String);
        assert!(has_string);
    }
}
