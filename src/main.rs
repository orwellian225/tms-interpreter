use std::time::Instant;

mod tm;

fn main() {
    let machine = tm::TuringMachine {
        states: vec![
            tm::State("start".to_string()),
            tm::State("accept".to_string()),
            tm::State("reject".to_string()),
            tm::State("iterate_to_end".to_string()),
            tm::State("last_symbol_check".to_string()),
        ],
        transitions: vec![
            vec![
                tm::TransitionResult(3, 0, 1),
                tm::TransitionResult(3, 1, 1),
                tm::TransitionResult(3, 2, 1),
                tm::TransitionResult(3, 3, 1),
            ],
            vec![], // accept transitions
            vec![], // reject transitions
            vec![
                tm::TransitionResult(4, 0, -1),
                tm::TransitionResult(3, 1, 1),
                tm::TransitionResult(3, 2, 1),
                tm::TransitionResult(3, 3, 1),
            ],
            vec![
                tm::TransitionResult(2, 0, 1),
                tm::TransitionResult(2, 1, 1),
                tm::TransitionResult(1, 2, 1), // accept if last symbol is 0
                tm::TransitionResult(2, 3, 1), // reject if last symbol is 1
            ],
        ],
        ..tm::TuringMachine::default()
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
        computation.compute();
        let elapsed = start.elapsed();
        let elapsed_seconds = elapsed.as_secs_f64();

        print!("\rPerformance (instructions / second): 1e{} ", (num_instructions as f64 / elapsed_seconds).log10()); 
    }

}
