# Learnings from Day 4

1. Itertools has `.collect_tuple()`, which is really convenient for parsing "pairs" of things.
2. Tuples were really great for this example.
3. Sometimes the compiler can figure out the types from the "left hand side" - even inside nested lambdas!
4. Error propagation would probably be good - so we can avoid all the `unwrap()` and just use the `?` operator.  Maybe I'll do that on the next one.
5. `rustfmt` definitely makes this particular code more "fluffy", but I can see the sense in it.