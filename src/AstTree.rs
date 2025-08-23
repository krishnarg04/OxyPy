use std::collections::HashMap;
use std::str;
use std::time::Instant;

use crate::tokenizer::{Types, DataHolder, Tokens, Callable, ComparisonOperator, LogicalOperator, ConditionalExpression, ExpressionNode, ArithmeticOperator};
use crate::Environment::Environment;

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        data_type: Types,
        value: AstExpressions,
    },
    ListDeclaration {
        name: String,
        elements: Vec<AstExpressions>,
        size: usize,
    },
    Function {
        function: Callable,
        parameters: Vec<AstExpressions>,
        environment: Environment,
    },
    FunctionDeclaration {
        name: String,
        params: Vec<FunctionParameter>,
        body: Vec<Statement>,
    },
    Conditional {
        condition: AstExpressions,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    ForLoop {
        variable: String,
        start: AstExpressions,
        end: AstExpressions,
        step: AstExpressions,
        body: Vec<Statement>,
    },
    WhileLoop {
        condition: AstExpressions,
        body: Vec<Statement>,
    },
    Block(Vec<Statement>),
    Assignment {
        name: String,
        value: AstExpressions,
    },
    
    MemberAssignment {
        object: AstExpressions,
        member: String,
        value: AstExpressions,
    },
    ExpressionStatement {
        expression: AstExpressions,
    },
    
    Return {
        value: Option<AstExpressions>,
    },

    ClassMeta {
        name: String,
        fields: HashMap<String, Statement>,
    },

    ClassAttribute {
        name: String,
        data_type: Types,
    }
}

#[derive(Debug, Clone)]
struct StructField {
    variable_map: HashMap<String, AstExpressions>,
}


#[derive(Debug, Clone)]
pub struct FunctionParameter {
    pub name: String,
    pub data_type: Types,
}

#[derive(Debug, Clone)]
pub enum AstExpressions {
    BinaryOperation {
        left: Box<AstExpressions>,
        operator: ArithmeticOperator,
        right: Box<AstExpressions>,
    },
    UnaryOperation {
        operator: ArithmeticOperator,
        operand: Box<AstExpressions>,
    },
    ComparisonOperation {
        left: Box<AstExpressions>,
        operator: ComparisonOperator,
        right: Box<AstExpressions>,
    },
    LogicalOperation {
        left: Box<AstExpressions>,
        operator: LogicalOperator,
        right: Box<AstExpressions>,
    },
    Value {
        value: DataHolder
    },
    Variable {
        name: String
    },
    Literal {
        value: String
    },
    ListLiteral {
        elements: Vec<AstExpressions>
    },
    FunctionCall {
        name: String,
        arguments: Vec<AstExpressions>,
    },
    
    MemberAccess {
        object: Box<AstExpressions>,
        member: String,
    },
    
    MethodCall {
        object: Box<AstExpressions>,
        method: String,
        arguments: Vec<AstExpressions>,
    },
    Grouping {
        expression: Box<AstExpressions>
    },
}


