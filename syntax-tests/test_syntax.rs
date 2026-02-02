// Rust Syntax Highlighting Demo

#[derive(Debug, Clone)]
pub struct Example<'a> {
    name: &'a str,
    count: i32,
}

impl<'a> Example<'a> {
    /// Creates a new example
    pub fn new(name: &'a str) -> Self {
        Self { name, count: 0 }
    }
    
    pub async fn process(&mut self) -> Result<(), String> {
        // Line comment
        let x = 42;
        let hex = 0xFF;
        let bin = 0b1010;
        let float = 3.14e-10;
        
        /* Block comment */
        for i in 0..10 {
            self.count += i;
        }
        
        match self.count {
            0 => println!("zero"),
            n if n > 0 => println!("positive: {}", n),
            _ => println!("negative"),
        }
        
        Ok(())
    }
}

fn main() {
    let mut example = Example::new("test");
    println!("Example: {:?}", example);
}
