use std::time::Instant;

mod tm_engine;

fn main() {
    let machine = tm_engine::deterministic::TuringMachine {
        states: vec![
            tm_engine::State("start".to_string()),
            tm_engine::State("accept".to_string()),
            tm_engine::State("reject".to_string()),
            tm_engine::State("iterate_to_end".to_string()),
            tm_engine::State("last_symbol_check".to_string()),
        ],
        transitions: vec![
            vec![
                tm_engine::Transition(3, 0, 1),
                tm_engine::Transition(3, 1, 1),
                tm_engine::Transition(3, 2, 1),
                tm_engine::Transition(3, 3, 1),
            ],
            vec![], // accept transitions
            vec![], // reject transitions
            vec![
                tm_engine::Transition(4, 0, -1),
                tm_engine::Transition(3, 1, 1),
                tm_engine::Transition(3, 2, 1),
                tm_engine::Transition(3, 3, 1),
            ],
            vec![
                tm_engine::Transition(2, 0, 1),
                tm_engine::Transition(2, 1, 1),
                tm_engine::Transition(1, 2, 1), // accept if last symbol is 0
                tm_engine::Transition(2, 3, 1), // reject if last symbol is 1
            ],
        ],
        ..tm_engine::deterministic::TuringMachine::default()
    };

    for i in (0..1_000_000_0).step_by(1_000) {
        let num_instructions = i;
        let mut word = "".to_string();
        for _ in 0..num_instructions {
            word.push_str("");
        }
        word.push_str("0");

        let mut computation = match machine.compute(&word) {
            Ok(val) => val,
            Err(_) => panic!("Is broken"),
        };

        let start = Instant::now();
        computation.run();
        let elapsed = start.elapsed();
        let elapsed_seconds = elapsed.as_secs_f64();

        print!("\rPerformance (instructions / second): 1e{} ", (num_instructions as f64 / elapsed_seconds).log10()); 
    }

}
