use clap::Parser;
use std::io::{self, IsTerminal, Read};

mod utils;

#[derive(Parser, Debug)]
#[command(name="zmorse",version, about = "ZMORSE", long_about = "Morse Code Converter", after_help="Type `zmorse` to go into playground.")]
struct Args {
    /// eg: `zmorse -e "Hello world"`
    #[arg(short, long, allow_hyphen_values = true, num_args = 0..=1, default_missing_value = "")]
    encode: Option<String>,

    /// eg: `zmorse -d ".... . .-.. .-.. --- / .-- --- .-. .-.. -.."`
    #[arg(short, long, allow_hyphen_values = true, num_args = 0..=1, default_missing_value = "")]
    decode: Option<String>
}

fn main() {
    let args = Args::parse();
    let is_terminal = io::stdin().is_terminal();

    if is_terminal{
        // Terminal
        if let Some(data) = args.encode {
            println!("{}", utils::encoder(utils::validate(&data)));
        } else if let Some(data) = args.decode {
            println!("{}", utils::decoder(utils::validate(&data)));
        } else {
            utils::playground();
        }
    } else {
        pipe_handler(args);
    }
}


fn pipe_handler(args: Args) {
  let mut buffer = String::new();
  io::stdin().read_to_string(&mut buffer).expect("Failed to read piped command");
  let input = buffer.trim().to_string();

  if input.is_empty() {return;}

  if args.decode.is_some() {
    println!("{}", utils::decoder(&input));
  } else {
    println!("{}", utils::encoder(&input));
  }
}
