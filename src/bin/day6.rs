use aoc_2021::get_input;
use anyhow::Result;

#[derive(Debug, Clone)]
struct LanternShoal {
    day8: usize,
    day7: usize,
    day6: usize,
    day5: usize,
    day4: usize,
    day3: usize,
    day2: usize,
    day1: usize,
    day0: usize,
}

impl LanternShoal {
    fn from_input(i: &str) -> Self {
        let mut day0: usize = 0;
        let mut day1: usize = 0;
        let mut day2: usize = 0;
        let mut day3: usize = 0;
        let mut day4: usize = 0;
        let mut day5: usize = 0;
        let mut day6: usize = 0;
        let mut day7: usize = 0;
        let mut day8: usize = 0;

        for num in i.split(",") {
            match num.trim() {
                "0" => day0 += 1,
                "1" => day1 += 1,
                "2" => day2 += 1,
                "3" => day3 += 1,
                "4" => day4 += 1,
                "5" => day5 += 1,
                "6" => day6 += 1,
                "7" => day7 += 1,
                "8" => day8 += 1,
                _ => panic!("Unexpected number {}",num),
            }
        }
        LanternShoal {
            day8,
            day7,
            day6,
            day5,
            day4,
            day3,
            day2,
            day1,
            day0,
        }
    }

    fn tick_once(&mut self) {
        let spawners = self.day0;
        self.day0 = self.day1;
        self.day1 = self.day2;
        self.day2 = self.day3;
        self.day3 = self.day4;
        self.day4 = self.day5;
        self.day5 = self.day6;
        self.day6 = self.day7;
        self.day7 = self.day8;
        self.day6 += spawners;
        self.day8 = spawners;
    }
    fn count(&self) -> usize {
        self.day0 +
        self.day1 +
        self.day2 +
        self.day3 +
        self.day4 +
        self.day5 +
        self.day6 +
        self.day7 +
        self.day8
    }
}

fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let input = get_input(2021, 6)?;
    let mut shoal = LanternShoal::from_input(&input);
    for _ in 0..80 {
        shoal.tick_once();
    }
    let answer1 = shoal.count();
    for _ in 0..176 {
        shoal.tick_once()
    }
    let answer2 = shoal.count();
    println!("Answer A {}", answer1);
    println!("Answer B {}", answer2);
    println!("Computed in {}us", start.elapsed().as_micros());
    Ok(())
}
