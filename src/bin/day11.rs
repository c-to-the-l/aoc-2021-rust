use aoc_2021::get_input;
use anyhow::Result;

struct Octopuses {
    width: usize,
    height: usize,
    energy: Vec<usize>,
    done_flash: Vec<bool>,
}

impl Octopuses {
    fn new(s: &str) -> Result<Self> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let energy = s
            .lines()
            .map(|x| x.chars())
            .flatten()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect();
        let done_flash = vec![false; width*height];
        Ok(Octopuses {
            width,
            height,
            energy,
            done_flash,
        })
    }

    fn fallout_flash(&mut self, n: usize) {
        let up = n >= self.width;
        let left = n % self.width != 0;
        let right = n % self.width != (self.width-1);
        let down = n + self.width < (self.width * self.height);
        // can go up
        if up {
            *self.energy.get_mut(n-self.width).unwrap() += 1;
            // can go up-left
            if left {
                *self.energy.get_mut(n-self.width-1).unwrap() += 1;
            }
            // can go up-right
            if right {
                *self.energy.get_mut(n-self.width+1).unwrap() += 1;
            }
        }
        // can go down
        if down {
            *self.energy.get_mut(n+self.width).unwrap() += 1;
            // can go down-left
            if left {
                *self.energy.get_mut(n+self.width-1).unwrap() += 1;
            }
            // can go down-right
            if right {
                *self.energy.get_mut(n+self.width+1).unwrap() += 1;
            }
        }
        // can go left
        if left {
            *self.energy.get_mut(n-1).unwrap() += 1
        }
        // can go right
        if right {
            *self.energy.get_mut(n+1).unwrap() += 1
        }

    }

    fn tick(&mut self) {
        for i in self.energy.iter_mut() {
            *i += 1;
        }
        let mut further_changes = true;
        while further_changes {
            further_changes = false;
            for n in 0..self.energy.len() {
                if self.energy.get(n).unwrap() > &9 {
                    if self.done_flash.get(n) == Some(&false) {
                        further_changes = true;
                        self.fallout_flash(n);
                        *(self.done_flash.get_mut(n).unwrap()) = true;
                    }
                }
            }
        }
        self.done_flash.fill(false);
    }
    fn count_flash_reset(&mut self) -> usize {
        let mut rv: usize = 0;
        for n in self.energy.iter_mut() {
            if *n > 9 {
                *n = 0;
                rv += 1;
            }
        }
        rv
    }
}

fn main() -> Result<()> {
    let input = get_input(2021, 11)?;
    let start = std::time::Instant::now();
    let mut octopuses = Octopuses::new(&input)?;
    let mut answer_a: usize = 0;
    for _ in 0..100 {
        octopuses.tick();
        answer_a += octopuses.count_flash_reset();
    }
    let mut answer_b: usize = 100;
    loop {
        answer_b += 1;
        octopuses.tick();
        if octopuses.count_flash_reset() >= 100 {
            break;
        }
    }
    println!("Answer A: {}", answer_a);
    println!("Answer B: {}", answer_b);

    println!("Computed in {}us", start.elapsed().as_micros());
    Ok(())
}
