use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
pub enum FSMInput {
    Char(char),
    Epsilon
}

pub struct FSM {
    initial_state: u32,
    accept_states: Vec<u32>,
    transitions: HashMap<(u32, FSMInput), u32>
}

impl FSM {
    pub fn new() -> FSM {
        FSM {
            initial_state: 0,
            accept_states: Vec::new(),
            transitions: HashMap::new()
        }
    }

    pub fn matches(&self, input: String) -> (bool, Vec<u32>) {
        let mut states: Vec<u32> = Vec::new();
        let mut chars: Vec<char> = input.chars().rev().collect();

        let mut current = self.initial_state;
        states.push(current);
        while let Some(char) = chars.pop() { 
            match self.transitions.get(&(current, FSMInput::Char(char))) {
                Some(to) => {
                    current = *to;
                    states.push(current);
                },
                None => {
                    if let Some(to) = self.transitions.get(&(current, FSMInput::Epsilon)) {
                            current = *to;
                            states.push(current);
                            continue;
                    }

                    return (false, states)
                }
            }
        }
        
        (
            if self.accept_states.contains(&current) {
                true
            } else { 
                false 
            }, states
        )
    }

    pub fn set_initial_state(&mut self, state: u32) {
        self.initial_state = state;
    }

    pub fn add_accept_state(&mut self, state: u32) {
        self.accept_states.push(state);
    }

    pub fn add_transition(&mut self, input: FSMInput ,from: u32, to: u32) {
        self.transitions.insert((from, input), to);
    }
}