impl AstExpressions {
    pub fn evaluate(&self, env: &Environment) -> Option<DataHolder> {
        match self {
            AstExpressions::Value { value } => Some(value.clone()),
            
            AstExpressions::Variable { name } => {
                env.get_variable(name).cloned()
            },
            
            AstExpressions::Literal { value } => {
                Some(DataHolder::STRING(value.clone()))
            },
            
            AstExpressions::BinaryOperation { left, operator, right } => {
                let left_val = left.evaluate(env)?;
                let right_val = right.evaluate(env)?;
                self.perform_arithmetic_operation(&left_val, operator, &right_val)
            },
            
            AstExpressions::UnaryOperation { operator, operand } => {
                let operand_val = operand.evaluate(env)?;
                self.perform_unary_operation(operator, &operand_val)
            },
            
            AstExpressions::ComparisonOperation { left, operator, right } => {
                let left_val = left.evaluate(env)?;
                let right_val = right.evaluate(env)?;
                self.perform_comparison_operation(&left_val, operator, &right_val)
            },
            
            AstExpressions::LogicalOperation { left, operator, right } => {
                let left_val = left.evaluate(env)?;
                
                match operator {
                    LogicalOperator::And => {
                        if let DataHolder::BOOLEAN(false) = left_val {
                            Some(DataHolder::BOOLEAN(false)) 
                        } else {
                            let right_val = right.evaluate(env)?;
                            match (left_val, right_val) {
                                (DataHolder::BOOLEAN(a), DataHolder::BOOLEAN(b)) => 
                                    Some(DataHolder::BOOLEAN(a && b)),
                                _ => None,
                            }
                        }
                    },
                    LogicalOperator::Or => {
                        if let DataHolder::BOOLEAN(true) = left_val {
                            Some(DataHolder::BOOLEAN(true)) 
                        } else {
                            let right_val = right.evaluate(env)?;
                            match (left_val, right_val) {
                                (DataHolder::BOOLEAN(a), DataHolder::BOOLEAN(b)) => 
                                    Some(DataHolder::BOOLEAN(a || b)),
                                _ => None,
                            }
                        }
                    },
                }
            },
            
            AstExpressions::ListLiteral { elements } => {
                let mut evaluated_elements = Vec::new();
                for element in elements {
                    if let Some(val) = element.evaluate(env) {
                        evaluated_elements.push(val);
                    } else {
                        return None;
                    }
                }
                Some(DataHolder::LIST(evaluated_elements))
            },
            
            AstExpressions::FunctionCall { name, arguments } => {
                let mut evaluated_args = Vec::new();
                for arg in arguments {
                    if let Some(val) = arg.evaluate(env) {
                        evaluated_args.push(val);
                    } else {
                        return None;
                    }
                }
                self.execute_function_call(name, evaluated_args, env)
            },
            
            AstExpressions::Grouping { expression } => {
                expression.evaluate(env)
            },
            AstExpressions::MemberAccess { object, member } => {
                let obj_val = object.evaluate(env)?;
                match obj_val {
                    DataHolder::CLASSINSTANCE(ref instance) => {
                        if let Some(field_val) = instance.fields.get(member) {
                            Some(field_val.clone())
                        } else {
                            None
                        }
                    },
                    _ => None,
                }
            },
            AstExpressions::MethodCall { object, method, arguments } => {
                let obj_val = object.evaluate(env)?;
                let mut evaluated_args = Vec::new();
                for arg in arguments {
                    if let Some(val) = arg.evaluate(env) {
                        evaluated_args.push(val);
                    } else {
                        return None;
                    }
                }
                match obj_val {
                    DataHolder::CLASSINSTANCE(ref instance) => {
                        
                        
                        
                        eprintln!("Method calls should be handled in runtime, not during AST evaluation");
                        None
                    },
                    _ => None,
                }
            },
        }
    }
    
    fn perform_arithmetic_operation(&self, left: &DataHolder, operator: &ArithmeticOperator, right: &DataHolder) -> Option<DataHolder> {
        match operator {
            ArithmeticOperator::Add => {
                match (left, right) {
                    (DataHolder::INTEGER32(a), DataHolder::INTEGER32(b)) => Some(DataHolder::INTEGER32(a + b)),
                    (DataHolder::INTEGER64(a), DataHolder::INTEGER64(b)) => Some(DataHolder::INTEGER64(a + b)),
                    (DataHolder::FLOAT32(a), DataHolder::FLOAT32(b)) => Some(DataHolder::FLOAT32(a + b)),
                    (DataHolder::FLOAT64(a), DataHolder::FLOAT64(b)) => Some(DataHolder::FLOAT64(a + b)),
                    (DataHolder::STRING(a), DataHolder::STRING(b)) => Some(DataHolder::STRING(format!("{}{}", a, b))),
                    _ => None,
                }
            },
            ArithmeticOperator::Subtract => {
                match (left, right) {
                    (DataHolder::INTEGER32(a), DataHolder::INTEGER32(b)) => Some(DataHolder::INTEGER32(a - b)),
                    (DataHolder::INTEGER64(a), DataHolder::INTEGER64(b)) => Some(DataHolder::INTEGER64(a - b)),
                    (DataHolder::FLOAT32(a), DataHolder::FLOAT32(b)) => Some(DataHolder::FLOAT32(a - b)),
                    (DataHolder::FLOAT64(a), DataHolder::FLOAT64(b)) => Some(DataHolder::FLOAT64(a - b)),
                    _ => None,
                }
            },
            ArithmeticOperator::Multiply => {
                match (left, right) {
                    (DataHolder::INTEGER32(a), DataHolder::INTEGER32(b)) => Some(DataHolder::INTEGER32(a * b)),
                    (DataHolder::INTEGER64(a), DataHolder::INTEGER64(b)) => Some(DataHolder::INTEGER64(a * b)),
                    (DataHolder::FLOAT32(a), DataHolder::FLOAT32(b)) => Some(DataHolder::FLOAT32(a * b)),
                    (DataHolder::FLOAT64(a), DataHolder::FLOAT64(b)) => Some(DataHolder::FLOAT64(a * b)),
                    _ => None,
                }
            },
            ArithmeticOperator::Divide => {
                match (left, right) {
                    (DataHolder::INTEGER32(a), DataHolder::INTEGER32(b)) => {
                        if *b == 0 { None } else { Some(DataHolder::INTEGER32(a / b)) }
                    },
                    (DataHolder::INTEGER64(a), DataHolder::INTEGER64(b)) => {
                        if *b == 0 { None } else { Some(DataHolder::INTEGER64(a / b)) }
                    },
                    (DataHolder::FLOAT32(a), DataHolder::FLOAT32(b)) => {
                        if *b == 0.0 { None } else { Some(DataHolder::FLOAT32(a / b)) }
                    },
                    (DataHolder::FLOAT64(a), DataHolder::FLOAT64(b)) => {
                        if *b == 0.0 { None } else { Some(DataHolder::FLOAT64(a / b)) }
                    },
                    _ => None,
                }
            },
            ArithmeticOperator::Modulo => {
                match (left, right) {
                    (DataHolder::INTEGER32(a), DataHolder::INTEGER32(b)) => {
                        if *b == 0 { None } else { Some(DataHolder::INTEGER32(a % b)) }
                    },
                    (DataHolder::INTEGER64(a), DataHolder::INTEGER64(b)) => {
                        if *b == 0 { None } else { Some(DataHolder::INTEGER64(a % b)) }
                    },
                    _ => None,
                }
            },
            ArithmeticOperator::Not => {
                match left {
                    DataHolder::BOOLEAN(b) => Some(DataHolder::BOOLEAN(!b)),
                    DataHolder::INTEGER32(i) => Some(DataHolder::BOOLEAN(*i == 0)),
                    DataHolder::INTEGER64(i) => Some(DataHolder::BOOLEAN(*i == 0)),
                    _ => None,
                }
            }
        }
    }
    
