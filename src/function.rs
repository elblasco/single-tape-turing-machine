use regex::Regex;
use std::collections::HashMap;
use std::fmt;

use crate::MyErrors;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

pub struct Function {
    function: std::collections::HashMap<(String, char), (char, String, Direction)>,
}

impl Function {
    pub fn new(input_path: String) -> Result<Self, MyErrors> {
        let input_file_stringified = std::fs::read_to_string(input_path);

        if input_file_stringified.is_err() {
            return Err(MyErrors::OpeningFile);
        }

        let mut function = Function {
            function: HashMap::new(),
        };

        let formatted_input = Function::parse_input(input_file_stringified.unwrap());
        for row in formatted_input {
            let divided_row: Vec<&str> = row.split(' ').collect();
            function.add(divided_row);
        }

        Ok(function)
    }

    fn add(&mut self, divided_row: Vec<&str>) {
        let dir = if divided_row[3] == "r" {
            Direction::Right
        } else {
            Direction::Left
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

    fn parse_input(input_file: String) -> Vec<String> {
        let remove_comments_regex: Regex = Regex::new(r"\s*;.*").unwrap();

        let input_matrix: Vec<&str> = input_file.split('\n').collect();
        let mut input_matrix_string: Vec<String> = vec![];
        for element_str in input_matrix {
            input_matrix_string.push(element_str.to_string());
        }

        // Remove comments after a line of code
        for elem in input_matrix_string.iter_mut() {
            let found = remove_comments_regex.find(elem);
            if let Some(match_struct) = found {
                elem.drain(match_struct.range());
            }
        }

        // Remove commet lines
        input_matrix_string.retain(|row| !row.is_empty() && !remove_comments_regex.is_match(row));
        input_matrix_string
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
