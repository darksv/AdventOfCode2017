fn main() {
    let mut m = Machine::new('A');

    m.define_state('A', StateDefinition {
        when0: ('B', true, Direction::Right),
        when1: ('E', false, Direction::Left),
    });
    m.define_state('B', StateDefinition {
        when0: ('C', true, Direction::Left),
        when1: ('A', false, Direction::Right),
    });
    m.define_state('C', StateDefinition {
        when0: ('D', true, Direction::Left),
        when1: ('C', false, Direction::Right),
    });
    m.define_state('D', StateDefinition {
        when0: ('E', true, Direction::Left),
        when1: ('F', false, Direction::Left),
    });
    m.define_state('E', StateDefinition {
        when0: ('A', true, Direction::Left),
        when1: ('C', true, Direction::Left),
    });
    m.define_state('F', StateDefinition {
        when0: ('E', true, Direction::Left),
        when1: ('A', false, Direction::Right),
    });

    for _ in 0..12208951 {
        m.step();
    }

    println!("1: {}", m.count_ones());
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

struct Machine<T: PartialEq> {
    tape: Tape,
    state: T,
    states: std::collections::HashMap<T, StateDefinition<T>>,
}

struct StateDefinition<T: PartialEq> {
    when0: (T, bool, Direction),
    when1: (T, bool, Direction),
}

impl<T: PartialEq + Copy + Eq + std::hash::Hash> Machine<T> {
    fn new(initial_state: T) -> Machine<T> {
        Machine {
            tape: Tape::new(),
            state: initial_state,
            states: std::collections::HashMap::new(),
        }
    }

    fn define_state(&mut self, state: T, definition: StateDefinition<T>) {
        self.states.insert(state, definition);
    }

    fn step(&mut self) {
        let state = self.states
            .get(&self.state)
            .expect("missing state?");

        let (next_state, next_value, dir) = match self.tape.read() {
            false => state.when0,
            true => state.when1,
        };

        self.tape.write(next_value);
        match dir {
            Direction::Right => self.tape.move_right(),
            Direction::Left => self.tape.move_left(),
        }
        self.state = next_state;
    }

    fn count_ones(&self) -> usize {
        self.tape.items.iter().filter(|&x| *x).count()
    }
}


struct Tape {
    items: std::collections::VecDeque<bool>,
    pointer: usize,
}

impl Tape {
    fn new() -> Tape {
        let mut tape = std::collections::VecDeque::new();
        tape.push_front(false);
        Tape {
            items: tape,
            pointer: 0,
        }
    }

    fn read(&mut self) -> bool {
        self.items[self.pointer]
    }

    fn write(&mut self, value: bool) {
        self.items[self.pointer] = value;
    }

    fn move_right(&mut self) {
        if self.pointer == self.items.len() - 1 {
            self.items.push_back(false);
        }
        self.pointer += 1;
    }

    fn move_left(&mut self) {
        if self.pointer == 0 {
            self.items.push_front(false);
        } else {
            self.pointer -= 1;
        }
    }
}