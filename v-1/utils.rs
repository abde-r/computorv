use std::collections::HashMap;

pub fn reduced_format(eq: &HashMap<i32, f64>) -> String {

    let mut result = String::new();
    let mut terms: Vec<(i32, f64)> = eq.iter().map(|(&power, &coeff)| (power, coeff)).collect();

    terms.sort_by(|a, b| b.0.cmp(&a.0));
    for (i, (power, coeff)) in terms.iter().enumerate() {
        let sign = if *coeff >= 0.0 && i > 0 {
            " + "
        } else if *coeff < 0.0 {
            " - "
        } else {
            ""
        };

        if *power == __DELIM_ {
            result.push_str(&format!("{}{}", sign, coeff.abs()));
        } else if *power == __DELIM__ {
            result.push_str(&format!("{}{} * X", sign, coeff.abs()));
        } else {
            result.push_str(&format!("{}{} * X^{}", sign, coeff.abs(), power));
        }
    }

    if result.is_empty() {
        result.push_str("0 * X^0 = 0");
    } else {
        result.push_str(" = 0");
    }

    result.trim().to_string()
}

pub fn syntax_checker(s: &str) -> bool {

    let allowed_chars: &str = "0123456789-+*^X =.";
    for c in s.chars() {
        if !allowed_chars.contains(c) {
            return false;
        }
    }

    return true;
}


fn solve_quadratic(a: f64, b: f64, c: f64) -> String {

    let discr = b*b-4.0*a*c;
    if discr > 0.0 {
        let x1 = (-b - discr.sqrt()) / (2.0*a);
        let x2 = (-b + discr.sqrt()) / (2.0*a);
        format!("Discriminant is strictly positive, the two solutions are:\nx1 = (-1*{} - √{}) / (2*{})\n   = {} / {}\n   = {:.6}\nx2 = (-1*{} + √{}) / (2*{})\n   = {} / {}\n   = {:.6}", b, discr, a, -1.0*b-discr.sqrt(), 2.0*a, x1, b, discr, a, -1.0*b+discr.sqrt(), 2.0*a, x2)
    } else if discr == 0.0 {
        let x = -b / (2.0*a);
        format!("Discriminant is 0, the solution is:\nx = -1*{} / (2*{})\n  = {} / {}\n  = {:.6}", b, a, -b, 2.0*a, x)
    } else {
        let real_part = -b/(2.0*a);
        let mut imaginary_part = (discr.abs().sqrt())/(2.0*a);
        if imaginary_part<0.0 {
            imaginary_part *= -1.0;
            format!("Discriminant is strictly negative, the two complex solutions are:\nx1 = -1*{}/(2*{}) + √abs({})/2*{}\n   = {:.6} - {:.6}𝒊\nx2 = -1*{}/(2*{}) - √abs({})/2*{}\n   = {:.6} + {:.6}𝒊", b, a, discr, a, real_part, imaginary_part, b, a, discr, a,  real_part, imaginary_part)
        } else {
            format!("Discriminant is strictly negative, the two complex solutions are:\nx1 = -1*{}/(2*{}) + √abs({})/2*{}\n   = {:.6} - {:.6}𝒊\nx2 = -1*{}/(2*{}) - √abs({})/2*{}\n   = {:.6} + {:.6}𝒊", b, a, discr, a, real_part, imaginary_part, b, a, discr, a,  real_part, imaginary_part)
        }
    }
}

pub fn solve_equation(equation: &HashMap<i32, f64>, degree: i32) -> String {

    if degree == 0 {
        let c = *equation.get(&0).unwrap_or(&0.0);
        if c == 0.0 {
            format!("Any real number is a solution.")
        }
        else {
            format!("No solution.")
        }
    }
    else if degree == 1 {
        let b = *equation.get(&1).unwrap_or(&0.0);
        let c = *equation.get(&0).unwrap_or(&0.0);
        if b == 0.0 && c == 0.0 {
            format!("Any real number is a solution.")
        }
        else if b == 0.0 {
            format!("No solution.")
        }
        else {
            format!("The solution is:\nx = -1*{} / {}\n  = {} / {}\n  = {:.6}", c, b, -1.0*c, b, -c/b)
        }
    }
    else if degree == 2 {
        let a = *equation.get(&2).unwrap_or(&0.0);
        let b = *equation.get(&1).unwrap_or(&0.0);
        let c = *equation.get(&0).unwrap_or(&0.0);
        solve_quadratic(a, b, c)
    }
    else {
        format!("The polynomial degree is strictly greater than 2, I can't solve.")
    }
}
