use aoc_2021::get_input;
use anyhow::Result;
use std::cmp::max;
use std::collections::HashSet;

enum FoldDir {
    Up,
    Left
}

fn fold_set(dir: &FoldDir, line: &usize, paper: &mut HashSet<(usize, usize)>) {
    let new: HashSet<(usize, usize)> = paper.drain().map(|(a, b)| {
        match *dir {
            FoldDir::Left => {
                (if a>=*line {*line-(a - *line)} else {a},b)
            },
            FoldDir::Up=> {
                (a,if b>=*line {*line-(b - *line)} else {b})
            }
        }
    }).collect();
    *paper = new;
}

fn paprint_set(paper: &HashSet<(usize, usize)>) {
    let (x_max, y_max): (usize, usize) = paper.iter().fold((0,0), |(xp, yp), (xn, yn)| {
        (max(*xn, xp), max(*yn, yp))
    });
    for y in 0..=y_max {
        for x in 0..=x_max {
            if paper.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("");
    }
}

fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let input = get_input(2021, 13)?;
    let mut point_end: usize = 0;
    let mut points: HashSet<(usize, usize)> = input.lines().enumerate().take_while(|(n, x)| {
            point_end = *n;
            x.trim().len() > 0
        }).map(|(_, s)| {
        let mut i = s.split(',');
        (
            i.next().unwrap().parse().unwrap(),
            i.next().unwrap().parse().unwrap(),
        )
    }).collect();
    let instructions: Vec<(FoldDir, usize)> = input.lines().skip(point_end+1).map(|x| {
        (
            match x.split(&[' ', '=']).skip(2).next().unwrap() {
                "x" => FoldDir::Left,
                "y" => FoldDir::Up,
                _ => panic!("Unexpected fold instruction"),
            }, 
            x.split(&[' ', '=']).skip(3).next().unwrap().parse().unwrap()
        )
    }).collect();
    let (line1, dir1) = instructions.get(0).unwrap();
    fold_set(line1, dir1, &mut points);
    let ans_a = points.len();
    for (line, dir) in instructions.iter() {
        fold_set(line, dir, &mut points);
    }
    println!("Answer A: {}", ans_a);
    paprint_set(&points);
    println!("Computed in {}us", start.elapsed().as_micros());
    Ok(())
}

