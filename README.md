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
```sh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/zsphinxyz/zmorse-cli/releases/download/v0.1.1/zmorse-installer.ps1 | iex"
```
### Linux
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/zsphinxyz/zmorse-cli/releases/download/v0.1.1/zmorse-installer.sh | sh
```
> More download options in [Release](https://github.com/zsphinxyz/zmorse-cli/releases).

## Development
### Requirements
  - Rustup
  - Rustc
  - Cargo
    - clap
    - colored
    - dist

### Getting Started
  - `git clone https://github.com/zsphinxyz/zmorse-cli`
  - `cd "zmorse-cli"`
  - `cargo run`