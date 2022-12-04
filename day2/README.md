# Learnings from Day 2

1. Works well to just keep the text from `fs::read_to_string` in a variable.  That way we know it will stick around long enough.
2. Enums are not pass-by-value by default, even though the discriminator is `i32` by default.   This is a likely due to the fact that any value inside the enum could be a structure.  So, just add `#[derive(Debug, Copy, Clone)]` and it works well.
3. `match` works great for decoding strings, enums, or basically anything.
4. Use `use crate::MyEnum::*` to bring the values into scope to remove some scoping fluff.
5. I really like that in Rust, most statements are also expressions.  Just like Kotlin!
