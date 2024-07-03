use std::env;

fn _parser(s: String) -> Option<String> {
    let equ: Vec<&str> = s.split('=').collect();
    if equ.len() != 2 {
        return None;
    }

    println!("splitted eq: {:?}", equ);
    return Some(s);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {

        match _parser(args[1].clone()) {
            Some(_coeff) => println!("coeff: {}", _coeff),
            None => println!("Invalid format"),
        }
        
       // Some(_coeff) = _parser(args[1].clone());
        
       // if _coeff {
         //   println!("{}", _coeff);
       // }
       // else {
        //    println!("invalid format!");
          //  return 0;
       // };
    }
    else {
        println!("invalid format");
    }
    //for arg in env::args() {
    //    println!("{arg}");
    //}
}
