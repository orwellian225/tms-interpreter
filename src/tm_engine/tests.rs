use super::*;

#[test]
fn default_tm() {
    let machine = TuringMachine::default();

    for i in 0..3 {
        assert_eq!(machine.states[i], State(format!("q{i}")));
    }

    assert_eq!(machine.tape_symbols[0], Symbol("_".to_string()));
    assert_eq!(machine.tape_symbols[1], Symbol(">".to_string()));
    assert_eq!(machine.language_symbols[0], Symbol("0".to_string()));
    assert_eq!(machine.language_symbols[1], Symbol("1".to_string()));

    assert_eq!(machine.start_state, 0);
    assert_eq!(machine.accept_state, 1);
    assert_eq!(machine.reject_state, 2);

    assert_eq!(machine.transitions[0][0], Transition(1, 0, 1));
    assert_eq!(machine.transitions[0][1], Transition(1, 1, 1));
    assert_eq!(machine.transitions[0][2], Transition(1, 2, 1));
    assert_eq!(machine.transitions[0][3], Transition(1, 3, 1));
}

#[test]
fn default_tm_step() {
    let mut execution = TMExecution {
        machine: &TuringMachine::default(),
        current_state: 0,
        head_position: 0,
        tape: vec![1],
        status: TMStatus::Executing,
        clock: TMClock::default()
    };

    execution.step();

    assert_eq!(execution.status, TMStatus::Accept);
}

#[test]
fn last_symbol_accept() {
    let machine = TuringMachine {
        states: vec![
            State("start".to_string()),
            State("accept".to_string()),
            State("reject".to_string()),
            State("iterate_to_end".to_string()),
            State("last_symbol_check".to_string()),
        ],
        transitions: vec![
            vec![
                Transition(3, 0, 1),
                Transition(3, 1, 1),
                Transition(3, 2, 1),
                Transition(3, 3, 1),
            ],
            vec![], // accept transitions
            vec![], // reject transitions
            vec![
                Transition(4, 0, -1),
                Transition(3, 1, 1),
                Transition(3, 2, 1),
                Transition(3, 3, 1),
            ],
            vec![
                Transition(2, 0, 1),
                Transition(2, 1, 1),
                Transition(1, 2, 1), // accept if last symbol is 0
                Transition(2, 3, 1), // reject if last symbol is 1
            ],
        ],
        ..TuringMachine::default()
    };

    let accepted_computation_result = machine.compute(&"1110".to_string());

    let mut accept_computation = match accepted_computation_result {
        Ok(computation) => computation,
        Err(_) => panic!("Error in accepted computation")
    };

    accept_computation.compute();

    assert_eq!(accept_computation.status, TMStatus::Accept);
    assert_eq!(accept_computation.tape.len(), 6);
    assert_eq!(accept_computation.tape, vec![1, 3, 3, 3, 2, 0]);
    assert_eq!(accept_computation.head_position, 4);
    assert_eq!(accept_computation.clock.time, 7);
    assert_eq!(accept_computation.clock.space, 6);
}

#[test]
fn last_symbol_reject() {
    let machine = TuringMachine {
        states: vec![
            State("start".to_string()),
            State("accept".to_string()),
            State("reject".to_string()),
            State("iterate_to_end".to_string()),
            State("last_symbol_check".to_string()),
        ],
        transitions: vec![
            vec![
                Transition(3, 0, 1),
                Transition(3, 1, 1),
                Transition(3, 2, 1),
                Transition(3, 3, 1),
            ],
            vec![], // accept transitions
            vec![], // reject transitions
            vec![
                Transition(4, 0, -1),
                Transition(3, 1, 1),
                Transition(3, 2, 1),
                Transition(3, 3, 1),
            ],
            vec![
                Transition(2, 0, 1),
                Transition(2, 1, 1),
                Transition(1, 2, 1), // accept if last symbol is 0
                Transition(2, 3, 1), // reject if last symbol is 1
            ],
        ],
        ..TuringMachine::default()
    };

    let rejected_computation_result = machine.compute(&"0001".to_string());

    let mut rejected_computation = match rejected_computation_result {
        Ok(computation) => computation,
        Err(_) => panic!("Error in rejected computation")
    };

    rejected_computation.compute();

    assert_eq!(rejected_computation.status, TMStatus::Reject);
    assert_eq!(rejected_computation.tape.len(), 6);
    assert_eq!(rejected_computation.tape, vec![1, 2, 2, 2, 3, 0]);
    assert_eq!(rejected_computation.head_position, 4);
    assert_eq!(rejected_computation.clock.time, 7);
    assert_eq!(rejected_computation.clock.space, 6);
}

