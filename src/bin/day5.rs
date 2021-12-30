use aoc_2021::get_input;
use anyhow::Result;

trait VentParser {
    fn parse_vent(&self) -> Option<(usize, usize, usize, usize)>;
}

impl VentParser for &str {
    fn parse_vent(&self) -> Option<(usize, usize, usize, usize)> {
        let mut halves = self.split(" -> ");
        let mut firsthalf = halves.next()?.split(",");
        let mut secondhalf = halves.next()?.split(",");
        Some((
            firsthalf.next()?.parse().unwrap(),
            firsthalf.next()?.parse().unwrap(),
            secondhalf.next()?.parse().unwrap(),
            secondhalf.next()?.parse().unwrap(),
        ))
    }
}

fn fold_max_coords(a: (usize, usize), b: &(usize, usize, usize, usize)) -> (usize, usize) {
    let (x1, y1) = a;
    let (x2, y2, x3, y3) = *b;
    let x = if x1 > x2 && x1 > x3 {
        x1
    } else if x2 > x3 {
        x2
    } else {
        x3
    };
    let y = if y1 > y2 && y1 > y3 {
        y1
    } else if y2 > y3 {
        y2
    } else {
        y3
    };
    (x, y)
}

fn get_bounds(v: &Vec<(usize, usize, usize, usize)>) -> (usize, usize) {
    v.iter().fold((0, 0), fold_max_coords)
}

fn is_straight(v: &&(usize, usize, usize, usize)) -> bool {
    let (a, b, c, d) = v;
    a == c || b == d
}

fn is_not_straight(v: &&(usize, usize, usize, usize)) -> bool {
    !is_straight(v)
}

fn make_ud_lr(v: &(usize, usize, usize, usize)) -> (usize, usize, usize, usize) {
    let (a, b, c, d) = *v;
    if a == c {
        if b > d {
            (c, d, a, b)
        } else {
            (a, b, c, d)
        }
    } else if b == d {
        if a > c {
            (c, d, a, b)
        } else {
            (a, b, c, d)
        }
    } else {
        panic!("make_ud_lr given non-cardinal input");
    }
}

trait VentMap {
    fn mark_vent(&mut self, v: &(usize, usize, usize, usize));
    fn mark_diag_vent(&mut self, v: &(usize, usize, usize, usize));
    fn inc_point(&mut self, x: usize, y: usize);
    fn sum_intersections(&self) -> usize;
    fn from_mapsize(x_max: usize, y_max: usize) -> Self;
}

impl VentMap for Vec<Vec<usize>> {
    fn mark_vent(&mut self, v: &(usize, usize, usize, usize)) {
        let (a, b, c, d) = *v;
        if b == d {
            for x in a..=c {
                self.inc_point(x, b)
            }
        } else if a == c {
            for y in b..=d {
                self.inc_point(a, y)
            }
        } else {
            panic!("mark_vent given non-cardinal input")
        }
    }
    fn mark_diag_vent(&mut self, v: &(usize, usize, usize, usize)) {
        let (a, b, c, d) = *v;
        if a < c && b < d {
            for (x, y) in (a..=c).zip(b..=d) {
                self.inc_point(x, y);
            }
        } else if a > c && b < d {
            for (x, y) in (c..=a).rev().zip(b..=d) {
                self.inc_point(x, y);
            }
        } else if a < c && b > d {
            for (x, y) in (a..=c).zip((d..=b).rev()) {
                self.inc_point(x, y);
            }
        } else if a > c && b > d {
            for (x, y) in (c..=a).rev().zip((d..=b).rev()) {
                self.inc_point(x, y);
            }
        } else {
            panic!("What?");
        }
    }
    fn inc_point(&mut self, x: usize, y: usize) {
        *(self.get_mut(y).unwrap().get_mut(x).unwrap()) += 1
    }
    fn from_mapsize(x_max: usize, y_max: usize) -> Self {
        vec![vec![0; x_max + 1]; y_max + 1]
    }
    fn sum_intersections(&self) -> usize {
        self.iter().fold(0, |prev, v| {
            prev + v
                .iter()
                .fold(0, |prev, point| if *point > 1 { prev + 1 } else { prev })
        })
    }
}

fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let input = get_input(2021, 5)?;
    let vents: Vec<(usize, usize, usize, usize)> =
        input.lines().map(|x| x.parse_vent().unwrap()).collect();
    let straight_vents: Vec<(usize, usize, usize, usize)> =
        vents.iter().filter(is_straight).map(make_ud_lr).collect();
    let diag_vents: Vec<(usize, usize, usize, usize)> =
        vents.iter().filter(is_not_straight).cloned().collect();
    let (x_max, y_max) = get_bounds(&vents);
    let mut vent_map: Vec<Vec<usize>> = VentMap::from_mapsize(x_max, y_max);
    for vent in straight_vents.iter() {
        vent_map.mark_vent(vent);
    }
    println!("Answer 1: {}", vent_map.sum_intersections());
    for vent in diag_vents.iter() {
        vent_map.mark_diag_vent(vent);
    }
    println!("Answer 2: {}", vent_map.sum_intersections());
    println!("Computed in {}us", start.elapsed().as_micros());
    Ok(())
}
