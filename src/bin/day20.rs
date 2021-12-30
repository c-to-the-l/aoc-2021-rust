use aoc_2021::get_input;
use anyhow::Result;


trait Get3x3 {
    fn get_default(&self, x: i64, y: i64, default: bool) -> bool;
    fn get_3x3(&self, x: i64, y: i64, default: bool) -> usize;
}

impl Get3x3 for Vec<Vec<bool>> {
    fn get_default(&self, x: i64, y: i64, default: bool) -> bool {
        if x < 0 || y < 0 {
            return default;
        }
        let x = x as usize;
        let y = y as usize;
        if let Some(r) = self.get(y) {
            if let Some(v) = r.get(x) {
                return *v;
            }
        }
        default
    }

    fn get_3x3(&self, x: i64, y: i64, default: bool) -> usize {
        (self.get_default(x - 1, y - 1, default) as usize) << 8
            | (self.get_default(x, y - 1, default) as usize) << 7
            | (self.get_default(x + 1, y - 1, default) as usize) << 6
            | (self.get_default(x - 1, y, default) as usize) << 5
            | (self.get_default(x, y, default) as usize) << 4
            | (self.get_default(x + 1, y, default) as usize) << 3
            | (self.get_default(x - 1, y + 1, default) as usize) << 2
            | (self.get_default(x, y + 1, default) as usize) << 1
            | (self.get_default(x + 1, y + 1, default) as usize)
    }
}

fn pad(i: &mut Vec<Vec<bool>>, n: usize) {
    let new_wid = i[0].len() + (2 * n);
    for j in 0..i.len() {
        for _ in 0..n {
            i[j].insert(0, false);
            i[j].push(false);
        }
    }
    for _ in 0..n {
        i.insert(0, vec![false; new_wid]);
        i.push(vec![false; new_wid]);
    }
}



fn main() -> Result<()> {
    let t_start = std::time::Instant::now();
    let input = get_input(2021, 20)?;
    let algo: Vec<bool> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| if x == '#' { true } else { false })
        .collect();
    let mut image: Vec<Vec<bool>> = input
        .lines()
        .skip(2)
        .map(|x| {
            x.chars()
                .map(|y| if y == '#' { true } else { false })
                .collect()
        })
        .collect();
    pad(&mut image, 10);
    let out_a: Vec<Vec<bool>> = (1..=2).fold(image, |prev, iter| {
        let default = if iter % 2 == 0 { true } else { false };
        (-1..=prev.len() as i64)
            .map(|y| {
                (-1..=prev[0].len() as i64)
                    .map(|x| {
                        algo[prev.get_3x3(x, y, default)]
                    })
                    .collect()
            })
            .collect()
    });
    let ans_a: usize = out_a.iter().map(|r| r.iter().filter(|v| **v).count()).sum();
    let out_b: Vec<Vec<bool>> = (3..=50).fold(out_a, |prev, iter| {
        let default = if iter % 2 == 0 { true } else { false };
        (-1..=prev.len() as i64)
            .map(|y| {
                (-1..=prev[0].len() as i64)
                    .map(|x| {
                        algo[prev.get_3x3(x, y, default)]
                    })
                    .collect()
            })
            .collect()
    });
    let ans_b: usize = out_b.iter().map(|r| r.iter().filter(|v| **v).count()).sum();
    println!("Answer A: {}", ans_a);
    println!("Answer B: {}", ans_b);
    println!("Computed in {}us", t_start.elapsed().as_micros());
    Ok(())
}
