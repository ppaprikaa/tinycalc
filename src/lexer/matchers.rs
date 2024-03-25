use super::fsm;

pub fn new_parenthesis_matcher() -> fsm::FSM {
    let mut fsm = fsm::FSM::new();

    fsm.set_initial_state(0);
    fsm.add_accept_state(1);

    fsm.add_transition(fsm::FSMInput::Char('('), 0, 1);
    fsm.add_transition(fsm::FSMInput::Char(')'), 0, 1);
    fsm
}

pub fn new_operator_matcher() -> fsm::FSM {
    let mut fsm = fsm::FSM::new();

    fsm.set_initial_state(0);
    fsm.add_accept_state(1);

    fsm.add_transition(fsm::FSMInput::Char('+'), 0, 1);
    fsm.add_transition(fsm::FSMInput::Char('-'), 0, 1);
    fsm.add_transition(fsm::FSMInput::Char('*'), 0, 1);
    fsm.add_transition(fsm::FSMInput::Char('/'), 0, 1);

    fsm
}

pub fn new_number_matcher() -> fsm::FSM {
    let digits: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut fsm = fsm::FSM::new();

    fsm.set_initial_state(0);
    fsm.add_accept_state(1);
    fsm.add_accept_state(2);
    fsm.add_accept_state(3);
    fsm.add_accept_state(4);
    fsm.add_accept_state(5);

    fsm.add_transition(fsm::FSMInput::Char(digits[0]), 0, 3);
    fsm.add_transition(fsm::FSMInput::Char('.'), 3, 4);
    fsm.add_transition(fsm::FSMInput::Char('.'), 1, 4);
    fsm.add_transition(fsm::FSMInput::Char('.'), 2, 4);

    for d in &digits[1..] {
        fsm.add_transition(fsm::FSMInput::Char(*d), 0, 1);
    }
    for d in digits {
        fsm.add_transition(fsm::FSMInput::Char(*d), 1, 2);
        fsm.add_transition(fsm::FSMInput::Char(*d), 2, 2);
        fsm.add_transition(fsm::FSMInput::Char(*d), 4, 5);
        fsm.add_transition(fsm::FSMInput::Char(*d), 5, 5);
    }
    
    fsm
}
