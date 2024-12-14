use good_lp::{Solution, SolverModel, constraint, default_solver, variables};
use regex::Regex;
use std::fs::*;

fn solve(c1: i32, c2: i32, c3: i32, c4: i32, k1: i32, k2: i32) -> Result<(i32, i32), ()> {
    let mut result = (0, 0);
    variables! {
        vars:
          0 <= a <= 100;
          0 <= b <= 100;
    }
    let solution = vars
        .minimise(3 * a + b)
        .using(default_solver) // microlp
        .with(constraint!(c1 * a + c3 * b == k1))
        .with(constraint!(c2 * a + c4 * b == k2))
        .solve();

    if let Ok(sol) = solution {
        result.0 = sol.value(a).round() as i32;
        result.1 = sol.value(b).round() as i32;
    } else {
        return Err(());
    }

    if c1 * result.0 + c3 * result.1 == k1 && c2 * result.0 + c4 * result.1 == k2 {
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
        let c1 = cap["c1"].parse::<i32>().unwrap();
        let c2 = cap["c2"].parse::<i32>().unwrap();

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
