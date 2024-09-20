use crate::function::Direction;
use std::fmt;
use std::ops::{AddAssign, SubAssign};

pub struct Tape {
    cells: Vec<char>,
    current_index: usize,
}

impl Tape {
    pub fn new(input: String) -> Self {
        let mut tape = Tape {
            cells: Vec::new(),
            current_index: 0,
        };

        for character in input.chars() {
            tape.cells.push(character)
        }

        tape
    }

    pub fn do_a_step(&mut self, new_symbol: char, direction: Direction) {
        self.cells.insert(self.current_index, new_symbol);
        match direction {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    fn move_left(&mut self) {
        if self.current_index == 0 {
            self.cells.insert(0, ' ');
        } else {
            self.current_index.sub_assign(1);
        }
    }

    fn move_right(&mut self) {
        if self.current_index + 1 == self.cells.len() {
            self.cells.push(' ');
        }
        self.current_index.add_assign(1);
    }
}

impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string_buf: String = format!("{:?}", self.cells);
        string_buf.push('\n');

        write!(f, "{}", string_buf)
    }
}
