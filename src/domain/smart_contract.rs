use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmartContract {
    pub code: String,
    pub state: HashMap<String, String>,
}

impl SmartContract {
    pub fn new(code: String) -> Self {
        Self {
            code,
            state: HashMap::new(),
        }
    }

    pub fn execute(&mut self, function: &str, args: Vec<String>) -> Option<String> {
        match function {
            "set" => {
                if args.len() == 2 {
                    self.state.insert(args[0].clone(), args[1].clone());
                    Some("OK".to_string())
                } else {
                    None
                }
            }
            "get" => self.state.get(&args[0]).cloned(),
            _ => None,
        }
    }
}
