use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;
use crate::tokenizer::DataHolder;
use crate::AstTree::Statement;

#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, DataHolder>,
    classes: HashMap<String, Statement>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
            classes: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: DataHolder) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&DataHolder> {
        self.variables.get(name)
    }

    pub fn get_all_variables(&self) -> &HashMap<String, DataHolder> {
        &self.variables
    }

    pub fn set_class(&mut self, name: String, fields: Statement) {
        self.classes.insert(name, fields);
    }

    pub fn get_class(&self, name: &str) -> Option<&Statement> {
        self.classes.get(name)
    }

    pub fn is_class_meta_exists(&self, name: &str) -> bool {
        self.classes.contains_key(name)
    }
}

static GLOBAL_ENV: OnceLock<Mutex<Environment>> = OnceLock::new();

pub fn get_global_env() -> &'static Mutex<Environment> {
    GLOBAL_ENV.get_or_init(|| Mutex::new(Environment::new()))
}
