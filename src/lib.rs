use std::collections::HashMap;

pub struct Database {
    map: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Query {
    #[allow(dead_code)] // operation field will be used...till then disabled lint
    operation: Operation,
    args: OpArgs,
}

#[derive(Debug)]
enum OpArgs {
    Pair((String, String)),
    Single(String),
    None,
}

#[derive(Debug)]
enum Operation {
    Get,
    Set,
    Exit,
}

impl Database {
    pub fn new() -> Self {
        Database {
            map: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, val: String) -> String {
        match self.map.insert(key, val) {
            Some(v) => v.to_string(),
            None => format!("New Key Added"),
        }
    }

    pub fn get(&mut self, key: String) -> String {
        match self.map.get(&key) {
            Some(v) => v.to_string(),
            None => format!("No Value found for {}", key),
        }
    }

    pub fn parse(&self, line: &String) -> Result<Query, String> {
        let line = line.trim().to_string();
        let tokens: Vec<&str> = line.split(" ").collect();
        if tokens[0] != "store" {
            return Err(format!("query should start with `store`"));
        }
        let mut args = OpArgs::None;
        let operation = match tokens[1] {
            "exit" => Operation::Exit,
            "get" => {
                args = OpArgs::Single(tokens[2].to_string());
                Operation::Get
            }
            "set" => {
                args = OpArgs::Pair((tokens[2].to_string(), tokens[3].to_string()));
                Operation::Set
            }
            _ => return Err(format!("Wrong operation")),
        };

        Ok(Query { operation, args })
    }

    pub fn exec(&mut self, query: &Query) -> Option<String> {
        match &query.args {
            OpArgs::Pair((k, v)) => Some(self.set(k.to_string(), v.to_string())),
            OpArgs::Single(k) => Some(self.get(k.to_string())),
            OpArgs::None => std::process::exit(0),
        }
    }
}
