# Learnings from Day 1

1. Read a file into a string with `fs::read_to_string("elves.txt")`
2. Need to be careful about the result of `fs::read_to_string` to make sure that it lives long enough.
3. Functional processing with `split`, `map`, `sum` and `collect` works similarly to Kotlin.