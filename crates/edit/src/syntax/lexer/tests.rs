use crate::syntax::lexer::{Language, LexerRegistry};

#[test]
fn test_c_lexer_basic() {
    let source = b"int main() { return 0; }";
    let lexer = LexerRegistry::get_lexer(Language::C);
    let tokens = lexer.tokenize(source);
    
    println!("Tokens: {}", tokens.len());
    for token in &tokens {
        println!("{:?}", token);
    }
    
    assert!(!tokens.is_empty(), "C lexer should produce tokens");
}

#[test]
fn test_java_lexer_basic() {
    let source = b"public class Test {}";
    let lexer = LexerRegistry::get_lexer(Language::Java);
    let tokens = lexer.tokenize(source);
    
    println!("Tokens: {}", tokens.len());
    assert!(!tokens.is_empty(), "Java lexer should produce tokens");
}

#[test]
fn test_csharp_lexer_basic() {
    let source = b"public class Test {}";
    let lexer = LexerRegistry::get_lexer(Language::CSharp);
    let tokens = lexer.tokenize(source);
    
    println!("Tokens: {}", tokens.len());
    assert!(!tokens.is_empty(), "C# lexer should produce tokens");
}

#[test]
fn test_cpp_lexer_basic() {
    let source = b"int main() { return 0; }";
    let lexer = LexerRegistry::get_lexer(Language::Cpp);
    let tokens = lexer.tokenize(source);
    
    println!("Tokens: {}", tokens.len());
    assert!(!tokens.is_empty(), "C++ lexer should produce tokens");
}
