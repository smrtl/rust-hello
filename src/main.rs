use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::str;

fn prompt(text: &str) -> String {
  let mut guess = String::new();

  print!("{}", text);
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut guess).unwrap();
  return guess;
}

fn prompt_number(text: &str) -> u32 {
  loop {
    return match prompt(&text).trim().parse() {
      Ok(num) => num,
      Err(e) => {
        println!("Invalid number: {}", e);
        continue;
      }
    };
  }
}

fn main() {
  println!("Guess a number between 0 and 100");
  let secret_number = rand::thread_rng().gen_range(0, 101);

  loop {
    match prompt_number("Please input your guess: ").cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You Win!");
        break;
      }
    }
  }
}
