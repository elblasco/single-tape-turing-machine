use regex::Regex;

use crate::function::Function;
use crate::tape::Tape;
use crate::MyErrors;

pub struct TuringMachine {
    tape: Tape,
    transition_function: Function,
    current_state: String,
}

impl TuringMachine {
    pub fn new(tape: Tape, transition_function: Function, starting_state: String) -> Self {
        TuringMachine {
            tape,
            transition_function,
            current_state: starting_state,
        }
    }

    pub fn get_config(&self) -> (Vec<char>, usize) {
        (self.tape.get_tape_content(), self.tape.get_current_index())
    }

    pub fn step(&mut self) -> Result<(), MyErrors> {
        let halting_state: Regex = Regex::new(r"halt.*").unwrap();
        if let Ok(new_phase) = self
            .transition_function
            .compute(&self.current_state, self.tape.get_current_value())
        {
            if halting_state.is_match(&new_phase.1) {
                return Ok(());
            }
            self.current_state = new_phase.1.to_owned();
            self.tape.write_and_move(new_phase.0, new_phase.2);
            return Ok(());
        }
        Err(MyErrors::StateNotFound)
    }

    pub fn execute(&mut self) -> Result<(), MyErrors> {
        let halting_state: Regex = Regex::new(r"halt.*").unwrap();
        while let Ok(new_phase) = self
            .transition_function
            .compute(&self.current_state, self.tape.get_current_value())
        {
            if halting_state.is_match(&new_phase.1) {
                return Ok(());
            }
            self.current_state = new_phase.1.to_owned();
            self.tape.write_and_move(new_phase.0, new_phase.2);
        }
        Err(MyErrors::StateNotFound)
    }
}
