use std::cell::RefCell;

// Some type aliases to make things readable, while still using simple types.  This is great
// for this type of problem where you don't need a structure and traits.

type Crate = char;
type Stack = Vec<Crate>;
type Move = (i32, usize, usize);

fn stack_from_string(s : &str) -> RefCell<Stack> {
    // Since Stack is just a Vec<char>, we can just collect into it.
    // We wrap this in a RefCell so that interior of the overall vector of stacks can be mutable.
    RefCell::new(s.chars().collect::<Stack>())
}

fn move_crates(n : i32, source : &mut Stack, target : &mut Stack) {
    println!("before: source={:?} target={:?}", source, target);
    // Remove 'n' elements from the front of the source stack, put them onto the target.
    source.drain(0..n as usize).for_each(|c| { target.insert(0, c) });
    println!("after: source={:?} target={:?}", source, target);
}

fn main() {
    // Outer vector is immutable, but the stacks inside can be borrowed mutably.
    let stacks = vec![
        stack_from_string("NZ"),
        stack_from_string("DCM"),
        stack_from_string("P")
    ];

    // The list of moves: Count, from stack (1 based), to stack (1 based)
    // Using 1-based addresses here because it makes it easier to verify against the example.
    let moves: Vec<Move> = vec![
        (1, 2, 1),
        (3, 1, 3),
        (2, 2, 1),
        (1, 1, 2)
    ];

    println!("stacks={:?}", stacks);
    for m in moves {
        move_crates(m.0,
                    &mut stacks[m.1 - 1].borrow_mut(),
                    &mut stacks[m.2 - 1].borrow_mut())
    }
    println!("After move: stacks={:?}", stacks);

    let top : Vec<Crate> = stacks.iter().map(|s| { s.borrow()[0] }).collect();

    println!("top={:?}", top);
}
