use anyhow::Result;
use aoc_2021::get_input;

fn search_at(map: &Vec<Vec<usize>>, searched: &mut Vec<Vec<bool>>, x: usize, y: usize, depth: usize) -> usize {
    if y >= map.len() || x >= map.get(0).unwrap().len() {
        return 0;
    }
    let marker = searched.get_mut(y).unwrap().get_mut(x).unwrap();
    if *marker {
        return 0;
    } else {
        *marker = true;
    }
    let val = map.get(y).unwrap().get(x).unwrap();
    if *val >= 9 {
        return 0;
    }
    let up = if y > 0 {
        search_at(map, searched, x, y - 1, depth+1)
    } else {
        0
    };
    let left = if x > 0 {
        search_at(map, searched, x - 1, y, depth+1)
    } else {
        0
    };
    let right = search_at(map, searched, x + 1, y, depth+1);
    let down = search_at(map, searched, x, y + 1, depth+1);
    1 + up + left + right + down
}

fn maybe_search(
    map: &Vec<Vec<usize>>,
    searched: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
) -> Option<usize> {
    if *searched.get(y).unwrap().get(x).unwrap() {
        None
    } else {
        Some(search_at(map, searched, x, y, 0))
    }
}

fn main() -> Result<()> {
    let t_start = std::time::Instant::now();
    let input = get_input(2021, 9)?;
    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let mut sum_a: usize = 0;
    for (x, row) in map.iter().enumerate() {
        //println!("{:?}", row);
        for (y, val) in row.iter().enumerate() {
            let up_val: usize = if x > 0 {
                if let Some(prev_row) = map.get(x - 1) {
                    if let Some(up_val) = prev_row.get(y) {
                        *up_val
                    } else {
                        10
                    }
                } else {
                    10
                }
            } else {
                10
            };
            let down_val: usize = if let Some(next_row) = map.get(x + 1) {
                if let Some(down_val) = next_row.get(y) {
                    *down_val
                } else {
                    10
                }
            } else {
                10
            };

            let left_val = if y > 0 {
                if let Some(left_val) = row.get(y - 1) {
                    *left_val
                } else {
                    10
                }
            } else {
                10
            };
            let right_val = if let Some(right_val) = row.get(y + 1) {
                *right_val
            } else {
                10
            };

            if *val < up_val && *val < down_val && *val < left_val && *val < right_val {
                sum_a += val + 1;
            }
        }
    }
    let y_len = map.len();
    let x_len = map.get(0).unwrap().len();
    let mut searched: Vec<Vec<bool>> = vec![vec![false; x_len + 1]; y_len + 1];
    let mut results_b: Vec<usize> = Vec::new();
    for y in 0..y_len {
        for x in 0..x_len {
            if let Some(val) = maybe_search(&map, &mut searched, x, y) {
                results_b.push(val);
            }
        }
    }
    results_b.sort();
    results_b.reverse();

    println!("Answer A {}", sum_a);
    println!("Answer B {}", results_b[..3].iter().product::<usize>());
    println!("Computed in {}us", t_start.elapsed().as_micros());
    Ok(())
}
