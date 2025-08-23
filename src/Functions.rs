use std::{collections::HashMap, sync::{Mutex, OnceLock}};
use crate::tokenizer::{Types, DataHolder};

type BuiltInFn = fn(Vec<DataHolder>) -> Option<DataHolder>;

pub struct BuiltInFunction {
    function_map: HashMap<String, BuiltInFn>,
}

impl BuiltInFunction {
    pub fn new() -> Self {
        let mut function_map = HashMap::new();
        function_map.insert("print".to_string(), print_fn as BuiltInFn);
        function_map.insert("println".to_string(), println_fn as BuiltInFn);
        function_map.insert("len".to_string(), len_fn as BuiltInFn);
        function_map.insert("current_time".to_string(), current_time_fn as BuiltInFn);
        function_map.insert("to_string".to_string(), to_string_fn as BuiltInFn);
        function_map.insert("parse_int".to_string(), parse_int_fn as BuiltInFn);

        BuiltInFunction { function_map }
    }

    pub fn call(&self, name: &str, args: Vec<DataHolder>) -> Option<DataHolder> {
        if let Some(func) = self.function_map.get(name) {
            func(args)
        } else {
            None
        }
    }

    pub fn has_function(&self, name: &str) -> bool {
        self.function_map.contains_key(name)
    }

    pub fn get_function_names(&self) -> Vec<String> {
        self.function_map.keys().cloned().collect()
    }
}

static BUILT_IN_FUNCTIONS: OnceLock<Mutex<BuiltInFunction>> = OnceLock::new();

pub fn get_built_in_functions() -> &'static Mutex<BuiltInFunction> {
    BUILT_IN_FUNCTIONS.get_or_init(|| Mutex::new(BuiltInFunction::new()))
}

fn print_fn(args: Vec<DataHolder>) -> Option<DataHolder> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 { print!(" "); }
        match arg {
            DataHolder::INTEGER32(n) => print!("{}", n),
            DataHolder::INTEGER64(n) => print!("{}", n),
            DataHolder::FLOAT32(n) => print!("{}", n),
            DataHolder::FLOAT64(n) => print!("{}", n),
            DataHolder::STRING(s) => print!("{}", s),
            DataHolder::BOOLEAN(b) => print!("{}", b),
            DataHolder::LIST(list) => {
                print!("[");
                for (j, item) in list.iter().enumerate() {
                    if j > 0 { print!(", "); }
                    match item {
                        DataHolder::STRING(s) => print!("\"{}\"", s),
                        other => match other {
                            DataHolder::INTEGER32(n) => print!("{}", n),
                            DataHolder::INTEGER64(n) => print!("{}", n),
                            DataHolder::FLOAT32(n) => print!("{}", n),
                            DataHolder::FLOAT64(n) => print!("{}", n),
                            DataHolder::BOOLEAN(b) => print!("{}", b),
                            _ => print!("{:?}", other),
                        }
                    }
                }
                print!("]");
            },
            _ => print!("{:?}", arg),
        }
    }
    Some(DataHolder::INTEGER32(0)) 
}

fn println_fn(args: Vec<DataHolder>) -> Option<DataHolder> {
    let result = print_fn(args);
    println!(); 
    result
}

fn len_fn(args: Vec<DataHolder>) -> Option<DataHolder> {
    if args.len() != 1 {
        eprintln!("Error: len() expects exactly 1 argument, got {}", args.len());
        return None;
    }
    match &args[0] {
        DataHolder::STRING(s) => Some(DataHolder::INTEGER32(s.len() as i32)),
        DataHolder::LIST(list) => Some(DataHolder::INTEGER32(list.len() as i32)),
        _ => {
            eprintln!("Error: len() can only be called on strings or lists");
            None
        }
    }
}

fn current_time_fn(_args: Vec<DataHolder>) -> Option<DataHolder> {
    
    use std::time::{SystemTime, UNIX_EPOCH};
    
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let timestamp = duration.as_secs();
            Some(DataHolder::INTEGER64(timestamp as i64))
        },
        Err(_) => {
            eprintln!("Error: Failed to get current time");
            None
        }
    }
}

fn to_string_fn(args: Vec<DataHolder>) -> Option<DataHolder> {
    if args.len() != 1 {
        eprintln!("Error: to_string() expects exactly 1 argument, got {}", args.len());
        return None;
    }
    
    match &args[0] {
        DataHolder::INTEGER32(n) => Some(DataHolder::STRING(n.to_string())),
        DataHolder::INTEGER64(n) => Some(DataHolder::STRING(n.to_string())),
        DataHolder::FLOAT32(n) => Some(DataHolder::STRING(n.to_string())),
        DataHolder::FLOAT64(n) => Some(DataHolder::STRING(n.to_string())),
        DataHolder::BOOLEAN(b) => Some(DataHolder::STRING(b.to_string())),
        DataHolder::STRING(s) => Some(DataHolder::STRING(s.clone())), 
        DataHolder::LIST(_) => {
            eprintln!("Error: Cannot convert list to string directly");
            None
        },
        _ => {
            eprintln!("Error: Cannot convert this type to string");
            None
        }
    }
}

fn parse_int_fn(args: Vec<DataHolder>) -> Option<DataHolder> {
    if args.len() != 1 {
        eprintln!("Error: parse_int() expects exactly 1 argument, got {}", args.len());
        return None;
    }
    
    match &args[0] {
        DataHolder::STRING(s) => {
            match s.parse::<i32>() {
                Ok(n) => Some(DataHolder::INTEGER32(n)),
                Err(_) => {
                    eprintln!("Error: Cannot parse '{}' as integer", s);
                    None
                }
            }
        },
        DataHolder::INTEGER32(n) => Some(DataHolder::INTEGER32(*n)), 
        DataHolder::INTEGER64(n) => Some(DataHolder::INTEGER32(*n as i32)), 
        DataHolder::FLOAT32(n) => Some(DataHolder::INTEGER32(*n as i32)), 
        DataHolder::FLOAT64(n) => Some(DataHolder::INTEGER32(*n as i32)), 
        _ => {
            eprintln!("Error: Cannot parse this type as integer");
            None
        }
    }
}