    fn perform_unary_operation(&self, operator: &ArithmeticOperator, operand: &DataHolder) -> Option<DataHolder> {
        match operator {
            ArithmeticOperator::Subtract => {
                match operand {
                    DataHolder::INTEGER32(n) => Some(DataHolder::INTEGER32(-n)),
                    DataHolder::INTEGER64(n) => Some(DataHolder::INTEGER64(-n)),
                    DataHolder::FLOAT32(n) => Some(DataHolder::FLOAT32(-n)),
                    DataHolder::FLOAT64(n) => Some(DataHolder::FLOAT64(-n)),
                    _ => None,
                }
            },
            ArithmeticOperator::Add => Some(operand.clone()), 
            _ => None, 
        }
    }
    
    fn perform_comparison_operation(&self, left: &DataHolder, operator: &ComparisonOperator, right: &DataHolder) -> Option<DataHolder> {
        match operator {
            ComparisonOperator::Equal => {
                match (left, right) {
                    (DataHolder::INTEGER32(a), DataHolder::INTEGER32(b)) => Some(DataHolder::BOOLEAN(a == b)),
                    (DataHolder::INTEGER64(a), DataHolder::INTEGER64(b)) => Some(DataHolder::BOOLEAN(a == b)),
                    (DataHolder::FLOAT32(a), DataHolder::FLOAT32(b)) => Some(DataHolder::BOOLEAN((a - b).abs() < f32::EPSILON)),
                    (DataHolder::FLOAT64(a), DataHolder::FLOAT64(b)) => Some(DataHolder::BOOLEAN((a - b).abs() < f64::EPSILON)),
                    (DataHolder::STRING(a), DataHolder::STRING(b)) => Some(DataHolder::BOOLEAN(a == b)),
                    (DataHolder::BOOLEAN(a), DataHolder::BOOLEAN(b)) => Some(DataHolder::BOOLEAN(a == b)),
                    _ => Some(DataHolder::BOOLEAN(false)),
                }
            },
            ComparisonOperator::NotEqual => {
                if let Some(DataHolder::BOOLEAN(result)) = self.perform_comparison_operation(left, &ComparisonOperator::Equal, right) {
                    Some(DataHolder::BOOLEAN(!result))
                } else {
                    None
                }
            },
            ComparisonOperator::Greater => {
                match (left, right) {
                    (DataHolder::INTEGER32(a), DataHolder::INTEGER32(b)) => Some(DataHolder::BOOLEAN(a > b)),
                    (DataHolder::INTEGER64(a), DataHolder::INTEGER64(b)) => Some(DataHolder::BOOLEAN(a > b)),
                    (DataHolder::FLOAT32(a), DataHolder::FLOAT32(b)) => Some(DataHolder::BOOLEAN(a > b)),
                    (DataHolder::FLOAT64(a), DataHolder::FLOAT64(b)) => Some(DataHolder::BOOLEAN(a > b)),
                    _ => None,
                }
            },
            ComparisonOperator::Less => {
                match (left, right) {
                    (DataHolder::INTEGER32(a), DataHolder::INTEGER32(b)) => Some(DataHolder::BOOLEAN(a < b)),
                    (DataHolder::INTEGER64(a), DataHolder::INTEGER64(b)) => Some(DataHolder::BOOLEAN(a < b)),
                    (DataHolder::FLOAT32(a), DataHolder::FLOAT32(b)) => Some(DataHolder::BOOLEAN(a < b)),
                    (DataHolder::FLOAT64(a), DataHolder::FLOAT64(b)) => Some(DataHolder::BOOLEAN(a < b)),
                    _ => None,
                }
            },
            ComparisonOperator::GreaterEqual => {
                if let Some(DataHolder::BOOLEAN(less_result)) = self.perform_comparison_operation(left, &ComparisonOperator::Less, right) {
                    Some(DataHolder::BOOLEAN(!less_result))
                } else {
                    None
                }
            },
            ComparisonOperator::LessEqual => {
                if let Some(DataHolder::BOOLEAN(greater_result)) = self.perform_comparison_operation(left, &ComparisonOperator::Greater, right) {
                    Some(DataHolder::BOOLEAN(!greater_result))
                } else {
                    None
                }
            },
        }
    }
    
