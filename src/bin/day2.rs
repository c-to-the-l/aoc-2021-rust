use aoc_2021::get_input;
use anyhow::Result;

fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let input = get_input(2021, 2)?;
    let (horizontal_1, vertical_1): (i32, i32) = input.lines().fold((0, 0), |(horiz, vert), y| {
        let (dir, distance) = y.split_once(" ").unwrap();
        let distance: i32 = distance.parse().unwrap();
        match dir {
            "forward" => (horiz + distance, vert),
            "up" => (horiz, vert - distance),
            "down" => (horiz, vert + distance),
            x => panic!("Unexpected input {}", x),
        }
    });

    let (_, horiz_2, vert_2): (i64, i64, i64) =
        input.lines().fold((0, 0, 0), |(aim, horiz, vert), y| {
            let (dir, distance) = y.split_once(" ").unwrap();
            let distance: i64 = distance.parse().unwrap();
            match dir {
                "forward" => (aim, horiz + distance, vert + (distance * aim)),
                "up" => (aim - distance, horiz, vert),
                "down" => (aim + distance, horiz, vert),
                x => panic!("Unexpected input {}", x),
            }
        });
    println!("Answer A {}", horizontal_1 * vertical_1);
    println!("Answer B {}", horiz_2 * vert_2);
    println!("Computed in {}us", start.elapsed().as_micros());

    Ok(())
}
