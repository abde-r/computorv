use std::collections::HashMap;

fn parse_term(term: &str) -> Result<(f64, i32), String> {

    if term.contains("X^") {
        let v: Vec<&str> = term.split("X^").collect();
        let coeff = if v[0].trim().is_empty() || v[0].trim() == "+" { 1.0 } else if v[0].trim() == "-" { -1.0 } else { v[0].replace("*", "").trim().parse::<f64>().unwrap() };
        let power = v[1].trim().parse::<i32>().map_err(|_| "Syntax Error! invalid power")?;
        Ok((coeff, power))
    } else if term.contains("X") {
        let v: Vec<&str> = term.split("X").collect();
        let coeff = if v[0].trim().is_empty() || v[0].trim() == "+" { 1.0 } else if v[0].trim() == "-" { -1.0 } else { v[0].replace("*", "").trim().parse::<f64>().unwrap() };
        Ok((coeff, __DELIM__))
    } else {
        Ok((term.trim().parse::<f64>().map_err(|_| "Syntax Error!")?, __DELIM_))
    }
}

fn parse_side(side: &str) -> Result<HashMap<i32, f64>, String> {

    let mut terms = HashMap::new();
    let mut current_term = String::new();
    let mut sign = 1.0;

    for c in side.chars() {
        if c == '+' || c == '-' {
            if !current_term.trim().is_empty() {
                let (coeff, power) = parse_term(&current_term.trim())?;
                *terms.entry(power).or_insert(0.0) += sign * coeff;
                current_term.clear();
            }
            sign = if c == '+' { 1.0 } else { -1.0 };
        }
        else {
            current_term.push(c);
        }
    }

    if !current_term.trim().is_empty() {
        let (coeff, power) = parse_term(&current_term.trim())?;
        *terms.entry(power).or_insert(0.0) += sign * coeff;
    }

    Ok(terms)
}

fn combine_sides(mut left_side: HashMap<i32, f64>, mut right_side: HashMap<i32, f64>) -> HashMap<i32, f64> {

    let mut duplicates = Vec::new();
    for (power, coeff) in &left_side {
        if right_side.get(power) == Some(coeff) {
            duplicates.push(*power);
        }
    }

    for power in duplicates {
        left_side.remove(&power);
        right_side.remove(&power);
    }

    let mut combined = left_side.clone();
    for (power, coeff) in right_side {
        *combined.entry(power).or_insert(0.0) -= coeff;
    }

    combined
}

pub fn _parser(s: &str) -> Result<HashMap<i32, f64>, String> {
   
    let sides: Vec<&str> = s.split('=').collect();
    if sides.len() != 2 {
        return Err("Invalid equation format!".to_string());
    }

    let left_side = parse_side(sides[0]);
    let right_side = parse_side(sides[1]);

    Ok(combine_sides(left_side?, right_side?))
}
