# Learnings from Day 3

1. `String::lines()` to split the text file into lines.   Cool!
2. `line.split_at(line.len() / 2)` to split the lines in two.  Cool!
3. `.collect::<String>()` to collect an iterator of chars into a String. 
3. Character ascii values: just use `as u32`.  This is a lot like getting an enum discriminator value!
4. "Itertools" adds `unique()` to iterators - pretty cool!
5. You can replace a closure with just it's function name, if the arguments will work.  `map(|x| { some_fn(x) })` becomes `map(some_fn)`.