#[test]
fn last_symbol_timeout() {
    let machine = TuringMachine {
        states: vec![
            State("start".to_string()),
            State("accept".to_string()),
            State("reject".to_string()),
            State("iterate_to_end".to_string()),
            State("last_symbol_check".to_string()),
        ],
        transitions: vec![
            vec![
                Transition(3, 0, 1),
                Transition(3, 1, 1),
                Transition(3, 2, 1),
                Transition(3, 3, 1),
            ],
            vec![], // accept transitions
            vec![], // reject transitions
            vec![
                Transition(4, 0, -1),
                Transition(3, 1, 1),
                Transition(3, 2, 1),
                Transition(3, 3, 1),
            ],
            vec![
                Transition(2, 0, 1),
                Transition(2, 1, 1),
                Transition(1, 2, 1), // accept if last symbol is 0
                Transition(2, 3, 1), // reject if last symbol is 1
            ],
        ],
        ..TuringMachine::default()
    };

    let comp_result = machine.bounded_compute(&"1110".to_string(), (Some(3), None));

    let mut comp = match comp_result {
        Ok(computation) => computation,
        Err(_) => panic!("Error in accepted computation")
    };

    comp.compute();

    assert_eq!(comp.status, TMStatus::Timeout);
    assert_eq!(comp.tape.len(), 5);
    assert_eq!(comp.tape, vec![1, 3, 3, 3, 2]);
    assert_eq!(comp.head_position, 2);
    assert_eq!(comp.clock.time, 3);
    assert_eq!(comp.clock.space, 5);
}

#[test]
fn last_symbol_spaceout() {
    let machine = TuringMachine {
        states: vec![
            State("start".to_string()),
            State("accept".to_string()),
            State("reject".to_string()),
            State("iterate_to_end".to_string()),
            State("last_symbol_check".to_string()),
        ],
        transitions: vec![
            vec![
                Transition(3, 0, 1),
                Transition(3, 1, 1),
                Transition(3, 2, 1),
                Transition(3, 3, 1),
            ],
            vec![], // accept transitions
            vec![], // reject transitions
            vec![
                Transition(4, 0, -1),
                Transition(3, 1, 1),
                Transition(3, 2, 1),
                Transition(3, 3, 1),
            ],
            vec![
                Transition(2, 0, 1),
                Transition(2, 1, 1),
                Transition(1, 2, 1), // accept if last symbol is 0
                Transition(2, 3, 1), // reject if last symbol is 1
            ],
        ],
        ..TuringMachine::default()
    };

    let comp_result = machine.bounded_compute(&"1110".to_string(), (None, Some(5)));

    let mut comp = match comp_result {
        Ok(computation) => computation,
        Err(_) => panic!("Error in accepted computation")
    };

    comp.compute();

    assert_eq!(comp.status, TMStatus::Spaceout);
    assert_eq!(comp.tape.len(), 6);
    assert_eq!(comp.tape, vec![1, 3, 3, 3, 2, 0]);
    assert_eq!(comp.head_position, 5);
    assert_eq!(comp.clock.time, 5);
    assert_eq!(comp.clock.space, 6);
}
