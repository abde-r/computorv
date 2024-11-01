use std::collections::HashMap;

fn parse_term(term: &str) -> (f64, i32) {

    let term = term.trim();
    if let Some(index) = term.find(" * X^") {
        let _coeff = &term[..index].trim();
        let _power = &term[index+5..].trim();

        let coeff = if _coeff.is_empty() || *_coeff == "+" { 1.0 } else if *_coeff == "-" { -1.0 } else { _coeff.parse::<f64>().unwrap() };
        let power = _power.parse::<i32>().unwrap();
        (coeff, power)
    } else if let Some(index) = term.find(" * X") {
        let _coeff = &term[..index].trim();
        let coeff = if _coeff.is_empty() || *_coeff == "+" { 1.0 } else if *_coeff == "-" { -1.0 } else { _coeff.parse::<f64>().unwrap() };
        (coeff, 1)
    } else {
        (term.parse::<f64>().unwrap(), 0)
    }
}

fn parse_side(side: &str) -> HashMap<i32, f64> {

    let mut terms = HashMap::new();
    let mut current_term = String::new();
    let mut sign = 1.0;

    for c in side.chars() {
        if c == '+' || c == '-' {
            if !current_term.is_empty() {
                let (coeff, power) = parse_term(&current_term);
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
        let (coeff, power) = parse_term(&current_term);
        *terms.entry(power).or_insert(0.0) += sign * coeff;
    }

    terms
}

fn combine_sides(left_side: HashMap<i32, f64>, right_side: HashMap<i32, f64>) -> HashMap<i32, f64> {

    let mut combined = left_side.clone();
    for (power, coeff) in right_side {
        *combined.entry(power).or_insert(0.0) -= coeff;
    }

    combined
}

pub fn _parser2(s: &str) -> HashMap<i32, f64> {

    let sides: Vec<&str> = s.split('=').collect();

    let left_side = parse_side(sides[0].trim());
    let right_side = parse_side(sides[1].trim());
    
    let mut res = combine_sides(left_side, right_side);
    res.retain(|_, &mut coeff| coeff != 0.0);
    res
}
