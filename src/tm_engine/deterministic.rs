use super::{ Symbol, State, Transition };
use super::compute::Computation;

#[derive(Debug)]
pub struct TuringMachine {
    pub states: Vec<State>,
    pub tape_symbols: Vec<Symbol>,
    pub language_symbols: Vec<Symbol>,
    pub transitions: Vec<Vec<Transition>>,
    pub start_state: usize,
    pub accept_state: usize,
    pub reject_state: usize,
}

impl TuringMachine {
    pub fn compute(&self, word: &String) -> Result<Computation, ()> {
        self.bounded_compute(word, (None, None))
    }

    pub fn bounded_compute(&self, word: &String, limits: (Option<usize>, Option<usize>)) -> Result<Computation, ()> {
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
                    Transition(1, 0, 1),
                    Transition(1, 1, 1),
                    Transition(1, 2, 1),
                    Transition(1, 3, 1),
                ]
            ],
            start_state: 0,
            accept_state: 1,
            reject_state: 2,
        }
    }
}