    fn execute_function_call(&self, func_name: &str, args: Vec<DataHolder>, env: &Environment) -> Option<DataHolder> {
        match func_name {
            "print" => {
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { print!(" "); }
                    match arg {
                        DataHolder::INTEGER32(n) => print!("{}", n),
                        DataHolder::INTEGER64(n) => print!("{}", n),
                        DataHolder::FLOAT32(n) => print!("{}", n),
                        DataHolder::FLOAT64(n) => print!("{}", n),
                        DataHolder::STRING(s) => print!("{}", s),
                        DataHolder::BOOLEAN(b) => print!("{}", b),
                        DataHolder::LIST(list) => print!("{:?}", list),
                        _ => print!("{:?}", arg),
                    }
                }
                println!();
                Some(DataHolder::INTEGER32(0))
            },
            "println" => {
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { print!(" "); }
                    match arg {
                        DataHolder::INTEGER32(n) => print!("{}", n),
                        DataHolder::INTEGER64(n) => print!("{}", n),
                        DataHolder::FLOAT32(n) => print!("{}", n),
                        DataHolder::FLOAT64(n) => print!("{}", n),
                        DataHolder::STRING(s) => print!("{}", s),
                        DataHolder::BOOLEAN(b) => print!("{}", b),
                        DataHolder::LIST(list) => print!("{:?}", list),
                        _ => print!("{:?}", arg),
                    }
                }
                println!();
                Some(DataHolder::INTEGER32(0))
            },
            "len" => {
                if args.len() == 1 {
                    match &args[0] {
                        DataHolder::STRING(s) => Some(DataHolder::INTEGER32(s.len() as i32)),
                        DataHolder::LIST(list) => Some(DataHolder::INTEGER32(list.len() as i32)),
                        _ => None,
                    }
                } else {
                    None
                }
            },
            "millis_as_str" => {
                Some(DataHolder::STRING(Instant::now().elapsed().as_millis().to_string()))
            },
            _ => {
                None
            }
        }
    }
}

struct TokenCursor {
    tokens: Vec<Tokens>,
    position: usize,
}

impl TokenCursor {
    fn new(tokens: Vec<Tokens>) -> Self {
        TokenCursor { tokens, position: 0 }
    }

    fn current_token(&self) -> Option<&Tokens> {
        self.tokens.get(self.position)
    }

    fn peek_token(&self, offset: usize) -> Option<&Tokens> {
        self.tokens.get(self.position + offset)
    }

