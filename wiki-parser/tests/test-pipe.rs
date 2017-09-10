use std::process::{Command, Stdio};
use std::env::current_dir;
use std::io::prelude::*;
use std::error::Error;

static wikicode: &'static str = r#"{{Q|w=35%|pre=1|Вот из плесени кисель!
Чай, не пробовал досель?
Дак испей — и враз забудешь
Про мирскую карусель!|Леонид Филатов. [[Про Федота-стрельца]], 1985}}

{{Q|w=35%|pre=1|Удивительное рядом, но оно запрещено...
|Владимир Высоцкий. [[Дорогая передача]]}}"#;

fn main() {
  let command = format!("./main.py");
  println!("{}", &command);

  let process = match Command::new(&command[..])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn() {
      Err(why) => panic!("couldn't spawn wiki-parser: {}", why.description()),
      Ok(process) => process,
  };

  match process.stdin.unwrap().write_all(wikicode.as_bytes()) {
    Err(why) => panic!("couldn't write to wiki-parser stdin: {}", why.description()),
    Ok(_) => println!("sent pangram to wiki-parser"),
  }

  let mut output = String::new();
  match process.stdout.unwrap().read_to_string(&mut output) {
    Err(why) => panic!("couldn't read wiki-parser stdout: {}", why.description()),
    Ok(_) => print!("wiki-parser responded with:\n{}", output),
  }
}