use good_lp::{Solution, SolverModel, constraint, default_solver, variables};
use regex::Regex;
use std::fs::*;

fn solve(c1: u64, c2: u64, c3: u64, c4: u64, k1: u64, k2: u64) -> Result<(u64, u64), ()> {
    let mut result = (0, 0);
    let l1 = k1 + 10000000000000;
    let l2 = k2 + 10000000000000;
    variables! {
        vars:
          0 <= a;
          0 <= b;
    }
    let solution = vars
        .minimise(3 * a + b)
        .using(default_solver) // microlp
        .with(constraint!(
            (c1 as f64) * a + (c3 as f64) * b == (l1 as f64)
        ))
        .with(constraint!(
            (c2 as f64) * a + (c4 as f64) * b == (l2 as f64)
        ))
        .solve();

    if let Ok(sol) = solution {
        result.0 = sol.value(a).round() as u64;
        result.1 = sol.value(b).round() as u64;
    } else {
        return Err(());
    }

    if c1 * result.0 + c3 * result.1 == l1 && c2 * result.0 + c4 * result.1 == l2 {
        Ok(result)
    } else {
        Err(())
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
        if let Ok(solution) = solve(c1, c2, c3, c4, k1, k2) {
            minimum += solution.0 * 3 + solution.1;
            solutions.push(solution);
        }
    }

    println!("{}", minimum);
}
