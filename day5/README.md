# Learnings from Day 5

1. This example seemed to need a more 'mutable' or stateful approach.  So I ran into ... (see next item)
2. Interior mutability - this is a "thing" in Rust.  I used `RefCell` to address it in this case.  It does add some 'ceremony' to things, but it's not terrible.
2. Tuples and type aliases - continues to be good for these examples.