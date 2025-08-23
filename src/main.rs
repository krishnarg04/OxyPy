use std::fs;
use std::io;

mod tokenizer;
mod AstTree;
mod Environment;
mod runtime;
mod Functions;

use tokenizer::Tokenizer;
use AstTree::ASTParser;
use runtime::Runtime;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 && args[1] == "--test" {
        return Ok(());
    }
    
    if args.len() < 2 {
        eprintln!("Usage: {} <filename> or {} --test", args[0], args[0]);
        return Ok(());
    }
    
    let file_name = &args[1];

    let tokenizer = Tokenizer::new();
    let mut parser = ASTParser::new();
    let mut runtime = Runtime::new();
    
    let file_content = fs::read_to_string(file_name)?;
    
    let cleaned_content = remove_comments(&file_content);
    
    if cleaned_content.trim().is_empty() {
        return Ok(());
    }
    
    let tokens = tokenizer.process_content(&cleaned_content);
    
    let statements = parser.parse(tokens);
    runtime.execute_statements(statements);

    Ok(())
}

fn remove_comments(content: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut string_char = '"';
    let mut escaped = false;
    let mut chars = content.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if escaped {
            result.push(ch);
            escaped = false;
            continue;
        }
        
        if in_string {
            result.push(ch);
            if ch == '\\' {
                escaped = true;
            } else if ch == string_char {
                in_string = false;
            }
            continue;
        }
        
        match ch {
            '"' | '\'' => {
                in_string = true;
                string_char = ch;
                result.push(ch);
            }
            '/' => {
                if let Some(&'/') = chars.peek() {
                    chars.next(); 
                    
                    while let Some(next_ch) = chars.next() {
                        if next_ch == '\n' {
                            result.push('\n'); 
                            break;
                        }
                    }
                } else {
                    result.push(ch);
                }
            }
            _ => {
                result.push(ch);
            }
        }
    }
    
    result
}



