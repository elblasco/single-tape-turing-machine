use regex::Regex;

#[derive(Debug)]
pub enum MyErrors {
    NoFileProvided,
    OpeningFile,
    Parsing,
}

fn main() -> Result<(), MyErrors> {
    let input_file = std::env::args().nth(1);

    if input_file.is_none() {
        return Err(MyErrors::NoFileProvided);
    }

    let input_file = std::fs::read_to_string(input_file.unwrap());

    if input_file.is_err() {
        return Err(MyErrors::OpeningFile);
    }

    let stringified_input: String = input_file.unwrap();

    let parsed_input: Vec<String> = parse_input(stringified_input);

    Ok(())
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
