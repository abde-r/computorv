use std::collections::HashMap;
use std::io;

fn main() {
    let equation = read_equation();
    match reduced_form(&equation) {
        Ok(form) => println!("Reduced form: {}", form),
        Err(err) => println!("Error: {}", err),
    }
}

fn read_equation() -> String {
    println!("Please enter a polynomial equation:");
    let mut equation = String::new();
    io::stdin().read_line(&mut equation).expect("Failed to read line");
    equation.trim().to_string()
}

fn parse_term(term: &str) -> Result<(f64, i32), String> {
    let parts: Vec<&str> = term.split('*').map(|s| s.trim()).collect();
    if parts.len() == 2 {
        let coeff: f64 = parts[0].parse().map_err(|_| "Invalid coefficient")?;
        let power: i32 = if parts[1] == "X" {
            1
        } else if parts[1].starts_with("X^") {
            parts[1][2..].parse().map_err(|_| "Invalid power")?
        } else {
            return Err("Invalid term format".to_string());
        };
        Ok((coeff, power))
    } else if parts.len() == 1 {
        let coeff: f64 = parts[0].parse().map_err(|_| "Invalid coefficient")?;
        Ok((coeff, 0))
    } else {
        Err("Invalid term format".to_string())
    }
}

fn parse_side(side: &str) -> Result<HashMap<i32, f64>, String> {
    let mut terms = HashMap::new();
    let mut current_term = String::new();
    let mut sign = 1.0;

    for c in side.chars() {
        if c == '+' || c == '-' {
            if !current_term.is_empty() {
                let (coeff, power) = parse_term(&current_term)?;
                *terms.entry(power).or_insert(0.0) += sign * coeff;
                current_term.clear();
            }
            sign = if c == '+' { 1.0 } else { -1.0 };
        } else {
            current_term.push(c);
        }
    }

    if !current_term.is_empty() {
        let (coeff, power) = parse_term(&current_term)?;
        *terms.entry(power).or_insert(0.0) += sign * coeff;
    }

    Ok(terms)
}

fn combine_sides(lhs: HashMap<i32, f64>, rhs: HashMap<i32, f64>) -> HashMap<i32, f64> {
    let mut combined = lhs.clone();
    for (power, coeff) in rhs {
        *combined.entry(power).or_insert(0.0) -= coeff;
    }
    combined
}

fn reduced_form(equation: &str) -> Result<String, String> {
    let sides: Vec<&str> = equation.split('=').map(|s| s.trim()).collect();
    if sides.len() != 2 {
        return Err("Invalid equation format".to_string());
    }

    let left_side = parse_side(sides[0])?;
    let right_side = parse_side(sides[1])?;
    let combined = combine_sides(left_side, right_side);

    let mut terms: Vec<String> = combined.iter().filter(|&(_, &coeff)| coeff != 0.0)
        .map(|(&power, &coeff)| format!("{:+} * X^{}", coeff, power))
        .collect();

    terms.sort_by(|a, b| {
        let power_a: i32 = a.split("^").last().unwrap().parse().unwrap();
        let power_b: i32 = b.split("^").last().unwrap().parse().unwrap();
        power_b.cmp(&power_a)
    });

    if terms.is_empty() {
        Ok("0 * X^0 = 0".to_string())
    } else {
        Ok(terms.join(" ") + " = 0")
    }
}


// 3 * X^2 - 2 * X^1 + 5 = 2 * X^2 + 3 * X^1 - 4
// -5 * X^0 + 4 * X^1 - 9.3 * X^2 = -1 * X^0
// irreducible fractions ???
// intermidiate steps --> (1- move to right side, discriminant, defining the solutions possibilities)