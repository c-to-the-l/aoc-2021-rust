use aoc_2021::get_input;
use anyhow::Result;


fn main() -> Result<()> {
    let t_start = std::time::Instant::now();
    let input = get_input(2021, 25)?;
    let mut cucumbers: Vec<Vec<u8>> = input.lines().map(|l| {
        l.chars().map(|c| {
            match c {
                '.' => 0,
                '>' => 1,
                'v' => 2,
                x => panic!("Unexpected token in input: {}", x),
            }
        }).collect()
    }).collect();

    let xl = cucumbers[0].len();
    let yl = cucumbers.len();

    let mut any_change: bool = true;
    let mut east_movers: Vec<(usize, usize)> = Vec::new();
    let mut south_movers: Vec<(usize, usize)> = Vec::new();
    let mut count = 0;
    while any_change {
        any_change = false;

        for x in 0..xl {
            for y in 0..yl {
                if cucumbers[y][x] == 1 && cucumbers[y][(x+1) % xl] == 0 {
                    east_movers.push((x, y));
                }
            }
        }
        for (xm, ym) in east_movers.drain(..) {
            any_change = true;
            cucumbers[ym][xm] = 0;
            cucumbers[ym][(xm+1) % xl] = 1;
        }
        for x in 0..xl {
            for y in 0..yl {
                if cucumbers[y][x] == 2 && cucumbers[(y+1) % yl][x] == 0 {
                    south_movers.push((x, y));
                }
            }
        }
        for (xm, ym) in south_movers.drain(..) {
            any_change = true;
            cucumbers[ym][xm] = 0;
            cucumbers[(ym+1) % yl][xm] = 2;
        }
        count += 1;
        
        println!("{}", count);
    }

    println!("Answer A: {}", count);
    println!("No answer B for the last puzzle!");

    println!("Computed in {}us", t_start.elapsed().as_micros());
    Ok(())
}
