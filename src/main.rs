mod function;
mod machine;
mod tape;

use function::Function;
use machine::TuringMachine;
use tape::Tape;

#[derive(Debug)]
pub enum MyErrors {
    NoFileProvided,
    OpeningFile,
    Parsing,
    BadInput,
    StateNotFound,
}

fn main() -> Result<(), MyErrors> {
    let input_path = std::env::args().nth(1);

    if input_path.is_none() {
        return Err(MyErrors::NoFileProvided);
    }

    let transition_function = Function::new(input_path.unwrap())?;

    println!("{transition_function}");

    let initial_tape_content: String = user_input("Insert tape initial content")?;

    let tape = Tape::new(initial_tape_content);

    let initial_state: String = user_input("Insert inital state name")?;

    println!("The initial state is {initial_state}");

    let mut tm = TuringMachine::new(tape, transition_function, initial_state);

    tm.execute()?;

    Ok(())
}

fn user_input(output_string: &str) -> Result<String, MyErrors> {
    println!("{}", output_string);

    let mut user_input = String::new();

    if std::io::stdin().read_line(&mut user_input).is_err() {
        Err(MyErrors::BadInput)
    } else {
        user_input.pop();
        Ok(user_input)
    }
}
