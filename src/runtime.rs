use std::collections::HashMap;
use std::time::Instant;
use crate::tokenizer::{ClassInstance, DataHolder, Types};
use crate::Environment::Environment;
use crate::AstTree::{Statement, AstExpressions, FunctionParameter};
use crate::Functions::get_built_in_functions;


#[derive(Debug, Clone)]
pub enum ExecutionResult {
    None,
    Return(DataHolder),
    Continue,
}


#[derive(Debug, Clone)]
pub struct MethodContext {
    pub instance: DataHolder, 
}


#[derive(Debug, Clone)]
pub struct UserFunction {
    pub name: String,
    pub params: Vec<FunctionParameter>,
    pub body: Vec<Statement>,
    pub is_method: bool, 
}

pub struct Runtime {
    environment: Environment,
    functions: HashMap<String, UserFunction>,
    returning: bool,
    return_value: Option<DataHolder>,
    method_context: Option<MethodContext>, 
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            environment: Environment::new(),
            functions: HashMap::new(),
            returning: false,
            return_value: None,
            method_context: None,
        }
    }

    pub fn execute_statements(&mut self, statements: Vec<Statement>) {
        for statement in statements.iter() {
            self.execute_statement(statement.clone());
            
            if self.returning {
                break;
            }
        }
    }

    pub fn execute_statement(&mut self, statement: Statement) -> ExecutionResult {
        
        if self.returning {
            return ExecutionResult::Return(self.return_value.clone().unwrap_or(DataHolder::INTEGER32(0)));
        }

        match statement {
            Statement::ClassMeta { name, fields } => {
                self.environment.set_class(name.clone(), Statement::ClassMeta { name: name.clone(), fields: fields.clone() });
                ExecutionResult::Continue
            },

            Statement::VariableDeclaration { name, data_type: _, value } => {
                if let Some(evaluated_value) = self.evaluate_expression(&value) {
                    self.environment.set_variable(name.clone(), evaluated_value);
                } else {
                    println!("ERROR: Failed to evaluate expression for variable '{}'", name);
                }
                ExecutionResult::Continue
            },
            
            Statement::ListDeclaration { name: _, elements: _, size: _ } => {
                ExecutionResult::Continue
            },
            
            Statement::Assignment { name, value } => {
                if let Some(evaluated_value) = self.evaluate_expression(&value) {
                    self.environment.set_variable(name, evaluated_value);
                }
                ExecutionResult::Continue
            },
            
            
            Statement::MemberAssignment { object, member, value } => {
                if let Some(new_value) = self.evaluate_expression(&value) {
                    if let AstExpressions::Variable { name: var_name } = object {
                        if let Some(obj_value) = self.environment.get_variable(&var_name).cloned() {
                            match obj_value {
                                DataHolder::CLASSINSTANCE(mut instance) => {
                                    instance.fields.insert(member.clone(), new_value);
                                    self.environment.set_variable(var_name, DataHolder::CLASSINSTANCE(instance));
                                },
                                _ => {
                                    eprintln!("Error: Cannot assign to member '{}' on non-object", member);
                                }
                            }
                        }
                    }
                }
                ExecutionResult::Continue
            },
            
            Statement::Conditional { condition, then_branch, else_branch } => {
                if let Some(condition_result) = self.evaluate_expression(&condition) {
                    let should_execute_then = match condition_result {
                        DataHolder::BOOLEAN(b) => b,
                        DataHolder::INTEGER32(i) => i != 0,
                        DataHolder::INTEGER64(i) => i != 0,
                        DataHolder::FLOAT32(f) => f != 0.0,
                        DataHolder::FLOAT64(f) => f != 0.0,
                        DataHolder::STRING(s) => !s.is_empty(),
                        DataHolder::LIST(list) => !list.is_empty(),
                        _ => false,
                    };
                    
                    if should_execute_then {
                        for stmt in then_branch {
                            let result = self.execute_statement(stmt);
                            if matches!(result, ExecutionResult::Return(_)) {
                                return result;
                            }
                        }
                    } else if let Some(else_statements) = else_branch {
                        for stmt in else_statements {
                            let result = self.execute_statement(stmt);
                            if matches!(result, ExecutionResult::Return(_)) {
                                return result;
                            }
                        }
                    }
                }
                ExecutionResult::Continue
            },
            
            Statement::Block(statements) => {
                for stmt in statements {
                    let result = self.execute_statement(stmt);
                    if matches!(result, ExecutionResult::Return(_)) {
                        return result;
                    }
                }
                ExecutionResult::Continue
            },
            
            Statement::ExpressionStatement { expression } => {
                self.evaluate_expression(&expression);
                ExecutionResult::Continue
            },
            
            Statement::ForLoop { variable, start, end, step, body } => {
                let start_val = self.evaluate_expression(&start);
                let end_val = self.evaluate_expression(&end);
                let step_val = self.evaluate_expression(&step);
                
                if let (Some(start_num), Some(end_num), Some(step_num)) = (start_val, end_val, step_val) {
                    match (start_num, end_num, step_num) {
                        (DataHolder::INTEGER32(start), DataHolder::INTEGER32(end), DataHolder::INTEGER32(step)) => {
                            let mut current = start;
                            while (step > 0 && current < end) || (step < 0 && current > end) {
                                self.environment.set_variable(variable.clone(), DataHolder::INTEGER32(current));

                                for stmt in &body {
                                    let result = self.execute_statement(stmt.clone());
                                    if matches!(result, ExecutionResult::Return(_)) {
                                        return result;
                                    }
                                }
                                
                                current += step;
                            }
                        },
                        (DataHolder::INTEGER64(start), DataHolder::INTEGER64(end), DataHolder::INTEGER64(step)) => {
                            let mut current = start;
                            while (step > 0 && current < end) || (step < 0 && current > end) {
                                self.environment.set_variable(variable.clone(), DataHolder::INTEGER64(current));
                                
                                for stmt in &body {
                                    let result = self.execute_statement(stmt.clone());
                                    if matches!(result, ExecutionResult::Return(_)) {
                                        return result;
                                    }
                                }
                                
                                current += step;
                            }
                        },
                        _ => {
                            println!("For loop requires numeric values for start, end, and step");
                        }
                    }
                }
                ExecutionResult::Continue
            },

            Statement::WhileLoop { condition, body } => {
                loop {
                    if let Some(condition_result) = self.evaluate_expression(&condition) {
                        let should_continue = match condition_result {
                            DataHolder::BOOLEAN(b) => b,
                            DataHolder::INTEGER32(i) => i != 0,
                            DataHolder::INTEGER64(i) => i != 0,
                            DataHolder::FLOAT32(f) => f != 0.0,
                            DataHolder::FLOAT64(f) => f != 0.0,
                            DataHolder::STRING(s) => !s.is_empty(),
                            DataHolder::LIST(list) => !list.is_empty(),
                            _ => false,
                        };
                        
                        if !should_continue {
                            break;
                        }
                        
                        for stmt in &body {
                            let result = self.execute_statement(stmt.clone());
                            if matches!(result, ExecutionResult::Return(_)) {
                                return result;
                            }
                        }
                    } else {
                        break;
                    }
                }
                ExecutionResult::Continue
            },

            Statement::FunctionDeclaration { name, params, body } => {
                let user_function = UserFunction {
                    name: name.clone(),
                    params,
                    body,
                    is_method: false, 
                };
                self.functions.insert(name.clone(), user_function);
                ExecutionResult::Continue
            },
            
            Statement::Return { value } => {
                let return_val = if let Some(expr) = value {
                    self.evaluate_expression(&expr).unwrap_or(DataHolder::INTEGER32(0))
                } else {
                    DataHolder::INTEGER32(0) 
                };
                
                self.returning = true;
                self.return_value = Some(return_val.clone());
                ExecutionResult::Return(return_val)
            },
            
            _ => {
                println!("Unhandled statement: {:?}", statement);
                ExecutionResult::Continue
            }
        }
    }
    
    
    pub fn evaluate_expression(&mut self, expr: &AstExpressions) -> Option<DataHolder> {
        match expr {
            AstExpressions::Value { value } => Some(value.clone()),
            
            AstExpressions::Variable { name } => {
                
                if name == "self" {
                    if let Some(context) = &self.method_context {
                        return Some(context.instance.clone());
                    }
                }
                self.environment.get_variable(name).cloned()
            },
            
            AstExpressions::Literal { value } => {
                Some(DataHolder::STRING(value.clone()))
            },
            
            AstExpressions::BinaryOperation { left, operator, right } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.perform_arithmetic_operation(&left_val, operator, &right_val)
            },
            
            AstExpressions::UnaryOperation { operator, operand } => {
                let operand_val = self.evaluate_expression(operand)?;
                match operator {
                    
                    crate::tokenizer::ArithmeticOperator::Not => {
                        match operand_val {
                            DataHolder::BOOLEAN(b) => Some(DataHolder::BOOLEAN(!b)),
                            DataHolder::INTEGER32(i) => Some(DataHolder::BOOLEAN(i == 0)),
                            DataHolder::INTEGER64(i) => Some(DataHolder::BOOLEAN(i == 0)),
                            _ => None,
                        }
                    },
                    _ => self.perform_unary_operation(operator, &operand_val)
                }
            },
            
            AstExpressions::ComparisonOperation { left, operator, right } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.perform_comparison_operation(&left_val, operator, &right_val)
            },
            
            AstExpressions::LogicalOperation { left, operator, right } => {
                let left_val = self.evaluate_expression(left)?;
                
                match operator {
                    crate::tokenizer::LogicalOperator::And => {
                        if let DataHolder::BOOLEAN(false) = left_val {
                            Some(DataHolder::BOOLEAN(false)) 
                        } else {
                            let right_val = self.evaluate_expression(right)?;
                            match (left_val, right_val) {
                                (DataHolder::BOOLEAN(a), DataHolder::BOOLEAN(b)) => 
                                    Some(DataHolder::BOOLEAN(a && b)),
                                _ => None,
                            }
                        }
                    },
                    crate::tokenizer::LogicalOperator::Or => {
                        if let DataHolder::BOOLEAN(true) = left_val {
                            Some(DataHolder::BOOLEAN(true)) 
                        } else {
                            let right_val = self.evaluate_expression(right)?;
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
                    if let Some(val) = self.evaluate_expression(element) {
                        evaluated_elements.push(val);
                    } else {
                        return None;
                    }
                }
                Some(DataHolder::LIST(evaluated_elements))
            },
            
            AstExpressions::MemberAccess { object, member } => {
                let obj_value = self.evaluate_expression(object)?;
                
                match obj_value {
                    DataHolder::CLASSINSTANCE(ref instance) => {
                        
                        instance.fields.get(member).cloned()
                    },
                    _ => {
                        eprintln!("Error: Cannot access member '{}' on non-object", member);
                        None
                    }
                }
            },

            AstExpressions::MethodCall { object, method, arguments } => {
                let obj_value = self.evaluate_expression(object);
                
                if obj_value.is_none() {
                    if let AstExpressions::Variable { name } = object.as_ref() { 
                        eprintln!("Variable '{}' not found in environment", name);
                        eprintln!("Available variables: {:?}", self.environment.get_all_variables().keys().collect::<Vec<_>>());
                    }
                    return None;
                }
                
                let obj_value = obj_value.unwrap();
                
                match obj_value {
                    DataHolder::CLASSINSTANCE(ref instance) => {

                        let mut evaluated_args = Vec::new();
                        for arg in arguments {
                            if let Some(val) = self.evaluate_expression(arg) {
                                evaluated_args.push(val);
                            } else {
                                return None;
                            }
                        }
                        
                        self.call_method(&instance.class_name, method, obj_value.clone(), evaluated_args)
                    },
                    _ => {
                        eprintln!("Error: Cannot call method '{}' on non-object: {:?}", method, obj_value);
                        None
                    }
                }
            },

            AstExpressions::FunctionCall { name, arguments } => {

                let is_class = self.environment.is_class_meta_exists(name);
                
                if is_class {
                    let result = self.create_class_instance(name, arguments);
                    return result;
                }
                let mut evaluated_args = Vec::new();
                for arg in arguments {
                    if let Some(val) = self.evaluate_expression(arg) {
                        evaluated_args.push(val);
                    } else {
                        return None;
                    }
                }
                self.call_function(name, evaluated_args)
            },

            AstExpressions::Grouping { expression } => {
                self.evaluate_expression(expression)
            },
        }
    }
    
    
    fn create_class_instance(&mut self, class_name: &str, arguments: &Vec<AstExpressions>) -> Option<DataHolder> {
        if let Some(class_def) = self.environment.get_class(class_name) {
            if let Statement::ClassMeta { name, fields } = class_def {
                let mut instance_fields = HashMap::new();
                
                
                for (field_name, field_stmt) in fields.iter() {
                    match field_stmt {
                        Statement::ClassAttribute { name: _, data_type } => {
                            let default_value = self.get_default_value(data_type);
                            instance_fields.insert(field_name.clone(), default_value);
                        },
                        Statement::FunctionDeclaration { .. } => {
                            
                            continue;
                        },
                        _ => continue,
                    }
                }
                
                let mut instance = DataHolder::CLASSINSTANCE(ClassInstance {
                    class_name: name.clone(),
                    fields: instance_fields,
                });
                
                
                if fields.contains_key("__init__") {
                    let mut evaluated_args = Vec::new();
                    for arg in arguments {
                        if let Some(val) = self.evaluate_expression(arg) {
                            evaluated_args.push(val);
                        } else {
                            return None;
                        }
                    }
                    
                    
                    self.call_method(class_name, "__init__", instance.clone(), evaluated_args);
                }
                
                return Some(instance);
            }
        }
        
        eprintln!("Error: Could not instantiate class '{}'", class_name);
        None
    }

    
    fn call_method(&mut self, class_name: &str, method_name: &str, instance: DataHolder, args: Vec<DataHolder>) -> Option<DataHolder> {
        
        if let Some(class_def) = self.environment.get_class(class_name).cloned() {
            if let Statement::ClassMeta { fields, .. } = class_def {
                
                if let Some(Statement::FunctionDeclaration { name, params, body }) = fields.get(method_name) {
                    
                    let method_params = params.clone();
                    let method_body = body.clone();
                    
                    
                    let old_context = self.method_context.take();
                    self.method_context = Some(MethodContext {
                        instance: instance.clone(),
                    });
                    
                    
                    let mut method_env = Environment::new();
                    
                    
                    let current_vars: Vec<(String, DataHolder)> = self.environment
                        .get_all_variables()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();
                
                for (key, value) in current_vars {
                    method_env.set_variable(key, value);
                }
                
                
                method_env.set_variable("self".to_string(), instance);
                
                
                let non_self_params: Vec<_> = method_params.iter()
                    .filter(|param| param.name != "self")
                    .collect();
                
                
                if args.len() != non_self_params.len() {
                    eprintln!("Method '{}' expects {} arguments, got {}", method_name, non_self_params.len(), args.len());
                    self.method_context = old_context;
                    return None;
                }
                
                
                for (param, arg) in non_self_params.iter().zip(args.iter()) {
                    method_env.set_variable(param.name.clone(), arg.clone());
                }
                
                
                let old_env = std::mem::replace(&mut self.environment, method_env);
                let old_returning = self.returning;
                let old_return_value = self.return_value.clone();
                
                
                self.returning = false;
                self.return_value = None;
                
                
                for statement in method_body.iter() {
                    let result = self.execute_statement(statement.clone());
                    if matches!(result, ExecutionResult::Return(_)) {
                        break;
                    }
                }
                
                
                let return_val = self.return_value.clone().unwrap_or(DataHolder::INTEGER32(0));
                
                
                self.environment = old_env;
                self.returning = old_returning;
                self.return_value = old_return_value;
                self.method_context = old_context;
                
                return Some(return_val);
            }
        }
    }
    
    eprintln!("Error: Method '{}' not found in class '{}'", method_name, class_name);
    None
}
    
    fn get_default_value(&self, data_type: &Types) -> DataHolder {
        match data_type {
            Types::INTEGER32 => DataHolder::INTEGER32(0),
            Types::INTEGER64 => DataHolder::INTEGER64(0),
            Types::FLOAT32 => DataHolder::FLOAT32(0.0),
            Types::FLOAT64 => DataHolder::FLOAT64(0.0),
            Types::BOOLEAN => DataHolder::BOOLEAN(false),
            Types::STRING => DataHolder::STRING(String::new()),
            Types::LIST => DataHolder::LIST(Vec::new()),
            _ => DataHolder::INTEGER32(0),
        }
    }

    
    fn perform_arithmetic_operation(&self, left: &DataHolder, operator: &crate::tokenizer::ArithmeticOperator, right: &DataHolder) -> Option<DataHolder> {
        match operator {
            crate::tokenizer::ArithmeticOperator::Add => {
                match (left, right) {
                    (DataHolder::INTEGER32(a), DataHolder::INTEGER32(b)) => Some(DataHolder::INTEGER32(a + b)),
                    (DataHolder::INTEGER64(a), DataHolder::INTEGER64(b)) => Some(DataHolder::INTEGER64(a + b)),
                    (DataHolder::FLOAT32(a), DataHolder::FLOAT32(b)) => Some(DataHolder::FLOAT32(a + b)),
                    (DataHolder::FLOAT64(a), DataHolder::FLOAT64(b)) => Some(DataHolder::FLOAT64(a + b)),
                    (DataHolder::STRING(a), DataHolder::STRING(b)) => Some(DataHolder::STRING(format!("{}{}", a, b))),
                    _ => None,
                }
            },
            crate::tokenizer::ArithmeticOperator::Subtract => {
                match (left, right) {
                    (DataHolder::INTEGER32(a), DataHolder::INTEGER32(b)) => Some(DataHolder::INTEGER32(a - b)),
                    (DataHolder::INTEGER64(a), DataHolder::INTEGER64(b)) => Some(DataHolder::INTEGER64(a - b)),
                    (DataHolder::FLOAT32(a), DataHolder::FLOAT32(b)) => Some(DataHolder::FLOAT32(a - b)),
                    (DataHolder::FLOAT64(a), DataHolder::FLOAT64(b)) => Some(DataHolder::FLOAT64(a - b)),
                    _ => None,
                }
            },
            crate::tokenizer::ArithmeticOperator::Multiply => {
                match (left, right) {
                    (DataHolder::INTEGER32(a), DataHolder::INTEGER32(b)) => Some(DataHolder::INTEGER32(a * b)),
                    (DataHolder::INTEGER64(a), DataHolder::INTEGER64(b)) => Some(DataHolder::INTEGER64(a * b)),
                    (DataHolder::FLOAT32(a), DataHolder::FLOAT32(b)) => Some(DataHolder::FLOAT32(a * b)),
                    (DataHolder::FLOAT64(a), DataHolder::FLOAT64(b)) => Some(DataHolder::FLOAT64(a * b)),
                    _ => None,
                }
            },
            crate::tokenizer::ArithmeticOperator::Divide => {
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
            crate::tokenizer::ArithmeticOperator::Modulo => {
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
            crate::tokenizer::ArithmeticOperator::Not => {
                
                None
            },
        }
    }
    
    fn perform_unary_operation(&self, operator: &crate::tokenizer::ArithmeticOperator, operand: &DataHolder) -> Option<DataHolder> {
        match operator {
            crate::tokenizer::ArithmeticOperator::Subtract => {
                match operand {
                    DataHolder::INTEGER32(n) => Some(DataHolder::INTEGER32(-n)),
                    DataHolder::INTEGER64(n) => Some(DataHolder::INTEGER64(-n)),
                    DataHolder::FLOAT32(n) => Some(DataHolder::FLOAT32(-n)),
                    DataHolder::FLOAT64(n) => Some(DataHolder::FLOAT64(-n)),
                    _ => None,
                }
            },
            crate::tokenizer::ArithmeticOperator::Add => Some(operand.clone()),
            
            crate::tokenizer::ArithmeticOperator::Not => {
                match operand {
                    DataHolder::BOOLEAN(b) => Some(DataHolder::BOOLEAN(!b)),
                    DataHolder::INTEGER32(i) => Some(DataHolder::BOOLEAN(*i == 0)),
                    DataHolder::INTEGER64(i) => Some(DataHolder::BOOLEAN(*i == 0)),
                    _ => None,
                }
            },
            _ => None, 
        }
    }
    
    fn perform_comparison_operation(&self, left: &DataHolder, operator: &crate::tokenizer::ComparisonOperator, right: &DataHolder) -> Option<DataHolder> {
        match operator {
            crate::tokenizer::ComparisonOperator::Equal => {
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
            crate::tokenizer::ComparisonOperator::NotEqual => {
                if let Some(DataHolder::BOOLEAN(result)) = self.perform_comparison_operation(left, &crate::tokenizer::ComparisonOperator::Equal, right) {
                    Some(DataHolder::BOOLEAN(!result))
                } else {
                    None
                }
            },
            crate::tokenizer::ComparisonOperator::Greater => {
                match (left, right) {
                    (DataHolder::INTEGER32(a), DataHolder::INTEGER32(b)) => Some(DataHolder::BOOLEAN(a > b)),
                    (DataHolder::INTEGER64(a), DataHolder::INTEGER64(b)) => Some(DataHolder::BOOLEAN(a > b)),
                    (DataHolder::FLOAT32(a), DataHolder::FLOAT32(b)) => Some(DataHolder::BOOLEAN(a > b)),
                    (DataHolder::FLOAT64(a), DataHolder::FLOAT64(b)) => Some(DataHolder::BOOLEAN(a > b)),
                    _ => None,
                }
            },
            crate::tokenizer::ComparisonOperator::Less => {
                match (left, right) {
                    (DataHolder::INTEGER32(a), DataHolder::INTEGER32(b)) => Some(DataHolder::BOOLEAN(a < b)),
                    (DataHolder::INTEGER64(a), DataHolder::INTEGER64(b)) => Some(DataHolder::BOOLEAN(a < b)),
                    (DataHolder::FLOAT32(a), DataHolder::FLOAT32(b)) => Some(DataHolder::BOOLEAN(a < b)),
                    (DataHolder::FLOAT64(a), DataHolder::FLOAT64(b)) => Some(DataHolder::BOOLEAN(a < b)),
                    _ => None,
                }
            },
            crate::tokenizer::ComparisonOperator::GreaterEqual => {
                if let Some(DataHolder::BOOLEAN(less_result)) = self.perform_comparison_operation(left, &crate::tokenizer::ComparisonOperator::Less, right) {
                    Some(DataHolder::BOOLEAN(!less_result))
                } else {
                    None
                }
            },
            crate::tokenizer::ComparisonOperator::LessEqual => {
                if let Some(DataHolder::BOOLEAN(greater_result)) = self.perform_comparison_operation(left, &crate::tokenizer::ComparisonOperator::Greater, right) {
                    Some(DataHolder::BOOLEAN(!greater_result))
                } else {
                    None
                }
            },
        }
    }
    
    pub fn call_function(&mut self, func_name: &str, args: Vec<DataHolder>) -> Option<DataHolder> {
    if let Some(function) = self.functions.get(func_name).cloned() {
        let mut function_env = Environment::new();
        
        
        for (key, value) in self.environment.get_all_variables() {
            function_env.set_variable(key.clone(), value.clone());
        }
        
        
        let non_self_params: Vec<_> = function.params.iter()
            .filter(|param| param.name != "self")
            .collect();
        
        if args.len() != non_self_params.len() {
            eprintln!("Function '{}' expects {} arguments, got {}", func_name, non_self_params.len(), args.len());
            return None;
        }
        
        
        for (param, arg) in non_self_params.iter().zip(args.iter()) {
            function_env.set_variable(param.name.clone(), arg.clone());
        }
        
        let old_env = std::mem::replace(&mut self.environment, function_env);
        let old_returning = self.returning;
        let old_return_value = self.return_value.clone();
        
        self.returning = false;
        self.return_value = None;
        
        for statement in function.body {
            let result = self.execute_statement(statement);
            if matches!(result, ExecutionResult::Return(_)) {
                break;
            }
        }
        
        let return_val = self.return_value.clone().unwrap_or(DataHolder::INTEGER32(0));
        
        self.environment = old_env;
        self.returning = old_returning;
        self.return_value = old_return_value;
        
        Some(return_val)
    } else {
        self.execute_builtin_function(func_name, args)
    }
}
    
    fn execute_builtin_function(&self, func_name: &str, args: Vec<DataHolder>) -> Option<DataHolder> {
        
        if let Ok(functions) = get_built_in_functions().lock() {
            functions.call(func_name, args)
        } else {
            eprintln!("Error: Could not access built-in functions");
            None
        }
    }
    
    pub fn get_environment(&self) -> &Environment {
        &self.environment
    }
    
    pub fn get_environment_mut(&mut self) -> &mut Environment {
        &mut self.environment
    }
}