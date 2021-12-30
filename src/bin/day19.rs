use aoc_2021::get_input;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;

fn rotate(n: usize, i: &HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    match n {
        // identity
        0 => i.clone(),

        // x axis rotations
        1 => i.iter().map(|(x, y, z)| (*x, -z, *y)).collect(),
        2 => i.iter().map(|(x, y, z)| (*x, -y, -z)).collect(),
        3 => i.iter().map(|(x, y, z)| (*x, *z, -y)).collect(),
        // y axis rotations
        4 => i.iter().map(|(x, y, z)| (*z, *y, -x)).collect(),
        5 => i.iter().map(|(x, y, z)| (-x, *y, -z)).collect(),
        6 => i.iter().map(|(x, y, z)| (-z, *y, *x)).collect(),
        // z axis rotations
        7 => i.iter().map(|(x, y, z)| (*y, -x, *z)).collect(),
        8 => i.iter().map(|(x, y, z)| (-x, -y, *z)).collect(),
        9 => i.iter().map(|(x, y, z)| (-y, *x, *z)).collect(),

        //edge rotations
        10 => i.iter().map(|(x, y, z)| (-x, -z, -y)).collect(),
        11 => i.iter().map(|(x, y, z)| (*z, -y, *x)).collect(), 
        12 => i.iter().map(|(x, y, z)| (-x, *z, *y)).collect(), 
        13 => i.iter().map(|(x, y, z)| (-z, -y, -x)).collect(), 
        14 => i.iter().map(|(x, y, z)| (*y, *x, -z)).collect(), 
        15 => i.iter().map(|(x, y, z)| (-y, -x, -z)).collect(), 

        // corner rotations
        16 => i.iter().map(|(x, y, z)| (*z, *x, *y)).collect(),
        17 => i.iter().map(|(x, y, z)| (*y, *z, *x)).collect(),
        18 => i.iter().map(|(x, y, z)| (*y, -z, -x)).collect(),
        19 => i.iter().map(|(x, y, z)| (-z, *x, -y)).collect(),

        20 => i.iter().map(|(x, y, z)| (-y, *z, -x)).collect(),
        21 => i.iter().map(|(x, y, z)| (-z, -x, *y)).collect(),
        22 => i.iter().map(|(x, y, z)| (*z, -x, -y)).collect(),
        23 => i.iter().map(|(x, y, z)| (-y, -z, *x)).collect(),
        x => panic!("Not a rotation: {}", x),
    }
}

#[derive(Debug)]
struct Scanner {
    position: Option<(i32, i32, i32)>,
    b_rel: HashSet<(i32, i32, i32)>,
    b_abs: HashSet<(i32, i32, i32)>,
}

impl Scanner {
    fn from_beacons(s: HashSet<(i32, i32, i32)>) -> Scanner {
        Scanner {
            position: None,
            b_rel: s,
            b_abs: HashSet::new(),
        }
    }

    fn is_anchor(&mut self) {
        self.position = Some((0, 0, 0));
        self.b_abs = self.b_rel.clone();
    }

    fn set_abs(&mut self, r: usize, offset: (i32, i32, i32)) {
        self.b_rel = rotate(r, &self.b_rel);
        self.position = Some(offset);
        let (ox, oy, oz) = offset;
        self.b_abs = self
            .b_rel
            .iter()
            .map(|(x, y, z)| (ox + x, oy + y, oz + z))
            .collect();
    }

    fn find_overlap(&self, s: &HashSet<(i32, i32, i32)>) -> Option<(usize, (i32, i32, i32))> {
        for r in 0..24 {
            let rota = rotate(r, &self.b_rel);
            let mut diff = s
                .iter()
                .map(|(ax, ay, az)| {
                    rota.iter()
                        .map(move |(bx, by, bz)| (ax - bx, ay - by, az - bz))
                })
                .flatten()
                .counts();
            diff.retain(|_, v| *v > 11);
            if diff.len() == 1 {
                let (k, _) = diff.drain().next().unwrap();
                return Some((r, k));
            }
            if diff.len() > 1 {
                panic!("Multiple valid rotations found for {:?} {:?}", self, s);
            }
        }

        None
    }

    fn manhattan(&self, other: &Self) -> i32 {
        if let Some((ax, ay, az)) = self.position {
            if let Some((bx, by, bz)) = other.position {
                (ax - bx).abs() + (ay - by).abs() + (az - bz).abs()
            } else {
                0
            }
        } else {
            0
        }
    }
}

fn main() -> Result<()> {
    let t_start = std::time::Instant::now();
    let input = get_input(2021, 19)?;
    let mut lines = input.lines();
    let mut scanners: Vec<Scanner> = Vec::new();
    while let Some(s) = lines.next() {
        if s.starts_with("--") {
            let mut beacons = HashSet::new();
            while let Some(ls) = lines.next() {
                if ls.trim().len() == 0 {
                    break;
                }
                let mut vi = ls.split(',');
                let x = vi.next().unwrap().parse().unwrap();
                let y = vi.next().unwrap().parse().unwrap();
                let z = vi.next().unwrap().parse().unwrap();
                beacons.insert((x, y, z));
            }
            scanners.push(Scanner::from_beacons(beacons));
        }
    }

    let mut abs_scanners: Vec<Scanner> = Vec::new();
    abs_scanners.push(scanners.swap_remove(0));
    abs_scanners[0].is_anchor();
    let mut known_points: HashSet<(i32, i32, i32)> = abs_scanners[0].b_abs.clone();

    loop {
        let mut swap: Option<usize> = None;

        for (n, scanner) in scanners.iter_mut().enumerate() {
            if let Some((rot, offset)) = scanner.find_overlap(&known_points) {
                swap = Some(n);
                scanner.set_abs(rot, offset);
                break;
            }
        }

        if let Some(n) = swap {
            let scanner = scanners.swap_remove(n);
            known_points = &known_points | &scanner.b_abs;
            abs_scanners.push(scanner);
        } else {
            break;
        }
    }
    let ans_a = known_points.len();

    let ans_b = (0..24)
        .filter_map(|a| {
            (a + 1..24)
                .map(|b| abs_scanners[a].manhattan(&abs_scanners[b]))
                .max()
        })
        .max()
        .unwrap();
    println!("Answer A: {}", ans_a);
    println!("Answer B: {}", ans_b);

    println!("Computed in {}us", t_start.elapsed().as_micros());
    Ok(())
}
