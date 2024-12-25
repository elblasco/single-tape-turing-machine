use regex::Regex;
use std::collections::HashMap;
use std::fmt;

use crate::MyErrors;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    NotMove,
}

pub type Key = (String, char);
pub type Val = (char, String, Direction);

pub struct Function {
    function: std::collections::HashMap<Key, Val>,
}

impl Function {
    pub fn new(input_path: &str) -> Result<Self, MyErrors> {
        let input_file_stringified = std::fs::read_to_string(input_path);

        if input_file_stringified.is_err() {
            return Err(MyErrors::OpeningFile);
        }

        let mut function = Function {
            function: HashMap::new(),
        };

        let binding = input_file_stringified.unwrap();
        let formatted_input = Function::parse_input(&binding);
        for row in formatted_input {
            let divided_row: Vec<&str> = row.split(' ').collect();
            function.add(divided_row);
        }

        Ok(function)
    }

    pub fn compute(&self, current_state: &str, current_symbol: char) -> Result<&Val, MyErrors> {
        let lookup_tuple = if current_symbol.is_whitespace() {
            (current_state.to_string(), '_')
        } else {
            (current_state.to_string(), current_symbol)
        };
        match (
            self.function.get(&lookup_tuple),
            self.function.get(&(current_state.to_string(), '*')),
        ) {
            (Some(result), _) | (None, Some(result)) => Ok(result),
            _ => Err(MyErrors::StateNotFound),
        }
    }

    //TODO handle not well formatted direction, e.g. 1o _ 1 invalid 4

    fn add(&mut self, divided_row: Vec<&str>) {
        let dir = match divided_row[3] {
            "r" => Direction::Right,
            "l" => Direction::Left,
            "*" => Direction::NotMove,
            _ => Direction::NotMove,
        };
        self.function.insert(
            (
                divided_row[0].to_string(),
                divided_row[1].parse::<char>().unwrap(),
            ),
            (
                divided_row[2].parse::<char>().unwrap(),
                divided_row[4].to_string(),
                dir,
            ),
        );
    }

    fn parse_input(input_file: &str) -> Vec<&str> {
        let line_comments_regex: Regex = Regex::new(r";.*").unwrap();

        let mut input_matrix: Vec<&str> = input_file.split('\n').collect();

        // Drains all the lines that starts with a comment
        input_matrix.retain(|elem| {
            let matchy = line_comments_regex.find(elem);
            matchy.is_none_or(|matchy_some| matchy_some.range().start != 0)
        });

        // Trims all the lines with inline comments
        for elem in input_matrix.iter_mut() {
            if let Some((prefix, _)) = elem.split_once(';') {
                *elem = prefix.trim_end();
            }
        }

        //Remove all the now-empty lines
        input_matrix.retain(|elem| !elem.is_empty());
        input_matrix
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string_buf: String = String::new();
        for elem in self.function.iter() {
            string_buf.push_str(format!("{:?}", elem).as_str());
            string_buf.push('\n');
        }
        write!(f, "{}", string_buf)
    }
}
