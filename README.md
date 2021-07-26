# terminate
A rust macro with `print!`-like syntax which terminates cleanly enough the current process.
Example output:
```
Nice custom message

thread 'main' panicked at 'term! macro', src/main.rs:5:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
