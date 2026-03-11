# ZMORSE-CLI

Morse Code converter CLI tool written in Rust.

## Usage
```sh
zmorse -e "Hello World"
>> .... . .-.. .-.. --- / .-- --- .-. .-.. -..

zmorse -d ".... . .-.. .-.. --- / .-- --- .-. .-.. -.."
>> hello world

// Use with pipe operator 
echo "sos" | zmorse
>> ... --- ...

echo "... --- ..." | zmorse -d
>> sos
```

## Playground
  Run `zmorse` without any arguments to go into playground. In playground, you can convert morse code to text and vice versa interactively. Run provided commands to configure the playground.

  | Commands     | &nbsp;                                  |
  | -            | -                                       |
  |`/switch, /s` | Switch between Encode and Decode Modes. |
  |`/help, /?`   | Display help message for playground.    |
  |`/table`      | Print morse code lookup table.          |
  |`/exit, /q`   | Exit the program.                       |

## Installation
### Windows

### Linux

### MacOS

## Development
### Requirements
  - Rustup
  - Rustc
  - Cargo

### Crates
  - clap
  - colored
  - dist

### Quick Start
  - `git clone https://github.com/zsphinxyz/zmorse-cli`
  - `cd "zmorse-cli"`
  - `cargo run`