    fn consume_token(&mut self) -> Option<&Tokens> {
        if self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn match_token(&mut self, expected: &Tokens) -> bool {
        if let Some(current) = self.current_token() {
            if std::mem::discriminant(current) == std::mem::discriminant(expected) {
                self.consume_token();
                return true;
            }
        }
        false
    }

    fn expect_token(&mut self, expected: &Tokens) -> Option<&Tokens> {
        if self.match_token(expected) {
            self.tokens.get(self.position - 1)
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }
}

pub struct ASTParser;

impl ASTParser {
    pub fn new() -> Self {
        ASTParser
    }

    pub fn parse(&mut self, tokens: Vec<Tokens>) -> Vec<Statement> {
        let mut cursor = TokenCursor::new(tokens);
        let mut statements = Vec::new();
        
        while !cursor.is_at_end() {
            if let Some(statement) = self.parse_statement(&mut cursor) {
                statements.push(statement);
            } else {
                cursor.consume_token(); 
            }
        }

        statements
    }

    fn parse_statement(&mut self, cursor: &mut TokenCursor) -> Option<Statement> {
        match cursor.current_token() {
            Some(Tokens::LET) => self.parse_variable_declaration(cursor),
            Some(Tokens::IF) => self.parse_conditional_statement(cursor),
            Some(Tokens::FOR) => self.parse_for_loop(cursor),
            Some(Tokens::FN) => self.parse_function_declaration(cursor),
            Some(Tokens::RETURN) => self.parse_return_statement(cursor),
            Some(Tokens::IDENTIFIER(_)) => self.parse_assignment_or_expression(cursor),
            Some(Tokens::LBRACE) => self.parse_block_statement(cursor),
            Some(Tokens::WHILE) => self.parse_while_loop(cursor),
            Some(Tokens::CLASS) => self.parse_class_declaration(cursor),
            _ => None,
        }
    }

    fn parse_function_declaration(&mut self, cursor: &mut TokenCursor) -> Option<Statement> {
        cursor.expect_token(&Tokens::FN)?; 

        let name = match cursor.consume_token()? {
            Tokens::IDENTIFIER(n) => n.clone(),
            _ => return None,
        };

        cursor.expect_token(&Tokens::LPAREN)?;
        
        let params = self.parse_function_parameters(cursor)?;
        
        cursor.expect_token(&Tokens::RPAREN)?; 

        cursor.expect_token(&Tokens::LBRACE)?;

        let body = self.parse_block_body(cursor)?;
        
        cursor.expect_token(&Tokens::RBRACE)?; 

        Some(Statement::FunctionDeclaration {
            name,
            params,
            body,
        })
    }

    fn parse_function_parameters(&mut self, cursor: &mut TokenCursor) -> Option<Vec<FunctionParameter>> {
        let mut params = Vec::new();

        
        if matches!(cursor.current_token(), Some(Tokens::RPAREN)) {
            return Some(params);
        }

        loop {  
            let param_name = match cursor.consume_token()? {
                Tokens::IDENTIFIER(name) => name.clone(),
                Tokens::SELF => "self".to_string(),
                _ => return None,
            };
            if param_name == "self" {
                params.push(FunctionParameter {
                    name: param_name,
                    data_type: Types::STRING,
                });
            } else {
                
                if !cursor.match_token(&Tokens::COLON) {
                    return None;
                }

                let param_type = match cursor.consume_token()? {
                    Tokens::TYPE(t) => t.clone(),
                    _ => return None,
                };

                params.push(FunctionParameter {
                    name: param_name,
                    data_type: param_type,
                });
            }

            if cursor.match_token(&Tokens::COMMA) {
                continue; 
            } else {
                break; 
            }
        }

        Some(params)
    }

    fn parse_return_statement(&mut self, cursor: &mut TokenCursor) -> Option<Statement> {
        cursor.expect_token(&Tokens::RETURN)?; 

        
        let value = if matches!(cursor.current_token(), Some(Tokens::RBRACE) | Some(Tokens::LET) | Some(Tokens::IF) | Some(Tokens::FOR) | None) {
            
            None
        } else {
            
            Some(self.parse_expression(cursor)?)
        };

        Some(Statement::Return { value })
    }

    fn parse_class_declaration(&mut self, cursor: &mut TokenCursor) -> Option<Statement> {
        if !cursor.match_token(&Tokens::CLASS) {
            return None;
        }

        let class_name = match cursor.consume_token() {
            Some(Tokens::IDENTIFIER(name)) => name.clone(),
            _ => return None,
        };
        
        if !cursor.match_token(&Tokens::LBRACE) {
            return None;
        }
        
        if !cursor.match_token(&Tokens::PUBLIC) {
            return None;
        }
        
        if !cursor.match_token(&Tokens::LBRACE) {
            return None;
        }
        
        let mut fields = HashMap::new();
        
        
        while !matches!(cursor.current_token(), Some(Tokens::RBRACE) | None) {
            let field_name = match cursor.consume_token() {
                Some(Tokens::IDENTIFIER(n)) => n.clone(),
                _ => return None,
            };

            if !cursor.match_token(&Tokens::COLON) {
                return None;
            }
            
            let field_type = match cursor.consume_token() {
                Some(Tokens::TYPE(t)) => t.clone(),
                _ => return None,
            };
            
            fields.insert(field_name.clone(), Statement::ClassAttribute {
                name: field_name,
                data_type: field_type,
            });
        }
        
        if !cursor.match_token(&Tokens::RBRACE) {
            return None;
        }

        
        if matches!(cursor.current_token(), Some(Tokens::PUBLIC)) {
            cursor.consume_token();
            
            if !cursor.match_token(&Tokens::LBRACE) {
                return None;
            }
            
            
            while !matches!(cursor.current_token(), Some(Tokens::RBRACE) | None) {
                if !cursor.match_token(&Tokens::FN) {
                    return None;
                }

                let method_name = match cursor.consume_token() {
                    Some(Tokens::IDENTIFIER(n)) => n.clone(),
                    _ => return None,
                };

                if !cursor.match_token(&Tokens::LPAREN) {
                    return None;
                }

                let params = self.parse_function_parameters(cursor);
                if params.is_none() {
                    return None;
                }
                let params = params.unwrap();
                
                if !cursor.match_token(&Tokens::RPAREN) {
                    return None;
                }

                if !cursor.match_token(&Tokens::LBRACE) {
                    return None;
                }

                let body = self.parse_block_body(cursor);
                if body.is_none() {
                    return None;
                }
                let body = body.unwrap();

                if !cursor.match_token(&Tokens::RBRACE) {
                    return None;
                }

                fields.insert(method_name.clone(), Statement::FunctionDeclaration {
                    name: method_name.clone(),
                    params,
                    body,
                });
            }
            
            if !cursor.match_token(&Tokens::RBRACE) {
                return None;
            }
        }

        if !cursor.match_token(&Tokens::RBRACE) {
            return None;
        }
        
        Some(Statement::ClassMeta {
            name: class_name,
            fields,
        })
    }

    fn parse_variable_declaration(&mut self, cursor: &mut TokenCursor) -> Option<Statement> {
        cursor.expect_token(&Tokens::LET)?;

        let name = match cursor.consume_token()? {
            Tokens::IDENTIFIER(n) => n.clone(),
            _ => return None,
        };

        let mut data_type = None;
        if cursor.match_token(&Tokens::COLON) {
            data_type = match cursor.consume_token()? {
                Tokens::TYPE(t) => Some(t.clone()),
                _ => return None,
            };
        }

        cursor.expect_token(&Tokens::EQUALS)?;
        let value_expr = self.parse_expression(cursor)?;

        
        if data_type.is_none() {
            data_type = Some(Types::STRING);
        }

        Some(Statement::VariableDeclaration {
            name,
            data_type: data_type.unwrap(),
            value: value_expr,
        })
    }

    fn parse_conditional_statement(&mut self, cursor: &mut TokenCursor) -> Option<Statement> {
        cursor.expect_token(&Tokens::IF)?;
        cursor.expect_token(&Tokens::LPAREN)?;

        let condition = self.parse_expression(cursor)?;
        
        cursor.expect_token(&Tokens::RPAREN)?;
        cursor.expect_token(&Tokens::LBRACE)?;

        let then_branch = self.parse_block_body(cursor)?;

        cursor.expect_token(&Tokens::RBRACE)?;

        let else_branch = if cursor.match_token(&Tokens::ELSE) {
            cursor.expect_token(&Tokens::LBRACE)?;
            let else_statements = self.parse_block_body(cursor)?;
            cursor.expect_token(&Tokens::RBRACE)?;
            Some(else_statements)
        } else {
            None
        };
        
        Some(Statement::Conditional {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_for_loop(&mut self, cursor: &mut TokenCursor) -> Option<Statement> {
        cursor.expect_token(&Tokens::FOR)?; 
        
        let variable = match cursor.consume_token()? {
            Tokens::IDENTIFIER(n) => n.clone(),
            _ => return None,
        };

        cursor.expect_token(&Tokens::IN)?; 
        cursor.expect_token(&Tokens::DOT)?;
        cursor.expect_token(&Tokens::SLASH)?;
        cursor.expect_token(&Tokens::LSQRBRAC)?;
        
        let start = self.parse_expression(cursor)?;
        cursor.expect_token(&Tokens::COMMA)?;
        
        let end = self.parse_expression(cursor)?;
        cursor.expect_token(&Tokens::COMMA)?;
        
        let step = self.parse_expression(cursor)?;

        cursor.expect_token(&Tokens::RSQRBRAC)?;
        cursor.expect_token(&Tokens::LBRACE)?;

        let body = self.parse_block_body(cursor)?;

        cursor.expect_token(&Tokens::RBRACE)?;
        
        Some(Statement::ForLoop {
            variable,
            start,
            end,
            step,
            body,
        })
    }

    fn parse_while_loop(&mut self, cursor: &mut TokenCursor) -> Option<Statement> {
        cursor.expect_token(&Tokens::WHILE)?;
        cursor.expect_token(&Tokens::LPAREN)?;

        let condition = self.parse_expression(cursor)?;

        cursor.expect_token(&Tokens::RPAREN)?;
        cursor.expect_token(&Tokens::LBRACE)?;

        let body = self.parse_block_body(cursor)?;

        cursor.expect_token(&Tokens::RBRACE)?;

        Some(Statement::WhileLoop {
            condition,
            body,
        })
    }

    fn parse_assignment_or_expression(&mut self, cursor: &mut TokenCursor) -> Option<Statement> {
        
        let start_pos = cursor.position;
        
        
        if let Some(expr) = self.parse_expression(cursor) {
            
            if cursor.match_token(&Tokens::EQUALS) {
                let value = self.parse_expression(cursor)?;
                
                
                match expr {
                    AstExpressions::MemberAccess { object, member } => {
                        Some(Statement::MemberAssignment {
                            object: *object,
                            member,
                            value,
                        })
                    },
                    AstExpressions::Variable { name } => {
                        Some(Statement::Assignment { name, value })
                    },
                    _ => {
                        None
                    }
                }
            } else {
                
                Some(Statement::ExpressionStatement { expression: expr })
            }
        } else {
            
            cursor.position = start_pos;
            
            let name = match cursor.consume_token()? {
                Tokens::IDENTIFIER(n) => n.clone(),
                _ => return None,
            };

            if cursor.match_token(&Tokens::EQUALS) {
                let value = self.parse_expression(cursor)?;
                Some(Statement::Assignment { name, value })
            } else {
                
                cursor.position -= 1;
                let expr = self.parse_expression(cursor)?;
                Some(Statement::ExpressionStatement { expression: expr })
            }
        }
    }

    fn parse_block_statement(&mut self, cursor: &mut TokenCursor) -> Option<Statement> {
        cursor.expect_token(&Tokens::LBRACE)?;
        let statements = self.parse_block_body(cursor)?;
        cursor.expect_token(&Tokens::RBRACE)?;
        Some(Statement::Block(statements))
    }

    fn parse_block_body(&mut self, cursor: &mut TokenCursor) -> Option<Vec<Statement>> {
        let mut statements = Vec::new();

        while !cursor.is_at_end() && !matches!(cursor.current_token(), Some(Tokens::RBRACE)) {
            if let Some(stmt) = self.parse_statement(cursor) {
                statements.push(stmt);
            } else {
                cursor.consume_token(); 
            }
        }

        Some(statements)
    }

    
    fn parse_expression(&mut self, cursor: &mut TokenCursor) -> Option<AstExpressions> {
        self.parse_logical_or(cursor)
    }

    fn parse_logical_or(&mut self, cursor: &mut TokenCursor) -> Option<AstExpressions> {
        let mut left = self.parse_logical_and(cursor)?;

        while cursor.match_token(&Tokens::OR) {
            let right = self.parse_logical_and(cursor)?;
            left = AstExpressions::LogicalOperation {
                left: Box::new(left),
                operator: LogicalOperator::Or,
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn parse_logical_and(&mut self, cursor: &mut TokenCursor) -> Option<AstExpressions> {
        let mut left = self.parse_equality(cursor)?;

        while cursor.match_token(&Tokens::AND) {
            let right = self.parse_equality(cursor)?;
            left = AstExpressions::LogicalOperation {
                left: Box::new(left),
                operator: LogicalOperator::And,
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn parse_equality(&mut self, cursor: &mut TokenCursor) -> Option<AstExpressions> {
        let mut left = self.parse_comparison(cursor)?;

        while let Some(operator) = self.match_comparison_operator(cursor) {
            let right = self.parse_comparison(cursor)?;
            left = AstExpressions::ComparisonOperation {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn parse_comparison(&mut self, cursor: &mut TokenCursor) -> Option<AstExpressions> {
        self.parse_term(cursor)
    }

    fn parse_term(&mut self, cursor: &mut TokenCursor) -> Option<AstExpressions> {
        let mut left = self.parse_factor(cursor)?;

        while let Some(operator) = self.match_arithmetic_operator(cursor, &[Tokens::PLUS, Tokens::MINUS]) {
            let right = self.parse_factor(cursor)?;
            left = AstExpressions::BinaryOperation {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn parse_factor(&mut self, cursor: &mut TokenCursor) -> Option<AstExpressions> {
        let mut left = self.parse_unary(cursor)?;

        while let Some(operator) = self.match_arithmetic_operator(cursor, &[Tokens::STAR, Tokens::SLASH, Tokens::MODULO]) {
            let right = self.parse_unary(cursor)?;
            left = AstExpressions::BinaryOperation {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn parse_unary(&mut self, cursor: &mut TokenCursor) -> Option<AstExpressions> {
        if let Some(operator) = self.match_unary_operator(cursor) {
            let operand = self.parse_unary(cursor)?;
            Some(AstExpressions::UnaryOperation {
                operator,
                operand: Box::new(operand),
            })
        } else {
            self.parse_primary(cursor)
        }
    }

    fn parse_primary(&mut self, cursor: &mut TokenCursor) -> Option<AstExpressions> {
        match cursor.current_token() {
            Some(Tokens::VALUE(value)) => {
                let val = value.clone();
                cursor.consume_token();
                Some(AstExpressions::Value { value: val })
            },
            Some(Tokens::IDENTIFIER(name)) => {
                let name = name.clone();
                cursor.consume_token();
                
                
                self.parse_member_access_or_call(AstExpressions::Variable { name }, cursor)
            },
            Some(Tokens::SELF) => {
                cursor.consume_token();
                let self_expr = AstExpressions::Variable { name: "self".to_string() };
                
                
                self.parse_member_access_or_call(self_expr, cursor)
            },
            Some(Tokens::LPAREN) => {
                cursor.consume_token();
                let expr = self.parse_expression(cursor)?;
                cursor.expect_token(&Tokens::RPAREN)?;
                Some(AstExpressions::Grouping { expression: Box::new(expr) })
            },
            Some(Tokens::LSQRBRAC) => {
                cursor.consume_token();
                let mut elements = Vec::new();
                
                if !matches!(cursor.current_token(), Some(Tokens::RSQRBRAC)) {
                    loop {
                        elements.push(self.parse_expression(cursor)?);
                        if cursor.match_token(&Tokens::COMMA) {
                            continue;
                        } else {
                            break;
                        }
                    }
                }
                cursor.expect_token(&Tokens::RSQRBRAC)?;
                Some(AstExpressions::ListLiteral { elements })
            },
            _ => None,
        }
    }
    
    fn parse_member_access_or_call(&mut self, mut expr: AstExpressions, cursor: &mut TokenCursor) -> Option<AstExpressions> {
        loop {
            match cursor.current_token() {
                Some(Tokens::DOT) => {
                    cursor.consume_token(); 
                    
                    let member_name = match cursor.consume_token()? {
                        Tokens::IDENTIFIER(name) => name.clone(),
                        _ => return None,
                    };  
                    
                    if matches!(cursor.current_token(), Some(Tokens::LPAREN)) {
                        cursor.consume_token(); 
                        let arguments = self.parse_function_arguments(cursor)?;
                        cursor.expect_token(&Tokens::RPAREN)?;
                        
                        expr = AstExpressions::MethodCall {
                            object: Box::new(expr),
                            method: member_name,
                            arguments,
                        };
                    } else {
                        
                        expr = AstExpressions::MemberAccess {
                            object: Box::new(expr),
                            member: member_name,
                        };
                    }
                },
                Some(Tokens::LPAREN) => {
                    
                    cursor.consume_token(); 
                    let arguments = self.parse_function_arguments(cursor)?;
                    cursor.expect_token(&Tokens::RPAREN)?;
                    
                    
                    if let AstExpressions::Variable { name } = expr {
                        expr = AstExpressions::FunctionCall {
                            name,
                            arguments,
                        };
                    } else {
                        return None; 
                    }
                },
                _ => break,
            }
        }
        
        Some(expr)
    }

    fn parse_function_arguments(&mut self, cursor: &mut TokenCursor) -> Option<Vec<AstExpressions>> {
        let mut args = Vec::new();

        if matches!(cursor.current_token(), Some(Tokens::RPAREN)) {
            return Some(args);
        }

        loop {
            args.push(self.parse_expression(cursor)?);
            if cursor.match_token(&Tokens::COMMA) {
                continue;
            } else {
                break;
            }
        }

        Some(args)
    }

    fn match_comparison_operator(&mut self, cursor: &mut TokenCursor) -> Option<ComparisonOperator> {
        match cursor.current_token() {
            Some(Tokens::EQUALS_EQUALS) => {
                cursor.consume_token();
                Some(ComparisonOperator::Equal)
            },
            Some(Tokens::NOT_EQUALS) => {
                cursor.consume_token();
                Some(ComparisonOperator::NotEqual)
            },
            Some(Tokens::GREATER) => {
                cursor.consume_token();
                Some(ComparisonOperator::Greater)
            },
            Some(Tokens::LESS) => {
                cursor.consume_token();
                Some(ComparisonOperator::Less)
            },
            Some(Tokens::GREATER_EQUALS) => {
                cursor.consume_token();
                Some(ComparisonOperator::GreaterEqual)
            },
            Some(Tokens::LESS_EQUALS) => {
                cursor.consume_token();
                Some(ComparisonOperator::LessEqual)
            },
            _ => None,
        }
    }

    fn match_arithmetic_operator(&mut self, cursor: &mut TokenCursor, allowed_tokens: &[Tokens]) -> Option<ArithmeticOperator> {
        if let Some(current) = cursor.current_token() {
            
            let current_token = current.clone();
            
            for token in allowed_tokens {
                if std::mem::discriminant(&current_token) == std::mem::discriminant(token) {
                    cursor.consume_token(); 
                    return match current_token {
                        Tokens::PLUS => Some(ArithmeticOperator::Add),
                        Tokens::MINUS => Some(ArithmeticOperator::Subtract),
                        Tokens::STAR => Some(ArithmeticOperator::Multiply),
                        Tokens::SLASH => Some(ArithmeticOperator::Divide),
                        Tokens::MODULO => Some(ArithmeticOperator::Modulo),
                        _ => None,
                    };
                }
            }
        }
        None
    }

    fn match_unary_operator(&mut self, cursor: &mut TokenCursor) -> Option<ArithmeticOperator> {
        match cursor.current_token() {
            Some(Tokens::MINUS) => {
                cursor.consume_token();
                Some(ArithmeticOperator::Subtract)
            },
            Some(Tokens::PLUS) => {
                cursor.consume_token();
                Some(ArithmeticOperator::Add)
            },
            _ => None,
        }
    }
}