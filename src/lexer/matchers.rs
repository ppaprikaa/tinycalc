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
    let mut fsm = fsm::FSM::new();

    fsm.set_initial_state(0);
    fsm.add_accept_state(1);
    fsm.add_accept_state(2);
    fsm.add_accept_state(3);
    fsm.add_accept_state(4);
    fsm.add_accept_state(5);

    fsm.add_transition(fsm::FSMInput::Char('0'), 0, 3);
    fsm.add_transition(fsm::FSMInput::Char('.'), 3, 4);
    fsm.add_transition(fsm::FSMInput::Char('.'), 1, 4);
    fsm.add_transition(fsm::FSMInput::Char('.'), 2, 4);
    
    for char in '0'..='9' {
        if char != '0' {
            fsm.add_transition(fsm::FSMInput::Char(char), 0, 1);
        }
        fsm.add_transition(fsm::FSMInput::Char(char), 1, 2);
        fsm.add_transition(fsm::FSMInput::Char(char), 2, 2);
        fsm.add_transition(fsm::FSMInput::Char(char), 4, 5);
        fsm.add_transition(fsm::FSMInput::Char(char), 5, 5);
    }
    
    fsm
}
