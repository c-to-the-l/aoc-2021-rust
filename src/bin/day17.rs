use aoc_2021::get_input;
use anyhow::Result;
use std::cmp::{min, max};

fn lands (xvi: i32, yvi: i32, xmin: i32, xmax: i32, ymin: i32, ymax: i32) -> bool {

    let (mut x, mut y) = (0,0);
    let (mut xv, mut yv) = (xvi, yvi);
    while (y > ymax || x < xmin) && (y >= ymin) {
        y += yv;
        x += xv;
        yv -= 1;
        xv = max(0, xv-1);
    }

    if y >= ymin && x <= xmax && x >= xmin {
        true
    } else {
        false
    }
}

fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let input = get_input(2021, 17)?;

    let i: Vec<&str> = input.trim().split(&[' ','.','=',',']).collect();
    let x1: i32 = i.get(3).unwrap().parse().unwrap();
    let x2: i32 = i.get(5).unwrap().parse().unwrap();
    let y1: i32 = i.get(8).unwrap().parse().unwrap();
    let y2: i32 = i.get(10).unwrap().parse().unwrap();
    let ymin = min(y1, y2);
    let ymax = max(y1, y2);
    let xmin = min(x1, x2);
    let xmax = max(x1, x2);
    let vel = ymin.abs() - 1;
    let ans_a = (vel * (vel + 1))/2;

    let mut count: usize = 0;
    for xv in 0..=xmax {
        for yv in ymin..=ymin.abs() {
            if lands(xv, yv, xmin, xmax, ymin, ymax) {
                count +=1;
            }
        }
    }

    println!("Answer A: {}", ans_a);
    println!("Answer B: {}", count);
    println!("Computed in {}us", start.elapsed().as_micros());
    Ok(())
}
