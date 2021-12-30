use aoc_2021::get_input;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum CaveType {
    Start, 
    End, 
    Big, 
    Small,
}

#[derive(Debug)]
struct Cave {
    ty: CaveType,
    edges: Vec<usize>,
}

fn walk_b(caves: &Vec<Cave>, paths: &mut usize, path: &mut Vec<usize>, current: usize, small_visited: bool) {
    let me = caves.get(current).unwrap();
    path.push(current);
    if matches!(me.ty, CaveType::End) {
        *paths += 1;
    } else {
        for n in me.edges.iter() {
            let next = caves.get(*n).unwrap();
            if matches!(next.ty, CaveType::Start) {
                continue;
            }
            if small_visited {
                if matches!(next.ty, CaveType::Small) && path.contains(n) {
                    continue;
                }
                walk_b(caves, paths, path, *n, true);
            } else {
                if matches!(next.ty, CaveType::Small) && path.contains(n) {
                    walk_b(caves, paths, path, *n, true);
                } else {
                    walk_b(caves, paths, path, *n, false);
                }
            }
        }
    }
    path.pop();
}

fn main() -> Result<()> {
    let t_start = std::time::Instant::now();
    let input = get_input(2021, 12)?;
    let cave_names: HashSet<&str> = input.split(|x: char| !x.is_alphanumeric()).collect();
    let mut names_to_numbers: HashMap<&str, usize> = HashMap::new();
    let mut caves: Vec<Cave> = Vec::new();
    let mut start = 0;
    for (n, name) in cave_names.iter().enumerate() {
        names_to_numbers.insert(name.clone(), n);

        if *name == "start" {
            start = n;
            caves.push(Cave {
                ty: CaveType::Start,
                edges: Vec::new(),
            });
        } else if *name == "end" {
            caves.push(Cave{
                ty: CaveType::End,
                edges: Vec::new(),
            })
        } else if name.chars().all(char::is_uppercase) {
            caves.push(Cave{
                ty: CaveType::Big,
                edges: Vec::new(),
            })
        } else if name.chars().all(char::is_lowercase) {
            caves.push(Cave{
                ty: CaveType::Small,
                edges: Vec::new(),
            })
        }
    }
    for l in input.lines() {
        let mut i = l.split("-");
        let left = names_to_numbers.get(i.next().unwrap()).unwrap();
        let right = names_to_numbers.get(i.next().unwrap()).unwrap();
        let lcave = caves.get_mut(*left).unwrap();
        lcave.edges.push(right.clone());
        let rcave = caves.get_mut(*right).unwrap();
        rcave.edges.push(left.clone());
    }
    let mut paths = 0;
    let mut path = Vec::new();
    path.reserve(50);
    walk_b(&caves, &mut paths, &mut path, start, true);
    println!("Answer A: {}", paths);
    paths = 0;
    path.clear();
    walk_b(&caves, &mut paths, &mut path, start, false);
    println!("Answer B: {}", paths);
    println!("Computed in {}us", t_start.elapsed().as_micros());
    Ok(())
}
