use crate::Tokenizer;
use crate::ASTParser;
use crate::Runtime;
use std;
use std::io::Read;
use std::io::{self, Write};

pub fn start_repl() {
    let tokenizer = Tokenizer::new();
    let mut parser = ASTParser::new();
    let mut runtime = Runtime::new();
    
    let stdin = std::io::stdin();
    loop {
        let input = read_multi_line_input(&stdin);
        
        if input.trim().is_empty() {
            continue;
        }
        
        if input.trim() == "exit" || input.trim() == "quit" {
            break;
        }
        
        let tokens = tokenizer.process_content(&input);
        let statements = parser.parse(tokens);
        runtime.execute_statements(statements);
    }
}

fn read_multi_line_input(stdin: &std::io::Stdin) -> String {
    let mut input = String::new();
    let mut line_buffer = String::new();
    let mut brace_count = 0;
    let mut consecutive_empty_lines = 0;
    
    print!(">> ");
    std::io::stdout().flush().unwrap();
    
    loop {
        line_buffer.clear();
        stdin.read_line(&mut line_buffer).unwrap();
        
        let line = line_buffer.trim_end();
        let trimmed_line = line.trim();
        
        
        for ch in line.chars() {
            match ch {
                '{' => brace_count += 1,
                '}' => brace_count -= 1,
                _ => {}
            }
        }
        
        input.push_str(line);
        input.push('\n');

        if trimmed_line.is_empty() {
            consecutive_empty_lines += 1;
            if brace_count == 0 && consecutive_empty_lines >= 2 {
                break;
            }
            print!(".. ");
            std::io::stdout().flush().unwrap();
            continue;
        } else {
            consecutive_empty_lines = 0;
        }

        if brace_count == 0 {
            let full_input = input.trim();

            if full_input.starts_with("if") && !full_input.contains("else") {
                print!(".. ");
                std::io::stdout().flush().unwrap();
                continue;
            }
            break;
        } else if brace_count > 0 {
            
            print!(".. ");
            std::io::stdout().flush().unwrap();
        } else {
            break;
        }
    }
    
    input
}

