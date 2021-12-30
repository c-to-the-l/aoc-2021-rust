use aoc_2021::get_input;
use anyhow::Result;
use std::collections::HashSet;

fn score_corrupt_bracket(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}
fn score_completion_bracket(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn open_bracket_for(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => '0',
    }
}

fn main() -> Result<()> {
    let input = get_input(2021, 10)?;
    let start = std::time::Instant::now();
    let obrackets: HashSet<char> = HashSet::from(['(', '{', '<', '[']);
    let mut stack: Vec<char> = Vec::new();
    let mut sum_a: usize = 0;
    let mut scores_b: Vec<usize> = Vec::new();
    for line in input.lines() {
        let mut inter_a: usize = 0;

        for c in line.chars() {
            if obrackets.contains(&c) {
                stack.push(c);
            } else {
                if Some(&open_bracket_for(c)) == stack.last() {
                    stack.pop();
                } else {
                    inter_a = score_corrupt_bracket(c);
                    break;
                }
            }
        }
        if inter_a == 0 {
            let mut inter_b: usize = 0;
            while let Some(i) = stack.pop() {
                inter_b = (inter_b*5) + score_completion_bracket(i);
            }
            scores_b.push(inter_b);
        } else {
            sum_a += inter_a;
        }
        stack.clear();
    }
    scores_b.sort();
    println!("Score A: {}", sum_a);
    println!("Score B: {}", scores_b.get(scores_b.len()/2).unwrap() );

    println!("Computed in {}us", start.elapsed().as_micros());

    Ok(())
}
