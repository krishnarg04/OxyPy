use std::{collections::HashMap, str};

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    LET,
    IDENTIFIER(String),
    COLON,
    TYPE(Types),
    VALUE(DataHolder),
    PLUS,
    MINUS,
    STAR,
    SLASH,
    LPAREN,
    RPAREN,
    EQUALS,
    COMMA, 
    DOUBLEQUOTES,
    SINGLEQUOTES,
    LSQRBRAC,
    RSQRBRAC,
    IF,
    ELSE,
    LBRACE,      
    RBRACE,       
    EQUALS_EQUALS, 
    GREATER,       
    LESS,          
    OR,
    AND,
    NOT,
    NOT_EQUALS, 
    LESS_EQUALS,
    GREATER_EQUALS,
    MODULO,
    FOR,
    IN,
    DOT,
    FN,
    RETURN,
    WHILE,
    CLASS,
    PUBLIC,
    SELF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Types{
    INTEGER32,
    INTEGER64,
    FLOAT32,
    FLOAT64,
    BOOLEAN,
    STRING,
    LIST,
    NONE,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataHolder {
    INTEGER32(i32),
    INTEGER64(i64),
    FLOAT32(f32),
    FLOAT64(f64),
    BOOLEAN(bool),
    STRING(String),
    LIST(Vec<DataHolder>),
    FUNCTION(String),
    CONDITIONAL_EXPRESSION(Box<ConditionalExpression>),
    CLASSINSTANCE(ClassInstance),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassInstance {
    pub class_name: String,
    pub fields: HashMap<String, DataHolder>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionalExpression {
    
    Comparison {
        left: ExpressionNode,
        operator: ComparisonOperator,
        right: ExpressionNode,
    },
    
    Logical {
        left: Box<ConditionalExpression>,
        operator: LogicalOperator,
        right: Box<ConditionalExpression>,
    },
    
    Value(ExpressionNode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    Literal(DataHolder),
    Variable(String),
    BinaryOperation {
        left: Box<ExpressionNode>,
        operator: ArithmeticOperator,
        right: Box<ExpressionNode>,
    },
    FunctionCall {
        name: String,
        args: Vec<ExpressionNode>,
    },
    ConditionalExpression(Box<ConditionalExpression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Not,  
}

impl DataHolder {
    pub fn get_type(&self) -> Types {
        match self {
            DataHolder::INTEGER32(_) => Types::INTEGER32,
            DataHolder::INTEGER64(_) => Types::INTEGER64,
            DataHolder::FLOAT32(_) => Types::FLOAT32,
            DataHolder::FLOAT64(_) => Types::FLOAT64,
            DataHolder::BOOLEAN(_) => Types::BOOLEAN,
            DataHolder::STRING(_) => Types::STRING,
            DataHolder::LIST(_) => Types::LIST,
            DataHolder::FUNCTION(_) => Types::STRING,
            _ => Types::NONE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Callable{
    Build_in : BuildInFunction
}

#[derive(Debug, Clone)]
struct BuildInFunction{
    name: String,
    param_count: i32,
    body: fn(Vec<DataHolder>) -> DataHolder,
}

pub struct Tokenizer;

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer
    }

    
    pub fn process_content(&self, content: &str) -> Vec<Tokens> {
        let mut tokens = Vec::new();
        let mut token = String::new();
        let mut chars = content.chars().peekable();

        while let Some(char) = chars.next() {
            match char {
                ' ' | '\t' | '\n' | '\r' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    
                }
                '"' => {
                    
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    
                    let mut string_content = String::new();
                    let mut escaped = false;
                    
                    
                    while let Some(inner_char) = chars.next() {
                        if escaped {
                            
                            match inner_char {
                                'n' => string_content.push('\n'),
                                't' => string_content.push('\t'),
                                'r' => string_content.push('\r'),
                                '\\' => string_content.push('\\'),
                                '"' => string_content.push('"'),
                                '\'' => string_content.push('\''),
                                _ => {
                                    string_content.push('\\');
                                    string_content.push(inner_char);
                                }
                            }
                            escaped = false;
                        } else if inner_char == '\\' {
                            escaped = true;
                        } else if inner_char == '"' {
                            break; 
                        } else {
                            string_content.push(inner_char);
                        }
                    }
                    
                    tokens.push(Tokens::VALUE(DataHolder::STRING(string_content)));
                }
                '\'' => {
                    
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    
                    let mut string_content = String::new();
                    let mut escaped = false;
                    
                    
                    while let Some(inner_char) = chars.next() {
                        if escaped {
                            
                            match inner_char {
                                'n' => string_content.push('\n'),
                                't' => string_content.push('\t'),
                                'r' => string_content.push('\r'),
                                '\\' => string_content.push('\\'),
                                '"' => string_content.push('"'),
                                '\'' => string_content.push('\''),
                                _ => {
                                    string_content.push('\\');
                                    string_content.push(inner_char);
                                }
                            }
                            escaped = false;
                        } else if inner_char == '\\' {
                            escaped = true;
                        } else if inner_char == '\'' {
                            break; 
                        } else {
                            string_content.push(inner_char);
                        }
                    }
                    
                    tokens.push(Tokens::VALUE(DataHolder::STRING(string_content)));
                }
                '+' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::PLUS);
                }
                '-' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::MINUS);
                }
                '*' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::STAR);
                }
                '/' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::SLASH);
                }
                '(' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::LPAREN);
                }
                ')' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::RPAREN);
                }
                ':' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::COLON);
                }
                '[' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::LSQRBRAC);
                }
                ',' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::COMMA);
                }
                ']' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::RSQRBRAC);
                }
                '{' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::LBRACE);
                }
                '}' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::RBRACE);
                }
                '=' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }

                    
                    if chars.peek() == Some(&'=') {
                        chars.next(); 
                        tokens.push(Tokens::EQUALS_EQUALS);
                    } else {
                        
                        match tokens.last() {
                            Some(&Tokens::NOT) => {
                                tokens.pop();
                                tokens.push(Tokens::NOT_EQUALS);
                            }
                            Some(&Tokens::LESS) => {
                                tokens.pop();
                                tokens.push(Tokens::LESS_EQUALS);
                            }
                            Some(&Tokens::GREATER) => {
                                tokens.pop();
                                tokens.push(Tokens::GREATER_EQUALS);
                            }
                            _ => {
                                tokens.push(Tokens::EQUALS);
                            }
                        }
                    }
                }
                '>' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::GREATER);
                }
                '<' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::LESS);
                }
                '!' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    
                    
                    if chars.peek() == Some(&'=') {
                        chars.next(); 
                        tokens.push(Tokens::NOT_EQUALS);
                    } else {
                        tokens.push(Tokens::NOT);
                    }
                }
                '%' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::MODULO);
                }
                '.' => {
                    if !token.is_empty() {
                        tokens.push(self.classify_token(&token));
                        token.clear();
                    }
                    tokens.push(Tokens::DOT);
                }
                _ => token.push(char),
            }
        }

        if !token.is_empty() {
            tokens.push(self.classify_token(&token));
        }

        tokens
    }

    
    pub fn process_line(&self, line: &str) -> Vec<Tokens> {
        self.process_content(line)
    }

    fn classify_token(&self, word: &str) -> Tokens {
        match word {
            "let" => Tokens::LET,
            "for" => Tokens::FOR,
            "in" => Tokens::IN,
            "i32" => Tokens::TYPE(Types::INTEGER32),
            "i64" => Tokens::TYPE(Types::INTEGER64),
            "f32" => Tokens::TYPE(Types::FLOAT32),
            "f64" => Tokens::TYPE(Types::FLOAT64),
            "bool" => Tokens::TYPE(Types::BOOLEAN),
            "string" => Tokens::TYPE(Types::STRING),
            "true" => Tokens::VALUE(DataHolder::BOOLEAN(true)),
            "false" => Tokens::VALUE(DataHolder::BOOLEAN(false)),
            "if" => Tokens::IF,
            "else" => Tokens::ELSE,
            "or" => Tokens::OR,
            "and" => Tokens::AND,
            "not" => Tokens::NOT,
            "fn" => Tokens::FN,
            "return" => Tokens::RETURN,
            "while" => Tokens::WHILE,
            "class" => Tokens::CLASS,
            "public" => Tokens::PUBLIC,
            "self" => Tokens::SELF,
            _ => {
                if let Some(value) = self.try_parse_number(word) {
                    return Tokens::VALUE(value);
                }
                Tokens::IDENTIFIER(word.to_string())
            }
        }
    }

    fn try_parse_number(&self, word: &str) -> Option<DataHolder> {
        if let Ok(value) = word.parse::<i32>() {
            return Some(DataHolder::INTEGER32(value));
        }

        if let Ok(value) = word.parse::<i64>() {
            return Some(DataHolder::INTEGER64(value));
        }

        if word.contains('.') {
            if let Ok(value) = word.parse::<f32>() {
                return Some(DataHolder::FLOAT32(value));
            }
            
            if let Ok(value) = word.parse::<f64>() {
                return Some(DataHolder::FLOAT64(value));
            }
        }

        None
    }
}