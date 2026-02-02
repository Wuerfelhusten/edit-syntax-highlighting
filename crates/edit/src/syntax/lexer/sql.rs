// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! High-performance SQL lexer with full language support.

use crate::syntax::lexer::{Lexer, is_whitespace, is_ident_start, is_ident_continue, is_ascii_digit};
use crate::syntax::{Token, TokenKind};

pub struct SqlLexer;

impl Lexer for SqlLexer {
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

                // Line comment (-- or #)
                b'-' if pos + 1 < text.len() && text[pos + 1] == b'-' => {
                    pos += 2;
                    while pos < text.len() && text[pos] != b'\n' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Comment, start..pos));
                }
                
                b'#' => {
                    pos += 1;
                    while pos < text.len() && text[pos] != b'\n' {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Comment, start..pos));
                }

                // Block comment /* ... */
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

                // Single-quoted string
                b'\'' => {
                    pos += 1;
                    while pos < text.len() {
                        if text[pos] == b'\'' {
                            pos += 1;
                            // Handle doubled single quotes (SQL escape)
                            if pos < text.len() && text[pos] == b'\'' {
                                pos += 1;
                            } else {
                                break;
                            }
                        } else {
                            pos += 1;
                        }
                    }
                    tokens.push(Token::new(TokenKind::String, start..pos));
                }

                // Double-quoted identifier (or string in some SQL dialects)
                b'"' => {
                    pos += 1;
                    while pos < text.len() {
                        if text[pos] == b'"' {
                            pos += 1;
                            // Handle doubled double quotes
                            if pos < text.len() && text[pos] == b'"' {
                                pos += 1;
                            } else {
                                break;
                            }
                        } else {
                            pos += 1;
                        }
                    }
                    tokens.push(Token::new(TokenKind::Identifier, start..pos));
                }

                // Backtick-quoted identifier (MySQL)
                b'`' => {
                    pos += 1;
                    while pos < text.len() && text[pos] != b'`' {
                        pos += 1;
                    }
                    if pos < text.len() {
                        pos += 1;
                    }
                    tokens.push(Token::new(TokenKind::Identifier, start..pos));
                }

                // Bracket-quoted identifier (SQL Server) [identifier]
                b'[' => {
                    pos += 1;
                    let mut is_identifier = false;
                    while pos < text.len() && text[pos] != b']' {
                        if is_ident_start(text[pos]) {
                            is_identifier = true;
                        }
                        pos += 1;
                    }
                    if pos < text.len() {
                        pos += 1;
                    }
                    tokens.push(Token::new(
                        if is_identifier { TokenKind::Identifier } else { TokenKind::Operator },
                        start..pos
                    ));
                }

                // Number
                b'0'..=b'9' => {
                    // Hex literal (0x...)
                    if b == b'0' && pos + 1 < text.len() && (text[pos + 1] == b'x' || text[pos + 1] == b'X') {
                        pos += 2;
                        while pos < text.len() && (is_ascii_digit(text[pos]) || matches!(text[pos], b'a'..=b'f' | b'A'..=b'F')) {
                            pos += 1;
                        }
                    }
                    // Decimal
                    else {
                        while pos < text.len() && is_ascii_digit(text[pos]) {
                            pos += 1;
                        }
                        // Float
                        if pos < text.len() && text[pos] == b'.' {
                            pos += 1;
                            while pos < text.len() && is_ascii_digit(text[pos]) {
                                pos += 1;
                            }
                        }
                        // Exponent
                        if pos < text.len() && (text[pos] == b'e' || text[pos] == b'E') {
                            pos += 1;
                            if pos < text.len() && (text[pos] == b'+' || text[pos] == b'-') {
                                pos += 1;
                            }
                            while pos < text.len() && is_ascii_digit(text[pos]) {
                                pos += 1;
                            }
                        }
                    }
                    tokens.push(Token::new(TokenKind::Number, start..pos));
                }

                // Identifier or keyword
                _ if is_ident_start(b) || b == b'_' || b == b'@' => {
                    // Variable (T-SQL @variable or @@system_variable)
                    if b == b'@' {
                        pos += 1;
                        if pos < text.len() && text[pos] == b'@' {
                            pos += 1;
                        }
                    }
                    
                    while pos < text.len() && (is_ident_continue(text[pos]) || text[pos] == b'_') {
                        pos += 1;
                    }
                    
                    let word = &text[start..pos];
                    
                    // Skip if it's a variable
                    if word.starts_with(b"@") {
                        tokens.push(Token::new(TokenKind::VariableName, start..pos));
                        continue;
                    }
                    
                    // Convert to uppercase for comparison (SQL is case-insensitive)
                    let mut upper = Vec::with_capacity(word.len());
                    for &byte in word {
                        upper.push(byte.to_ascii_uppercase());
                    }
                    
                    let kind = match upper.as_slice() {
                        // SQL Keywords - DDL
                        b"CREATE" | b"ALTER" | b"DROP" | b"TRUNCATE" | b"RENAME" |
                        b"TABLE" | b"VIEW" | b"INDEX" | b"DATABASE" | b"SCHEMA" |
                        b"PROCEDURE" | b"FUNCTION" | b"TRIGGER" | b"SEQUENCE" => TokenKind::Keyword,
                        
                        // SQL Keywords - DML
                        b"SELECT" | b"INSERT" | b"UPDATE" | b"DELETE" | b"MERGE" |
                        b"FROM" | b"WHERE" | b"JOIN" | b"INNER" | b"LEFT" | b"RIGHT" | b"FULL" | b"CROSS" |
                        b"ON" | b"USING" | b"GROUP" | b"HAVING" | b"ORDER" | b"BY" |
                        b"LIMIT" | b"OFFSET" | b"FETCH" | b"TOP" |
                        b"UNION" | b"INTERSECT" | b"EXCEPT" | b"MINUS" |
                        b"INTO" | b"VALUES" | b"SET" => TokenKind::Keyword,
                        
                        // SQL Keywords - DCL
                        b"GRANT" | b"REVOKE" | b"DENY" => TokenKind::Keyword,
                        
                        // SQL Keywords - TCL
                        b"COMMIT" | b"ROLLBACK" | b"SAVEPOINT" | b"BEGIN" | b"END" |
                        b"TRANSACTION" | b"START" => TokenKind::Keyword,
                        
                        // SQL Keywords - Constraints
                        b"PRIMARY" | b"FOREIGN" | b"KEY" | b"UNIQUE" | b"CHECK" |
                        b"DEFAULT" | b"NOT" | b"NULL" | b"CONSTRAINT" | b"REFERENCES" => TokenKind::Keyword,
                        
                        // SQL Keywords - Other
                        b"AS" | b"DISTINCT" | b"ALL" | b"ANY" | b"SOME" | b"EXISTS" |
                        b"IN" | b"BETWEEN" | b"LIKE" | b"IS" | b"AND" | b"OR" |
                        b"CASE" | b"WHEN" | b"THEN" | b"ELSE" |
                        b"IF" | b"WHILE" | b"LOOP" | b"REPEAT" | b"GOTO" | b"RETURN" |
                        b"DECLARE" | b"CURSOR" | b"OPEN" | b"CLOSE" |
                        b"WITH" | b"RECURSIVE" | b"OVER" | b"PARTITION" |
                        b"WINDOW" | b"ROWS" | b"RANGE" | b"PRECEDING" | b"FOLLOWING" |
                        b"CURRENT" | b"ROW" | b"UNBOUNDED" => TokenKind::Keyword,
                        
                        // Data types
                        b"INT" | b"INTEGER" | b"BIGINT" | b"SMALLINT" | b"TINYINT" |
                        b"DECIMAL" | b"NUMERIC" | b"FLOAT" | b"REAL" | b"DOUBLE" |
                        b"CHAR" | b"VARCHAR" | b"TEXT" | b"NCHAR" | b"NVARCHAR" | b"NTEXT" |
                        b"DATE" | b"TIME" | b"DATETIME" | b"TIMESTAMP" | b"YEAR" |
                        b"BOOLEAN" | b"BOOL" | b"BIT" |
                        b"BLOB" | b"CLOB" | b"BINARY" | b"VARBINARY" |
                        b"JSON" | b"XML" | b"UUID" | b"SERIAL" | b"AUTO_INCREMENT" => TokenKind::TypeName,
                        
                        // Boolean literals
                        b"TRUE" | b"FALSE" => TokenKind::Boolean,
                        
                        // Aggregate functions
                        b"COUNT" | b"SUM" | b"AVG" | b"MIN" | b"MAX" |
                        b"STDDEV" | b"VARIANCE" | b"GROUP_CONCAT" | b"STRING_AGG" => TokenKind::FunctionName,
                        
                        // String functions
                        b"CONCAT" | b"SUBSTRING" | b"SUBSTR" | b"LENGTH" | b"UPPER" | b"LOWER" |
                        b"TRIM" | b"LTRIM" | b"RTRIM" | b"REPLACE" | b"COALESCE" => TokenKind::FunctionName,
                        
                        // Date functions
                        b"NOW" | b"CURRENT_DATE" | b"CURRENT_TIME" | b"CURRENT_TIMESTAMP" |
                        b"DATEADD" | b"DATEDIFF" | b"EXTRACT" => TokenKind::FunctionName,
                        
                        // Conversion functions
                        b"CAST" | b"CONVERT" | b"TO_CHAR" | b"TO_DATE" | b"TO_NUMBER" => TokenKind::FunctionName,
                        
                        _ => TokenKind::Identifier,
                    };
                    tokens.push(Token::new(kind, start..pos));
                }

                // Operators and punctuation
                b'=' | b'<' | b'>' | b'!' | b'+' | b'-' | b'*' | b'/' | b'%' |
                b'(' | b')' | b',' | b';' | b'.' | b'|' | b'&' | b'^' | b'~' => {
                    pos += 1;
                    // Handle multi-character operators
                    if pos < text.len() {
                        match (b, text[pos]) {
                            (b'=', b'=') | (b'<', b'=') | (b'>', b'=') | (b'!', b'=') |
                            (b'<', b'>') | (b'<', b'<') | (b'>', b'>') |
                            (b'|', b'|') | (b':', b':') => {
                                pos += 1;
                            }
                            _ => {}
                        }
                    }
                    tokens.push(Token::new(TokenKind::Operator, start..pos));
                }

                // Unknown character
                _ => {
                    pos += 1;
                    tokens.push(Token::new(TokenKind::Error, start..pos));
                }
            }
        }

        tokens
    }
}
