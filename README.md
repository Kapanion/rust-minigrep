# minigrep
Small version of grep command implemented in Rust.
The program writes all the lines from the specified file that contain the specified word.

The name of the crate is `minigrep`, and most of the code is placed in __lib.rs__.

Based on the minigrep project from [The Rust Programming Language](https://doc.rust-lang.org/book/) book.

## How to Run
To run this project, you must have `cargo` installed. Open the base directory and run the following command:
```
cargo run <word_to_search> <file_path>
```

Case sensitivity can be controlled by existence `CASE_INSENSITIVE` environment variable.

Adding the variable:
```
$Env:CASE_INSENSITIVE=1
```
Removing the variable:
```
Remove-Item Env:CASE_INSENSITIVE
```