use aoc_2021::get_input;
use anyhow::Result;

fn abs_diff(a: usize, b: &usize) -> usize {
    if a < *b {
        b - a
    } else {
        a - b
    }
}
fn abs_diff_scale(a: usize, b: &usize) -> usize {
    let n = if a < *b {
        b - a
    } else {
        a - b
    };
    (n * (n+1))/2 // == (0..=n).sum() but much faster. Triangular numbers!
}

fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let input = get_input(2021, 7)?;
    let mut crabs: Vec<usize> = input.trim().split(",").map(|x| x.parse().unwrap()).collect();
    crabs.sort();
    let min = crabs.iter().min().unwrap().clone();
    let max = crabs.iter().max().unwrap().clone();

    let min_fuel_a: usize = (min..max).map(|x| {
        crabs.iter().map(|crab| {
            abs_diff(x, crab)
        }).sum()
    }).min().unwrap();
    
    let min_fuel_b: usize = (min..max).map(|x| {
        crabs.iter().map(|crab| {
            abs_diff_scale(x, crab)
        }).sum()
    }).min().unwrap();

    let mean: usize = crabs.iter().sum::<usize>()/crabs.len();
    let median: usize = crabs.get(crabs.len()/2).unwrap().clone();
    let maybe_a: usize = crabs.iter().map(|crab| {
        abs_diff(median, crab)
    }).sum();
    let maybe_b: usize = crabs.iter().map(|crab| {
        abs_diff_scale(mean, crab)
    }).sum();
    assert_eq!(maybe_a, min_fuel_a);
    assert_eq!(maybe_b, min_fuel_b);

    println!("Answer A {:?}", min_fuel_a);
    println!("Answer B {:?}", min_fuel_b);
    println!("Computed in {}us", start.elapsed().as_micros());  
    Ok(())
}