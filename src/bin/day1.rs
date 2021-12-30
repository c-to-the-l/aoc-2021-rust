use aoc_2021::get_input;
use anyhow::Result;
use itertools::izip;

fn main() -> Result<()> {
    let t_start = std::time::Instant::now();
    let depths: Vec<i32> = get_input(2021, 1)?
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let p1result = depths[..]
        .iter()
        .zip(depths[1..].iter())
        .filter(|(prev, next)| prev < next)
        .count();
    println!("Part 1: {}", p1result);

    let p2result = izip!(
        depths[..].iter(),
        depths[1..].iter(),
        depths[2..].iter(),
        depths[3..].iter()
    )
    .filter(|(a, b, c, d)| (*a + *b + *c) < (*b + *c + *d))
    .count();
    println!("Part 2: {}", p2result);
    println!("Computed in {}us", t_start.elapsed().as_micros());
    Ok(())
}
