use std::env;
use std::collections::HashMap;

fn parse_side(side: &str) -> (f64, i32) {

    let s = side.trim();
    let v: Vec<&str> = s.split(" * X^").collect();
    if v.len() == 2 {

        let coeff = v[0].parse::<f64>().unwrap();
        let power = v[1].parse::<i32>().unwrap();
        (coeff, power)
    }
    else if s.contains("X") {

        let coeff = v[0].replace("* X", "").trim().parse::<f64>().unwrap();
        (coeff, 1)
    }
    else {

        (s.parse::<f64>().unwrap(), 0)
    }
}

fn parse_sides(side: &str) -> HashMap<i32, f64> {

    let mut sides = HashMap::new();
    for _side in side.split('+').flat_map(|t| t.split('-').map(|s| s.trim())) {
        let (_coeff, power) = parse_side(_side);
        *sides.entry(power).or_insert(0.0) += _coeff;
    }
    sides
}

fn combine_sides(left_side: HashMap<i32, f64>, right_side: HashMap<i32, f64>) -> HashMap<i32, f64> {

    let mut combined = left_side.clone();
    for (power, coeff) in right_side {

        *combined.entry(power).or_insert(0.0) -= coeff;
    }
    combined
}

fn _parser(s: String) -> HashMap<i32, f64> {
    let equ: Vec<&str> = s.split('=').collect();
    //if equ.len() != 2 {
    //    return None;
    //}

    println!("splitted eq: {:?}", equ);
    let left_side = parse_sides(equ[0]);
    let right_side = parse_sides(equ[1]);

    return combine_sides(left_side, right_side);
   // return Some(s);
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> (String, f64) {

    let discr = b*b-(4.0*a*c);
    if discr > 0.0 {

        let r1 = (-b + discr.sqrt()) / (2.0*a);
        let r2 = (-b - discr.sqrt()) / (2.0*a);
        (format!("Discriminant is strictly positive, the two solutions are: {}, {}", r1, r2), discr)
    }
    else if discr == 0.0 {

        let r = (-1.0*b)/(2.0*a);
        (format!("The solution is: {}", r), discr)
    }
    else {
    
        (format!("Discriminant is strictly positive, the two solutions are: {}, {}", -b*(2.0*a), (-discr).sqrt() / (2.0*a)), discr)
    }
}

fn solve_equation(equation: HashMap<i32, f64>, degree: i32) -> (String, Option<f64>) {

    if degree == 2 {

        let a = equation.get(&2).unwrap_or(&0.0);
        let b = equation.get(&1).unwrap_or(&0.0);
        let c = equation.get(&0).unwrap_or(&0.0);
        let (_solution, discr) = solve_quadratic(*a, *b, *c);
        (_solution, Some(discr))
    }
    else if degree == 1 {

        let b = equation.get(&1).unwrap_or(&0.0);
        let c = equation.get(&0).unwrap_or(&0.0);
        (format!("The solution is {}", (-1.0*c)/b), None)
    }
    else {

        (format!("The polynomial degree is strictly greater than 2, I can't solve."), None)
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {

        let reduced_form = _parser(args[1].clone());
        let temp = reduced_form.clone();
        let temp2 = temp.clone();
        let degree = temp2.keys().max().unwrap_or(&0);

        let (solutions, discr) = solve_equation(temp, *degree);
        
        println!("reduced form: {:?}", reduced_form);
        println!("degree: {}", degree);
        if let Some(d) = discr {
            println!("discr: {}", d);
        }
        println!("solutions: {}", solutions);
        //match _parser(args[1].clone()) {
        //    Some(_coeff) => println!("coeff: {}", _coeff),
        //    None => println!("Invalid format"),
        //}
    }
    else {
        println!("invalid format");
    }
//    return 1;
}
