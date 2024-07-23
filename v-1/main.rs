use std::env;
use std::io::{self, Write};
mod utils;
mod parse;
mod parser2;


fn main() {

    let args: Vec<String> = env::args().collect();
    let equation = if args.len() == 2 {
        args[1].clone()
    }
    else {
        print!("Enter the equation: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    };

    if utils::syntax_checker(&equation) {
        match parse::_parser(&equation) {
            Ok(reduced_form) => {
                let reduced_form_str = utils::reduced_format(&reduced_form);
                
                println!("Reduced form: {}", reduced_form_str);
                
                let reduced_form2 = parser2::_parser2(&reduced_form_str.to_string());
                let degree = *reduced_form2.keys().max().unwrap_or(&0);
                let solutions = utils::solve_equation(&reduced_form2, degree);
                
                println!("Polynomial degree: {}", degree);
                println!("{}", solutions);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    else {
        println!("Syntax Error! invalid characters");
    }

}
