use aoc_2021::get_input;
use anyhow::Result;


fn main() -> Result<()> {
    let t_start = std::time::Instant::now();
    let input = get_input(2021, 22)?;
    let mut xs: Vec<i64> = Vec::new();
    let mut ys: Vec<i64> = Vec::new();
    let mut zs: Vec<i64> = Vec::new();
    let mut cubes: Vec<(bool, i64, i64, i64, i64, i64, i64)> = Vec::new();

    for line in input.lines() {
        let mut items = line.split(&[' ',',','=','.']);
        let onoff = match items.next() {
            Some("on") => true,
            Some("off") => false,
            x => panic!("Unexpected onoff {:?}", x),
        };
        let x1: i64 = items.nth(1).unwrap().parse().unwrap();
        let x2: i64 = items.nth(1).unwrap().parse().unwrap();
        let y1: i64 = items.nth(1).unwrap().parse().unwrap();
        let y2: i64 = items.nth(1).unwrap().parse().unwrap();
        let z1: i64 = items.nth(1).unwrap().parse().unwrap();
        let z2: i64 = items.nth(1).unwrap().parse().unwrap();

        xs.push(x1);
        xs.push(x2+1);
        ys.push(y1);
        ys.push(y2+1);
        zs.push(z1);
        zs.push(z2+1);
        cubes.push((onoff, x1,x2,y1,y2,z1,z2));
    }

    xs.sort();
    ys.sort();
    zs.sort();

    let xs_50: Vec<i64> = xs.iter().filter(|x| **x > -51 && **x < 51).cloned().collect();
    let ys_50: Vec<i64> = ys.iter().filter(|y| **y > -51 && **y < 51).cloned().collect();
    let zs_50: Vec<i64> = zs.iter().filter(|z| **z > -51 && **z < 51).cloned().collect();

    let mut count_a: i64 = 0;

    for (x1, x2) in xs_50.iter().zip(xs_50[1..].iter()) {
        for (y1, y2) in ys_50.iter().zip(ys_50[1..].iter()) {
            for (z1, z2) in zs_50.iter().zip(zs_50[1..].iter()) {
                match cubes.iter().rev().filter(|(_,x_1,x_2,y_1,y_2,z_1,z_2)| {
                    x_1 <= x1 && x1 <= x_2 &&
                    y_1 <= y1 && y1 <= y_2 &&
                    z_1 <= z1 && z1 <= z_2
                }).next() {
                    Some((true, _,_,_,_,_,_)) => {
                        count_a += (*x2 - *x1) * (*y2 - *y1) * (*z2 - *z1)
                    },
                    _ => {}
                }
            }
        }
    }

    println!("Answer A:{}", count_a);
    println!("Warning, the next step will take a while;");
    println!("Up to 5 minutes on slow computers.");
    println!("Go make a cup of tea or something.");

    let mut count_b: i64 = 0;

    for (x1, x2) in xs.iter().zip(xs[1..].iter()) {
        for (y1, y2) in ys.iter().zip(ys[1..].iter()) {
            for (z1, z2) in zs.iter().zip(zs[1..].iter()) {
                match cubes.iter().rev().filter(|(_,x_1,x_2,y_1,y_2,z_1,z_2)| {
                    x_1 <= x1 && x1 <= x_2 &&
                    y_1 <= y1 && y1 <= y_2 &&
                    z_1 <= z1 && z1 <= z_2
                }).next() {
                    Some((true, _,_,_,_,_,_)) => {
                        count_b += (*x2 - *x1) * (*y2 - *y1) * (*z2 - *z1)
                    },
                    _ => {}
                }
            }
        }
    }

    println!("Answer B: {}", count_b);
    println!("Computed in {}us", t_start.elapsed().as_micros());
    Ok(())
}
