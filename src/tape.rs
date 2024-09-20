use crate::function::Direction;
use std::ops::{AddAssign, SubAssign};

pub struct Tape {
    cells: Vec<char>,
    current_index: usize,
}

impl Tape {
    pub fn new() -> Self {
        Tape {
            cells: Vec::new(),
            current_index: 0,
        }
    }

    pub fn init_fill(&mut self, input: String) {
        for character in input.chars() {
            self.cells.push(character)
        }
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
