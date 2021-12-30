use aoc_2021::get_input;
use anyhow::Result;

fn into_vec(i: &str) -> Vec<i32> {
    i.split_inclusive(|_| true)
        .map(|x| x.parse().unwrap())
        .collect()
}

fn sum_join<T: std::ops::Add + Copy>(a: &Vec<T>, b: &Vec<T>) -> Vec<T::Output> {
    a.iter().zip(b).map(|(x, y)| x.add(*y)).collect()
}

fn filter_by_value(i: &Vec<Vec<i32>>, bit: i32, idx: usize) -> Vec<Vec<i32>> {
    i.iter()
        .filter(|x| *x.get(idx).unwrap() == bit)
        .cloned()
        .collect()
}

fn filter_by_popular_bit(i: &Vec<Vec<i32>>, idx: usize) -> Vec<i32> {
    if i.len() == 1 {
        return i.get(0).unwrap().clone();
    }
    if i.len() == 0 {
        panic!("hit zero before we could exit");
    }
    let ones = filter_by_value(i, 1, idx);
    let zeros = filter_by_value(i, 0, idx);
    if ones.len() == zeros.len() {
        filter_by_popular_bit(&ones, idx + 1)
    } else if ones.len() < zeros.len() {
        filter_by_popular_bit(&zeros, idx + 1)
    } else {
        filter_by_popular_bit(&ones, idx + 1)
    }
}

fn filter_by_unpopular_bit(i: &Vec<Vec<i32>>, idx: usize) -> Vec<i32> {
    if i.len() == 1 {
        return i.get(0).unwrap().clone();
    }
    let ones = filter_by_value(i, 1, idx);
    let zeros = filter_by_value(i, 0, idx);
    if ones.len() == zeros.len() {
        filter_by_unpopular_bit(&zeros, idx + 1)
    } else if ones.len() > zeros.len() {
        filter_by_unpopular_bit(&zeros, idx + 1)
    } else {
        filter_by_unpopular_bit(&ones, idx + 1)
    }
}

fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let input = get_input(2021, 3)?;
    let line_len = input.lines().next().unwrap().len();
    let input: Vec<Vec<i32>> = input.lines().map(into_vec).collect();
    let counts: Vec<i32> = input
        .iter()
        .fold(vec![0; line_len], |prev, input| sum_join(&prev, input));
    let gamma: i64 = counts
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| {
            if *val > 500 {
                i64::pow(2, idx as u32)
            } else {
                0
            }
        })
        .sum();
    let epsilon: i64 = counts
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| {
            if *val > 500 {
                0
            } else {
                i64::pow(2, idx as u32)
            }
        })
        .sum();
    let o2rating = filter_by_popular_bit(&input, 0);
    let o2rating: i64 = o2rating
        .iter()
        .rev()
        .enumerate()
        .map(
            |(idx, val)| {
                if *val > 0 {
                    i64::pow(2, idx as u32)
                } else {
                    0
                }
            },
        )
        .sum();
    let co2rating = filter_by_unpopular_bit(&input, 0);
    let co2rating: i64 = co2rating
        .iter()
        .rev()
        .enumerate()
        .map(
            |(idx, val)| {
                if *val > 0 {
                    i64::pow(2, idx as u32)
                } else {
                    0
                }
            },
        )
        .sum();

    println!(
        "Answer A - gamma: {}, epsilon: {}, Power Consumption: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
    println!(
        "Answer B - o2rating: {}, co2rating: {}, Life Support: {}",
        o2rating,
        co2rating,
        o2rating * co2rating
    );
    println!("Computed in {}us", start.elapsed().as_micros());
    Ok(())
}
