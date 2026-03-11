use std::io::{self, Write};
use colored::Colorize;
pub struct Morse;

impl Morse {
  const MORSE_LOOKUP: [(&'static str, char); 53] = [
    ("-.", 'n'),
    (".-", 'a'),
    ("-...", 'b'),
    ("-.-.", 'c'),
    ("-..", 'd'),
    (".", 'e'),
    ("..-.", 'f'),
    ("--.", 'g'),
    ("....", 'h'),
    ("..", 'i'),
    (".---", 'j'),
    ("-.-", 'k'),
    (".-..", 'l'),
    ("--", 'm'),
    ("---", 'o'),
    (".--.", 'p'),
    ("--.-", 'q'),
    (".-.", 'r'),
    ("...", 's'),
    ("-", 't'),
    ("..-", 'u'),
    ("...-", 'v'),
    (".--", 'w'),
    ("-..-", 'x'),
    ("-.--", 'y'),
    ("--..", 'z'),
    (".-.-.-", '.'),
    ("--..--", ','),
    ("..--..", '?'),
    (".----.", '\''),
    (".-..-.", '"'),
    ("-.-.--", '!'),
    ("-..-.", '/'),
    ("---...", ':'),
    ("-.-.-.", ';'),
    ("-.--.", '('),
    ("-.--.-", ')'),
    ("-...-", '='),
    ("-....-", '-'),
    ("..--.-", '_'),
    (".-.-.", '+'),
    (".--.-.", '@'),
    (".----", '1'),
    ("..---", '2'),
    ("...--", '3'),
    ("....-", '4'),
    (".....", '5'),
    ("-....", '6'),
    ("--...", '7'),
    ("---..", '8'),
    ("----.", '9'),
    ("-----", '0'),
    ("/", ' ')
  ];
  
  fn encode(char: char) -> &'static str {
    Self::MORSE_LOOKUP
      .iter()
      .find(|(_, c)| *c == char)
      .map(|(s,_)| *s)
      .unwrap_or("#")
  }
  
  fn decode(morse: &str) -> char {
    Self::MORSE_LOOKUP
      .iter()
      .find(|(m, _)| *m == morse )
      .map(|(_, s)| *s)
      .unwrap_or('#')
  }

  fn table() {
    for (key, value) in &Self::MORSE_LOOKUP[..&Self::MORSE_LOOKUP.len()-1] {
      println!("{:>8} {:>2}", key, value)
    }
  }
}


pub fn print_help() {
    println!("Morse code playground. Switch between {} and {} modes for converting text to morse code and vice versa. 
Note: Use / for space. # will be used for invalid character.
    {}
    {:<11}    :Switch between Encode and Decode Modes.
    {:<11}    :Display this help message.
    {:<11}    :Print lookup table.
    {:<11}    :Exit the program.",
    "Encode".cyan(), "Decode".cyan(), "Commands-".underline().bold(),
    "/switch, /s".bold().cyan(), "/help, /?".bold().cyan(), "/table".bold().cyan(), "/exit, /q".bold().cyan()
    );
}

pub fn encoder(input: &String) -> String {
  let letter_li = input.trim().chars();
  let morse_li: Vec<&str> = letter_li.map(|i|{Morse::encode(i.to_ascii_lowercase())}).collect();
  morse_li.join(" ")
}

pub fn decoder(input: &String) -> String{
  let morse_li: Vec<&str> = input.trim().split(' ').collect();
  let char_li: String = morse_li.iter().map(|i| {Morse::decode(i)}).collect() ;
  char_li
}

pub fn validate(v: &String) -> &String {
    if v.is_empty() {
        eprintln!("{}: {}", "Error".red().bold(), "unexpected argument found!\n\nFor more information, try '--help'");
        std::process::exit(1);
    }
    v
}


pub fn playground() {
    println!("zmorse playground.\nType \"{}\", \"{}\" to switch modes.\n \"{}\" for more information and \"{}\" to exit the program.", "/encode".cyan(), "/decode".cyan(), "/help".cyan(), "/exit".cyan());
    
    let mut switch = true;
    let mut input = String::new();
    
    loop {
        print!("{} ", if switch {"▪"} else {"▸"});
        io::stdout().flush().expect("Failed to flush stdout");
        input.clear();
        io::stdin().read_line(&mut input).expect("Error");
    
        match input.trim().to_lowercase().as_str() {
            "/switch" | "/s" => {
              switch = !switch;
              println!("{}", format!("{} {}", ">> Switched to", if switch {"encoder. Enter text."} else {"decoder. Enter morse code."}).green());
            },
            "/table"        => Morse::table(),
            "/help" | "/?"  => print_help(),
            "/exit" | "/q"  => {
                println!("{}", ">> Program Exited".green());
                break;
            },
            _ => {
              if switch {
                println!("{}", encoder(&input));
              } else {
                println!("{}", decoder(&input));
              }
            }
        }
    }
}

