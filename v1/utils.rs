use std::collections::HashMap;


pub fn reduced_format(eq: &HashMap<i32, f64>) -> String {

    let mut terms: Vec<String> = eq.iter()
    //.filter(|&(_, &coeff)| coeff != 0.0)
    .map(|(&power, &coeff)| {
        if power == 0 {
            format!("{} * X^{}", coeff, power) 
        }
        else {
            format!("{:+} * X^{}", coeff, power) 
        }
    })
    .collect();

    terms.sort_by(|a, b| {

        let power_a: i32 = a.split("^").last().unwrap().parse().unwrap();
        let power_b: i32 = b.split("^").last().unwrap().parse().unwrap();
        power_a.cmp(&power_b)
    });

    if terms.is_empty() {
    
        "0".to_string()
    }
    else {

        terms.join(" ")+" = 0"
    }
}

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> String {

    let discr = b*b-4.0*a*c;
    if discr > 0.0 {

        let r1 = (-b - discr.sqrt()) / (2.0*a);
        let r2 = (-b + discr.sqrt()) / (2.0*a);
        format!("Discriminant is strictly positive, the two solutions are:\n{:.6} \n{:.6}", r1, r2)
    }
    else if discr == 0.0 {

        let r = -b/(2.0*a);
        format!("The solution is:\n{:.6}", r)
    }
    else {
    
        format!("Discriminant is strictly positive, the two solutions are:\n{:.6} \n{:.6}", -b*(2.0*a), (-discr).sqrt() / (2.0*a))
    }
}

pub fn solve_equation(equation: &HashMap<i32, f64>, degree: i32) -> String {

    if degree == 2 {

        let a = *equation.get(&2).unwrap_or(&0.0);
        let b = *equation.get(&1).unwrap_or(&0.0);
        let c = *equation.get(&0).unwrap_or(&0.0);
        solve_quadratic(a, b, c)
    }
    else if degree == 1 {

        let b = *equation.get(&1).unwrap_or(&0.0);
        let c = *equation.get(&0).unwrap_or(&0.0);
        format!("The solution is\n{}", -c/b)
    }
    else {

        format!("The polynomial degree is strictly greater than 2, I can't solve.")
    }
}
