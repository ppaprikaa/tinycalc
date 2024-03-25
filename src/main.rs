use tinycalc::lexer;
use tinycalc::parser;
use tinycalc::eval;

use std::io::Write;

fn main() {
    let lexer = lexer::Lexer::new();
    let mut line: String;
    println!("write \"exit\" or press Ctrl-C to exit");
    loop {
        line = String::new();

        print!(">> ");
        let _ = std::io::stdout().flush();
        std::io::stdin().read_line(&mut line)
            .expect("failed to read line");
        if line.trim() == "exit" {
            std::process::exit(0);
        }

        let tokens = lexer.analyze(String::from(line.to_string().trim()));
        match tokens {
            Ok(tokens) => {
                let mut parser = parser::Parser::new(tokens);
                let node = parser.parse();
                match node {
                    Ok(node) => {
                        println!("{}", node.to_string());
                        println!("{:?}", eval::eval(node));
                    },
                    Err(err) => println!("{}", err),
                }
            },
            Err(err) => println!("{}", err),
        }
    }
}
