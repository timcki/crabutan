mod lexer;

use lexer::Lexer;
use std::io::{self, Write};

struct Repl {
    prompt: &'static str,
    //lexer: Lexer<'a>,
}

impl Repl {
    fn new(prompt: &'static str) -> Repl {
        Repl {
            prompt,
            //lexer: Lexer::new(""),
        }
    }

    fn start(&self) {
        println!("Welcome to the Crabutan REPL!");
        loop {
            let mut input = String::new();
            print!("{}", self.prompt);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            println!("You entered: {:?}", input);

            match input.trim().as_ref() {
                "" => continue,
                "exit" => break,
                _ => {}
            }
            let lexer = Lexer::new(&input);
            for token in lexer {
                println!("{:?}", token);
            }
            //println!("You entered: {}", input);
        }
    }
}

fn main() {
    let repl = Repl::new("> ");
    repl.start();
}
