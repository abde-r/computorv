use std::env;
mod utils;
mod parse;


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {

        match parse::_parser(&args[1]) {

            Ok(reduced_form) => {


                if reduced_form.values().all(|&coeff| coeff == 0.0) {
                    println!("Reduced form: 0 = 0");
                    println!("Polynomial degree: 0");
                    println!("Any number is a solution.");
                }
                else {


                    //let reduced_form = parse::_parser(&args[1]);
                    let degree = *reduced_form.keys().max().unwrap_or(&0);
                    let solutions = utils::solve_equation(&reduced_form, degree);
        
                    println!("Reduced form: {}", utils::reduced_format(&reduced_form));
                    println!("Polynomial degree: {}", degree);
                    println!("{}", solutions);
                   // if let Some(d) = discr {

                     //   println!("Discr: {:.6}", d);
                    //}
                }
            }
                Err(e) => {

                    println!("Error: {}", e);
                }
            
        }
        //let reduced_form = parse::_parser(&args[1]);
        //let degree = *reduced_form.keys().max().unwrap_or(&0);

        //let solutions = utils::solve_equation(&reduced_form, degree);
        
        //println!("Reduced form: {}", utils::reduced_format(&reduced_form));
        //println!("Polynomial degree: {}", degree);
        //println!("{}", solutions);
    }
    else {
        println!("invalid format");
    }
}
