use aoc_2021::get_input;
use anyhow::Result;
use std::collections::{HashMap};

trait IncrementOrInsert {
    fn insert_add(&mut self, k: &(char, char), v: u128);
}

impl IncrementOrInsert for HashMap<(char, char), u128> {
    fn insert_add(&mut self, k: &(char, char), v: u128) {
        if let Some(x) = self.get_mut(k) {
            *x += v;
        } else {
            self.insert(*k, v);
        }
    }
}


fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let input = get_input(2021, 14)?;
    let line1 =  input.lines().next().unwrap();
    let mut init_map: HashMap<(char, char), u128> = HashMap::new();
    for (p, n) in line1.chars().zip(line1.chars().skip(1)) {
        init_map.insert_add(&(p, n), 1);
    }
    let pair_map: HashMap<(char, char), ((char, char), (char, char))> = input
        .lines()
        .skip(2)
        .map(|s| {
            let p = s.chars().next().unwrap();
            let n = s.chars().skip(1).next().unwrap();
            let m = s.chars().skip(6).next().unwrap();
            ((p, n), ((p, m), (m, n)))
        })
        .collect();

    let result: HashMap<(char, char), u128> = (0..10).fold(init_map.clone(), |prev, _| {
        let mut next = HashMap::new();
        for (k, v) in prev.iter() {
            let (l, r) = pair_map.get(k).unwrap();
            next.insert_add(l, *v);
            next.insert_add(r, *v);
        };
        next
    });
    let mut character_values: HashMap<char, u128> = HashMap::new();
    for ((l, _), v) in result.iter() {
        if let Some(x) = character_values.get_mut(l) {
            *x += v;
        } else {
            character_values.insert(*l, *v);
        }
    }
    if let Some(x) = character_values.get_mut(&line1.chars().last().unwrap()) {
        *x += 1;
    } else {
        character_values.insert(line1.chars().last().unwrap(), 1);
    }

    println!("Answer A {}-{} = {}", character_values.values().max().unwrap(), character_values.values().min().unwrap(), character_values.values().max().unwrap()-character_values.values().min().unwrap());

    let result: HashMap<(char, char), u128> = (0..40).fold(init_map, |prev, _| {
        let mut next = HashMap::new();
        for (k, v) in prev.iter() {
            let (l, r) = pair_map.get(k).unwrap();
            next.insert_add(l, *v);
            next.insert_add(r, *v);
        };
        next
    });
    let mut character_values: HashMap<char, u128> = HashMap::new();
    for ((l, _), v) in result.iter() {
        if let Some(x) = character_values.get_mut(l) {
            *x += v;
        } else {
            character_values.insert(*l, *v);
        }
    }
    if let Some(x) = character_values.get_mut(&line1.chars().last().unwrap()) {
        *x += 1;
    } else {
        character_values.insert(line1.chars().last().unwrap(), 1);
    }

    println!("Answer B {}-{} = {}", character_values.values().max().unwrap(), character_values.values().min().unwrap(), character_values.values().max().unwrap()-character_values.values().min().unwrap());

    println!("Computed in {}us", start.elapsed().as_micros());
    Ok(())
}
