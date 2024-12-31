#[derive(Debug, PartialEq)]
enum States {
    Init,
    GotA,
    GotB,
    GotC,
}

fn fsm(state: States, c: char) -> States {
    println!("{state:?} {c}");
    match (state, c) {
        (States::Init, 'a') => States::GotA,
        (States::GotA, 'a') => States::GotA,
        (States::GotA, 'b') => States::GotB,
        (States::GotA, 'c') => States::GotC,
        (States::GotB, 'b') => States::GotB,
        (States::GotB, 'c') => States::GotC,
        (States::GotC, _) => if c != 'c' { States::GotC } else { States::Init },
        (_, _) => States::Init,
    }
}

pub fn recognize_pattern(input: &str) -> bool {
    // Implement your finite state automaton here
    println!("pattern: {input}");
    if input.len() > 0 {
        let mut state = States::Init;
        for c in input.chars() {
            state = fsm(state, c);
        }
        if state == States::GotC {
            return true;
        }
    }
    false
}

