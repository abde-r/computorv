use std::env;
mod utils;
mod parse;


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {

        let reduced_form = parse::_parser(&args[1]);
        let degree = *reduced_form.keys().max().unwrap_or(&0);

        let solutions = utils::solve_equation(&reduced_form, degree);
        
        println!("Reduced form: {}", utils::reduced_format(&reduced_form));
        println!("Polynomial degree: {}", degree);
        println!("{}", solutions);
    }
    else {
        println!("invalid format");
    }
}
