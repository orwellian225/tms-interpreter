use core::fmt::{Formatter, Display};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct State(pub String);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(pub String);
#[derive(Debug, PartialEq, Eq)]
pub struct TransitionResult(pub usize, pub usize, pub i32);

#[derive(Debug)]
pub struct TuringMachine {
    pub states: Vec<State>,
    pub tape_symbols: Vec<Symbol>,
    pub language_symbols: Vec<Symbol>,
    pub transitions: Vec<Vec<TransitionResult>>,
    pub start_state: usize,
    pub accept_state: usize,
    pub reject_state: usize,
}

impl TuringMachine {
    pub fn compute(&self, word: &String) -> Result<TMExecution, ()> {
        self.bounded_compute(word, (None, None))
    }

    pub fn bounded_compute(&self, word: &String, limits: (Option<usize>, Option<usize>)) -> Result<TMExecution, ()> {
        let mut tape = vec![1];

        for w in word.chars() {
            let symbol = Symbol(w.to_string());
            let index = match self.language_symbols.iter().position(|x| { x == &symbol }) {
                Some(val) => val + self.tape_symbols.len(),
                None => match self.tape_symbols.iter().position(|x| { x == &symbol }) {
                    Some(val) => val,
                    None => return Err(todo!())
                }
            };

            tape.push(index);
        }

        Ok(TMExecution {
            machine: &self,
            current_state: self.start_state,
            head_position: 0,
            status: TMStatus::Executing,
            clock: TMClock {
                time: 0,
                time_limit: limits.0,
                space: tape.len(),
                space_limit: limits.1
            },
            tape
        })
    }
}

impl Default for TuringMachine {
    fn default() -> Self {
        Self {
            states: vec![
                State("q0".to_string()),
                State("q1".to_string()), 
                State("q2".to_string())
            ],
            language_symbols: vec![
                Symbol("0".to_string()), 
                Symbol("1".to_string())
            ],
            tape_symbols: vec![
                Symbol("_".to_string()), 
                Symbol(">".to_string()),
            ],
            transitions: vec![
                vec![
                    TransitionResult(1, 0, 1),
                    TransitionResult(1, 1, 1),
                    TransitionResult(1, 2, 1),
                    TransitionResult(1, 3, 1),
                ]
            ],
            start_state: 0,
            accept_state: 1,
            reject_state: 2,
        }
    }
}

#[derive(Debug)]
pub struct TMClock {
    time_limit: Option<usize>,
    time: usize,
    space_limit: Option<usize>,
    space: usize
}

impl Default for TMClock {
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
pub enum TMStatus {
    #[default]
    Executing,
    Accept,
    Reject,
    Timeout,
    Spaceout
}

#[derive(Debug)]
pub struct TMExecution<'a> {
    machine: &'a TuringMachine,
    current_state: usize,
    head_position: usize,
    tape: Vec<usize>,
    pub status: TMStatus,
    clock: TMClock,
}

impl TMExecution<'_> {

    pub fn compute(&mut self) {
        while self.status == TMStatus::Executing {
            self.step();
        }
    }

    pub fn step(&mut self) {
        match self.status {
            TMStatus::Executing => (),
            _ => return
        }

        self.clock.time += 1;
        match self.clock.time_limit {
            Some(limit) => if self.clock.time >= limit {
                self.status = TMStatus::Timeout;
                return;
            },
            None => ()
        }

        let read_symbol = self.tape[self.head_position];
        let TransitionResult(next_state, write_symbol, direction) = self.machine.transitions[self.current_state][read_symbol];

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
                    self.status = TMStatus::Spaceout;
                    return;
                },
                None => ()
            }
        }

        if next_state == self.machine.accept_state {
            self.status = TMStatus::Accept;
            return;
        } else if next_state == self.machine.reject_state {
            self.status = TMStatus::Reject;
            return;
        }

    }

}

impl Display for TMExecution<'_> {

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
