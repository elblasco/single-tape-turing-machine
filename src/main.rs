mod function;
mod machine;
mod tape;

use function::Function;

use crate::machine::TuringMachine;

#[derive(Debug)]
pub enum MyErrors {
    NoFileProvided,
    OpeningFile,
    Parsing,
}

fn main() -> Result<(), MyErrors> {
    let input_path = std::env::args().nth(1);

    if input_path.is_none() {
        return Err(MyErrors::NoFileProvided);
    }

    let transition_function = Function::new(input_path.unwrap())?;

    println!("{}", transition_function);

    Ok(())
}
