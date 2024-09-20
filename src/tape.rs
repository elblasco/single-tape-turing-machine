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

    pub fn write_and_move(&mut self, new_symbol: char, direction: Direction) {
        self.cells[self.current_index] = new_symbol;
        match direction {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::NotMove => (),
        }
    }

    pub fn get_current_value(&self) -> char {
        *self.cells.get(self.current_index).unwrap()
    }

    fn move_left(&mut self) {
        if self.current_index == 0 {
            self.cells.insert(0, '_');
        } else {
            self.current_index.sub_assign(1);
        }
    }

    fn move_right(&mut self) {
        if self.current_index + 1 == self.cells.len() {
            self.cells.push('_');
        }
        self.current_index.add_assign(1);
    }
}

impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string_buf: String = String::new();

        for chara in self.cells.iter() {
            string_buf.push(*chara);
        }

        string_buf.push('\n');

        for _ in 0..self.current_index {
            string_buf.push(' ');
        }

        string_buf.push('^');

        write!(f, "{}", string_buf)
    }
}
