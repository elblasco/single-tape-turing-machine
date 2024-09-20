use crate::function::Function;
use crate::tape::Tape;

pub struct TuringMachine {
    tape: Tape,
    transition_function: Function,
    current_state: char,
}

impl TuringMachine {
    pub fn new(tape: Tape, transition_function: Function, starting_state: char) -> Self {
        TuringMachine {
            tape,
            transition_function,
            current_state: starting_state,
        }
    }
}
