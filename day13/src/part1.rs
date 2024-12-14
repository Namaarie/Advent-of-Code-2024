use regex::Regex;
use std::fs::*;

pub fn solve(m_i: (u64, u64, u64, u64), b_i: (u64, u64)) -> Option<(u64, u64)> {
    // get f64 version of input
    let m = (m_i.0 as f64, m_i.1 as f64, m_i.2 as f64, m_i.3 as f64);
    let b = (b_i.0 as f64, b_i.1 as f64);

    // solve Ax=b using inverse A*b
    // invert 2x2 using determinant
    let mut d = m.0 * m.3 - m.1 * m.2;
    if d == 0.0 {
        return None;
    } else {
        d = 1.0 / d;
    }

    let inv = (m.3 * d, -m.1 * d, -m.2 * d, m.0 * d);
    let x_f = (inv.0 * b.0 + inv.1 * b.1, inv.2 * b.0 + inv.3 * b.1);
    let x_i = (x_f.0.round() as u64, x_f.1.round() as u64);

    // check if vaid solution
    if m_i.0 * x_i.0 + m_i.1 * x_i.1 == b_i.0 && m_i.2 * x_i.0 + m_i.3 * x_i.1 == b_i.1 {
        return Some((x_i.0, x_i.1));
    } else {
        return None;
    }
}

pub fn main() {
    let binding = read_to_string("input.txt").unwrap();
    let re = Regex::new(r"(?<c1>[0-9]+)(.*?)(?<c2>[0-9]+)").unwrap();

    let mut nums = Vec::new();

    for cap in re.captures_iter(&binding) {
        let c1 = cap["c1"].parse::<u64>().unwrap();
        let c2 = cap["c2"].parse::<u64>().unwrap();

        nums.push(c1);
        nums.push(c2);
    }

    let mut solutions = Vec::new();
    let mut minimum = 0;

    while nums.len() > 0 {
        let k2 = nums.pop().unwrap();
        let k1 = nums.pop().unwrap();
        let c4 = nums.pop().unwrap();
        let c3 = nums.pop().unwrap();
        let c2 = nums.pop().unwrap();
        let c1 = nums.pop().unwrap();
        if let Some(solution) = solve((c1, c3, c2, c4), (k1, k2)) {
            minimum += solution.0 * 3 + solution.1;
            solutions.push(solution);
        }
    }

    println!("{}", minimum);
}
