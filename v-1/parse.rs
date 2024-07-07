use std::collections::HashMap;


pub fn combine_sides(left_side: HashMap<i32, f64>, right_side: HashMap<i32, f64>) -> HashMap<i32, f64> {

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

    let left_side = parse_side(sides[0])?;
    let right_side = parse_side(sides[1])?;

    Ok(combine_sides(left_side, right_side))
}

pub fn parse_term(term: &str) -> Result<(f64, i32), String> {

    let term = term.trim();
    if let Some(index) = term.find(" * X^") {

        let _coeff = &term[..index].trim();
        let _power = &term[index+5..].trim();
        //let coeff = if _coeff.is_empty() || *_coeff == "+" { 1.0 } else if *_coeff == "-" { -1.0 } else { _coeff.parse::<f64>().unwrap() };
        let coeff = _coeff.parse::<f64>().map_err(|_| format!("invalid coefficient: {}", _coeff))?;
        let power = _power.parse::<i32>().map_err(|_| format!("invalid power: {}", _power))?;
        //let power = _power.parse::<i32>().unwrap();
        Ok((coeff, power))
    }
    else if term.contains("X") {

        let _coeff = term.split(" * X").collect::<Vec<&str>>()[0].trim();
        //let _coeff = &term[..index].trim();
        let coeff = if _coeff.is_empty() { 1.0 } else { _coeff.parse::<f64>().map_err(|_| format!("ivalid coefficient: {}", _coeff))? };
        Ok((coeff, 1))
    }
    else {

        Ok((term.parse::<f64>().map_err(|_| format!("invalid term: {}", term))?, 0))
       // (term.parse::<f64>().unwrap(), 0)
    }
}

pub fn parse_side(side: &str) -> Result<HashMap<i32, f64>, String> {

    let mut terms = HashMap::new();
    //let chars: Vec<char> = side.chars().collect();
    let mut current_term = String::new();
    //let mut index = 0;
    let mut sign = 1.0;

    for c in side.chars() {

        //let c = chars[index];
        if c == '+' || c == '-' {

            if !current_term.is_empty() {

                let (coeff, power) = parse_term(&current_term)?;
                *terms.entry(power).or_insert(0.0) += sign * coeff;
                current_term.clear();
            }
            sign = if c == '+' { 1.0 } else { -1.0 };
        }
        else {
            current_term.push(c);
        }
     //   index+=1;
    }

    if !current_term.is_empty() {

        let (coeff, power) = parse_term(&current_term)?;
        *terms.entry(power).or_insert(0.0) += sign * coeff;
    }

    Ok(terms)
}
