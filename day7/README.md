# Learnings from Day 7

* On Mac OSX, the Rust toolchain requires `xcode` command line tools.   I had to re-install them with `xcode-select --install` to solve `xcrun: error` issues. 
  * _NOTE: This opens up a GUI window._
  * _ALSO NOTE: This may take a few minutes, you have to wait for the GUI window to stop._
* Again, enums with values.   Useful for parsing.  But... this seems like I can solve it with recursion somehow?
* Recursion is possible, but we need to pass a mutable iterator through, so we can move the "read" pointer forward in the nested call.  This is simple enough with `& mut lines.iter()`.
* Experimented with `simple-error`, which implements `std::err::Error`.  Pretty good, but not as convenient as `anyhow`?
* Could probably have implemented the whole thing as one pass, but that would mix syntax errors / parsing with the actual logic.  I kinda like it better in this two-phase approach.
