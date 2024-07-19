use std::collections::HashMap;

fn parse_term(term: &str) -> (f64, i32) {

    if term.contains("X^") {

        let v: Vec<&str> = term.split("X^").collect();
        // let v =v[0].replace("*", "");
        // println!("we we {:?}", v);
        let coeff = if v[0].trim().is_empty() || v[0].trim() == "+" { 1.0 } else if v[0].trim() == "-" { -1.0 } else { v[0].replace("*", "").trim().parse::<f64>().unwrap() };
        let power = v[1].trim().parse::<i32>().unwrap();
        (coeff, power)
    } else if term.contains("X") {
        let v: Vec<&str> = term.split("X").collect();
        // let v =v[0].replace("*", "");
        // println!("we we {:?}", v);
        let coeff = if v[0].trim().is_empty() || v[0].trim() == "+" { 1.0 } else if v[0].trim() == "-" { -1.0 } else { v[0].replace("*", "").trim().parse::<f64>().unwrap() };
        (coeff, -88)
    } else {
        (term.trim().parse::<f64>().unwrap(), -99)
    }

    // let term = term.trim();
    // println!("term {}", term);

    // if let Some(index) = term.find(" * X^") {

    //     let _coeff = &term[..index].trim();
    //     let _power = &term[index+5..].trim();
    //     // let coeff = _coeff.parse::<f64>().map_err(|_| format!("invalid coefficient: {}", _coeff))?;
        
    // }
    // else if term.contains("X") {
        
        
    //     println!("we we {}", _coeff);
    //     let coeff = if _coeff.is_empty() || _coeff == "+" { 1.0 } else if _coeff == "-" { -1.0 } else { _coeff.parse::<f64>().unwrap() };
    //     // let coeff = if _coeff.is_empty() { 1.0 } else { _coeff.parse::<f64>().map_err(|_| format!("ivalid coefficient: {}", _coeff))? };
    //     (coeff, -88)
    // }
    // else {

    //     (term.parse::<f64>().unwrap(), -99)
    // }
}

fn parse_side(side: &str) -> HashMap<i32, f64> {

    let mut terms = HashMap::new();
    let mut current_term = String::new();
    let mut sign = 1.0;

    for c in side.chars() {
        if c == '+' || c == '-' {
            if !current_term.is_empty() {
                let (coeff, power) = parse_term(&current_term);
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
        let (coeff, power) = parse_term(&current_term);
        *terms.entry(power).or_insert(0.0) += sign * coeff;
    }

    terms
}

fn to_left(eq: &HashMap<i32, f64>) -> String {

        let mut result = String::new();
        let terms: Vec<(i32, f64)> = eq.iter().map(|(&power, &coeff)| (power, coeff)).collect();
    
        // println!("we wee {:?} {:?}", eq, terms);
        // terms.sort_by(|a, b| b.0.cmp(&a.0));
        for (i, (power, coeff)) in terms.iter().enumerate() {
            // if *coeff != 0.0 {
                let sign = if *coeff >= 0.0 && i > 0 {
                    " + "
                } else if *coeff < 0.0 {
                    " - "
                } else {
                    ""
                };
    
                if *power == -99 {
                    result.push_str(&format!("{}{}", sign, coeff.abs()));
                }
                else if *power == -88 {
                    result.push_str(&format!("{}{} * X", sign, coeff.abs()));
                }
                else {
                    result.push_str(&format!("{}{} * X^{}", sign, coeff.abs(), power));
                }
                // let abs_coeff = coeff.abs();
    
                // if power == &0 {
                //     result.push_str(&format!("{}{}", sign, abs_coeff));
                // } else {
                // }
            // }
        }
    
        // if result.is_empty() {
        //     // result.push('0');
        //     result.push_str("0 * X^0 = 0");
    
            
        // } else {
        //     result.push_str(" = 0");
        // }
    
        result.trim().to_string()
}
fn combine_sides(mut left_side: HashMap<i32, f64>, mut right_side: HashMap<i32, f64>) -> HashMap<i32, f64> {

    println!("we we{}", to_left(&right_side));
    println!("Move everything to right side: {:?} = 0", to_left(&left_side) + if &to_left(&right_side)[0..0] == "-" {" "} else { " + " } + &to_left(&right_side));

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

    // println!("\nleft: {:?}\nright: {:?}\n", left_side, right_side);

    let mut combined = left_side.clone();
    for (power, coeff) in right_side {
        *combined.entry(power).or_insert(0.0) -= coeff;
    }

    // println!("Moving to right side {:?}", combined);
    combined
}

pub fn _parser(s: &str) -> Result<HashMap<i32, f64>, String> {
   
    let sides: Vec<&str> = s.split('=').collect();
    if sides.len() != 2 {
        return Err("Invalid equation format!".to_string());
    }

    let left_side = parse_side(sides[0]);
    let right_side = parse_side(sides[1]);
    
    // let mut res = combine_sides(left_side, right_side);
    // println!("res before {:?}", res);
    // Ok(combine_sides(left_side, right_side))
    // res.retain(|_, &mut coeff| coeff != 0.0);
    // println!("res after {:?}", res);
    Ok(combine_sides(left_side, right_side))
    // Ok(res)
}
