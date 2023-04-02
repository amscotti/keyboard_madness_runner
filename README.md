# Keyboard Madness Runner
Simple program exercise given to me by an ex coworker sometime ago for fun. This is a Rust port of my program that was written [F#](https://github.com/amscotti/KeyboardMadness).

## Exercise details

```
"1" "2" "3" "4" "5" "6" "7" "8" "9" "0"
"Q" "W" "E" "R" "T" "Y" "U" "I" "O" "P"
"A" "S" "D" "F" "G" "H" "J" "K" "L" ";"
"Z" "X" "C" "V" "B" "N" "M" "," "." "?"
```

Given this keyboard layout, an starting point of (4, 2) "G" and string of instructions, print out any selected keys. Instructions are separated by a comma, with some instructions haveing a count with them, meaning to repeat to give an action by the number of count.

### Instructions
* "R" - Move Right, can also have a count like "R:3"
* "L" - Move Left, can also have a count like "L:3"
* "D" - Move Down, can also have a count like "D:2"
* "U" - Move Up, can also have a count like "U:2"
* "_" - Add a space to the selected keys
* "N" - Add a new line to the selected keys
* "S" - Select the key at that point

Any unknown instructions are ignored

Sample instruction looks like `R,S,U,L:3,S,D,R:6,S,S,U,S` which will output "HELLO"

## Running Unit Test
Run `cargo test`

## Run Command line Application
Run `cargo run R,S,U,L:3,S,D,R:6,S,S,U,S`

## Usage

```
Usage: keyboard_madness_runner <COMMAND>

Commands:
  run       Run instructions on the keyboard
  generate  Generate instructions
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Building
* Run `cargo build --release`
* Run with `./target/release/keyboard_madness_runner run R,S,U,L:3,S,D,R:6,S,S,U,S`

### Build docker image and use local image
* Build with `docker build . -t keyboard-madness-runner`
* Run with `docker run keyboard-madness-runner run R,S,U,L:3,S,D,R:6,S,S,U,S`