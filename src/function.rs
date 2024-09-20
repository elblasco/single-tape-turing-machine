use std::collections::HashMap;

enum Direction {
    Left,
    Right,
}

pub struct Function {
    function: std::collections::HashMap<(String, String), (String, String, Direction)>,
}

impl Function {
    pub fn new() -> Self {
        Function {
            function: HashMap::new(),
        }
    }

    pub fn from_parsed_input(input: Vec<String>) -> Self {
        Function::new()
    }
}
