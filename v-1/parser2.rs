use std::collections::HashMap;

pub fn parse_term2(term: &str) -> (f64, i32) {

    // println!(" term: {}", term);
    let term = term.trim();
    if let Some(index) = term.find(" * X^") {

        let _coeff = &term[..index].trim();
        let _power = &term[index+5..].trim();

        let coeff = if _coeff.is_empty() || *_coeff == "+" { 1.0 } else if *_coeff == "-" { -1.0 } else { _coeff.parse::<f64>().unwrap() };
        let power = _power.parse::<i32>().unwrap();
        (coeff, power)
    }
    else if let Some(index) = term.find(" * X") {

        let _coeff = &term[..index].trim();
        let coeff = if _coeff.is_empty() || *_coeff == "+" { 1.0 } else if *_coeff == "-" { -1.0 } else { _coeff.parse::<f64>().unwrap() };
        (coeff, 1)
    }
    else {

        (term.parse::<f64>().unwrap(), 0)
    }
}

pub fn parse_side2(side: &str) -> HashMap<i32, f64> {

    let mut terms = HashMap::new();
    let mut current_term = String::new();
    let mut sign = 1.0;

    for c in side.chars() {
        if c == '+' || c == '-' {
            if !current_term.is_empty() {
                let (coeff, power) = parse_term2(&current_term);
                // println!("current_term: {}, coeff: {} power: {}", current_term, coeff, power);
                *terms.entry(power).or_insert(0.0) += sign * coeff;
                current_term.clear();
            }
            sign = if c == '+' { 1.0 } else { -1.0 };
        }
        else {
            current_term.push(c);
        }
    }

    if !current_term.is_empty() {
        let (coeff, power) = parse_term2(&current_term);
        *terms.entry(power).or_insert(0.0) += sign * coeff;
    }

    terms
}

pub fn combine_sides2(left_side: HashMap<i32, f64>, right_side: HashMap<i32, f64>) -> HashMap<i32, f64> {

    // println!("\nleft: {:?}\nright: {:?}\n", left_side, right_side);

    let mut combined = left_side.clone();
    for (power, coeff) in right_side {
        *combined.entry(power).or_insert(0.0) -= coeff;
    }

    // println!("combined: {:?}\n", combined);
    combined
}

pub fn _parser2(s: &str) -> HashMap<i32, f64> {
   
    // println!("s {:?}", s);

    let sides: Vec<&str> = s.split('=').collect();
    // if sides.len() != 2 {
    //     return Err("Invalid equation format!".to_string());
    // }

    let left_side = parse_side2(sides[0].trim());
    let right_side = parse_side2(sides[1].trim());
    
    let mut res = combine_sides2(left_side, right_side);
    // println!("res before {:?}", res);
    // Ok(combine_sides(left_side, right_side))
    res.retain(|_, &mut coeff| coeff != 0.0);
    // println!("res after {:?}", res);
    // Ok(combine_sides(left_side, right_side))
    res
}
