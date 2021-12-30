use aoc_2021::get_input;
use anyhow::Result;
use std::cmp::{max, min};
use std::collections::HashMap;


struct DeterministicDice {
    count: usize,
}

impl DeterministicDice {
    fn new() -> Self {
        DeterministicDice { count: 0 }
    }

    fn take(&mut self) -> usize {
        self.count += 1;
        1 + ((self.count - 1) % 100)
    }

    fn take3(&mut self) -> usize {
        self.take() + self.take() + self.take()
    }
}


fn roll_a(
    cache_a: &mut HashMap<(usize, usize, usize, usize, usize), (usize, usize)>,
    cache_b: &mut HashMap<(usize, usize, usize, usize, usize), (usize, usize)>,
    pos_a: usize,
    pos_b: usize,
    score_a: usize,
    score_b: usize,
    roll: usize,
) -> (usize, usize) {
    if let Some(x) = cache_a.get(&(pos_a, pos_b, score_a, score_b, roll)) {
        return *x;
    }
    let pos_a_n = (pos_a + roll) % 10;
    let score_a_n = score_a + pos_a_n + 1;
    if score_a_n >= 21 {
        return (1, 0);
    }
    let mut ra = 0;
    let mut rb = 0;
    for x in 1..=3 {
        for y in 1..=3 {
            for z in 1..=3 {
                let (r_a, r_b) = roll_b(cache_a, cache_b, pos_a_n, pos_b, score_a_n, score_b, x + y + z);
                ra += r_a;
                rb += r_b;
            }
        }
    }
    cache_a.insert((pos_a, pos_b, score_a, score_b, roll), (ra, rb));

    (ra, rb)
}

fn roll_b(
    cache_a: &mut HashMap<(usize, usize, usize, usize, usize), (usize, usize)>,
    cache_b: &mut HashMap<(usize, usize, usize, usize, usize), (usize, usize)>,
    pos_a: usize,
    pos_b: usize,
    score_a: usize,
    score_b: usize,
    roll: usize,
) -> (usize, usize) {
    if let Some(x) = cache_b.get(&(pos_a, pos_b, score_a, score_b, roll)) {
        return *x;
    }
    let pos_b_n = (pos_b + roll) % 10;
    let score_b_n = score_b + pos_b_n + 1;
    if score_b_n >= 21 {
        return (0, 1);
    }
    let mut ra = 0;
    let mut rb = 0;
    for x in 1..=3 {
        for y in 1..=3 {
            for z in 1..=3 {
                let (r_a, r_b) = roll_a(cache_a, cache_b, pos_a, pos_b_n, score_a, score_b_n, x + y + z);
                ra += r_a;
                rb += r_b;
            }
        }
    }
    cache_b.insert((pos_a, pos_b, score_a, score_b, roll), (ra, rb));

    (ra, rb)
}
fn main() -> Result<()> {
    let t_start = std::time::Instant::now();
    let input = get_input(2021, 21)?;
    let mut lines = input.lines();
    let pa_start: usize = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        - 1;
    let pb_start: usize = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        - 1;

    let mut pa_pos: usize = pa_start;
    let mut pb_pos: usize = pb_start;
    let mut pa_score: usize = 0;
    let mut pb_score: usize = 0;
    let mut dice = DeterministicDice::new();
    while pa_score < 1000 && pb_score < 1000 {
        let r = dice.take3();
        pa_pos = (pa_pos + r) % 10;
        pa_score += pa_pos + 1;
        if pa_score >= 1000 {
            break;
        }
        let r = dice.take3();
        pb_pos = (pb_pos + r) % 10;
        pb_score += pb_pos + 1;
    }


    let mut ra = 0;
    let mut rb = 0;
    let mut cache_a: HashMap<(usize, usize, usize, usize, usize), (usize, usize)> = HashMap::new();
    let mut cache_b: HashMap<(usize, usize, usize, usize, usize), (usize, usize)> = HashMap::new();

    for x in 1..=3 {
        for y in 1..=3 {
            for z in 1..=3 {
                let (r_a, r_b) = roll_a(
                    &mut cache_a,
                    &mut cache_b,
                    pa_start,
                    pb_start,
                    0,
                    0,
                    x + y + z,
                );
                ra += r_a;
                rb += r_b;
            }
        }
    }

    println!("Answer A: {}", min(pa_score, pb_score) * dice.count);
    println!("Answer B: {}", max(ra, rb));

    println!("Computed in {}us", t_start.elapsed().as_micros());
    Ok(())
}
