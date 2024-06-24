# Turing Machiine Script

A programming language-esque scripting tool to execute a Turing Machine.

[Project Brief](./TM_Project_Requirements.pdf)

## Usage

```bash
tms-interpreter file.tms "input_word"
tms-interpreter file.tms -t "TEST_IDENTIFIER"
tms-interpreter file.tms -s "SUITE_IDENTIFIER"
tms-interpreter file.tms -f file.test.tms
```

### Input

* `file.tms` - File to be executed - Must have tms extension
* Input(test / word) - see below options

### Options

* `-f` `--file` specify a test file, will override any tests in `file.tms`
* `-t` `--test` Specify a test to run in `file.tms` or `-f`
* `-ta` `--test-all` run all tests in `file.tms` or `-f`
* `-s` `--suite` specify a suite of tests in `file.tms` or `-f`
* `-sa` `--suite-all` run all suites in `file.tms` or `-f`
* `-nr` `--no-run` check for errors
* `--step` print out each step of the execution
* `-d` `--debug` run the machine in debug mode

### Commands

* `view` - list out information (unknown what so far) about the TM
* `generate` - create a set of testcases by artificially stepping through the TM

## Syntax

```c
    // This is a comment
    meta {
        title: ""; // optional - defaults to empty_string
        language_description: ""; // optional - defaults to empty_string
        stay_direction: false; // optional - defaults to false
        repeated_direction: false; // optional - defaults to false
        strict: true;// optional - defaults to true
    }

    // Symbols and states maybe any utf-32 string 
    states: [ start_state, accept_state, reject_state, state_1, state_2, ... ];
    // Will always have blank_symbol or end_of_tape_symbol: They can just be modified to something else here
    tape_symbols: [ blank_symbol, end_of_tape_symbol, seperator_symbol, ... ];
    // need at least one symbol
    language_symbols: [ symbol_1, symbol_2, ... ];

    state,symbol,state,symbol,direction; // This is a transition
    // In strict mode, any missing symbol state pairs will throw an error


    test TEST_IDENTIFIER(OPTIONAL) {
        input: <input_word>;
        expected: <output_word>;
        max_timestep: <unsigned number>;
        max_cells: <unsigned number>;
    }

    suite SUITE_IDENTIFIER {
        test_count: 0;
        test_weights: [ 0.1, 0.1, ... ];

        test TEST_IDENTIFIER(OPTIONAL) {
            input: <input_word>;
            expected: <output_word>;
            max_timestep: <unsigned number>;
            max_cells: <unsigned number>;
        }

        test TEST_IDENTIFIER(OPTIONAL) {
            input: <input_word>;
            expected: <output_word>;
            max_timestep: <unsigned number>;
            max_cells: <unsigned number>;
        }
    }
```
