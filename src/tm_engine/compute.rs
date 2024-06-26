use core::fmt::{Formatter, Display};
use std::rc::Rc;

use super::deterministic::TuringMachine;
use super::{State, Symbol, Transition};

#[derive(Debug)]
pub struct CompClock {
    time_limit: Option<usize>,
    time: usize,
    space_limit: Option<usize>,
    space: usize
}

impl Default for CompClock {
    fn default() -> Self {
        return Self {
            time_limit: None,
            time: 0,
            space_limit: None,
            space: 1
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum CompStatus {
    #[default]
    Executing,
    Accept,
    Reject,
    Timeout,
    Spaceout
}

#[derive(Debug)]
pub struct Computation {
    pub machine: Rc<TuringMachine>,
    pub current_state: usize,
    pub head_position: usize,
    pub tape: Vec<usize>,
    pub status: CompStatus,
    pub clock: CompClock,
}

impl Computation {

    pub fn start(machine: Rc<TuringMachine>, word: &String, limits: (Option<usize>, Option<usize>)) -> Result<Self, ()> {
        let mut tape = vec![1];

        for w in word.chars() {
            let symbol = Symbol(w.to_string());
            let index = match machine.language_symbols.iter().position(|x| { x == &symbol }) {
                Some(val) => val + machine.tape_symbols.len(),
                None => match machine.tape_symbols.iter().position(|x| { x == &symbol }) {
                    Some(val) => val,
                    None => return Err(todo!())
                }
            };

            tape.push(index);
        }

        Ok(Computation {
            machine,
            current_state: machine.start_state,
            head_position: 0,
            status: CompStatus::Executing,
            clock: CompClock {
                time: 0,
                time_limit: limits.0,
                space: tape.len(),
                space_limit: limits.1
            },
            tape
        })
    }

    pub fn run(&mut self) {
        while self.status == CompStatus::Executing {
            self.step();
        }
    }

    pub fn step(&mut self) {
        match self.status {
            CompStatus::Executing => (),
            _ => return
        }

        self.clock.time += 1;
        match self.clock.time_limit {
            Some(limit) => if self.clock.time >= limit {
                self.status = CompStatus::Timeout;
                return;
            },
            None => ()
        }

        let read_symbol = self.tape[self.head_position];
        let Transition(next_state, write_symbol, direction) = self.machine.transitions[self.current_state][read_symbol];

        let new_head_position = match direction.is_negative() {
            false => match self.head_position.checked_add(direction as usize) {
                Some(val) => val,
                None => panic!("Head position exceded max usize")
            },
            true => match self.head_position.checked_sub(direction.wrapping_abs() as usize) {
                Some(val) => val,
                None => 0
            }
        };

        self.current_state = next_state;
        self.tape[self.head_position] = write_symbol;
        self.head_position = new_head_position;

        if self.head_position >= self.tape.len() {
            self.tape.push(0 as usize);
            self.clock.space += 1;

            match self.clock.space_limit {
                Some(limit) => if self.clock.space >= limit {
                    self.status = CompStatus::Spaceout;
                    return;
                },
                None => ()
            }
        }

        if next_state == self.machine.accept_state {
            self.status = CompStatus::Accept;
            return;
        } else if next_state == self.machine.reject_state {
            self.status = CompStatus::Reject;
            return;
        }

    }

}

impl Display for Computation {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {

        let mut config_str: String = "".to_string();
        let State(state_str) = &self.machine.states[self.current_state];

        for (i, s_i) in self.tape.iter().enumerate() {

            if i == self.head_position {
                config_str.push_str(format!("({})", state_str).as_str());
            }

            if s_i < &self.machine.tape_symbols.len() {
                let Symbol(s) = &self.machine.tape_symbols[*s_i];
                config_str.push_str(s);
            } else if 1 < *s_i && *s_i < self.machine.language_symbols.len() + 2 {
                let Symbol(s) = &self.machine.language_symbols[*s_i - 2];
                config_str.push_str(s);
            }
        }


        write!(f, "{}", config_str)
    }

}