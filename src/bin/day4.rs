use aoc_2021::get_input;
use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
struct BingoBoard {
    numbers: Vec<Vec<i32>>,
    marked: Vec<Vec<bool>>,
}

impl BingoBoard {
    fn new(inputs: &Vec<&str>) -> Self {
        let numbers = inputs
            .iter()
            .map(|x| {
                x.trim()
                    .split_whitespace()
                    .map(|y| y.parse().unwrap())
                    .collect()
            })
            .collect();
        let marked = inputs
            .iter()
            .map(|x| x.trim().split_whitespace().map(|_| false).collect())
            .collect();
        BingoBoard { numbers, marked }
    }

    fn has_number(&self, val: &i32) -> Option<(usize, usize)> {
        for (row, nums) in self.numbers.iter().enumerate() {
            for (col, num) in nums.iter().enumerate() {
                if *num == *val {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn mark(&mut self, row: usize, col: usize) {
        let v = self.marked.get_mut(row).unwrap().get_mut(col).unwrap();
        *v = true;
    }

    fn mark_if_present(&mut self, val: &i32) -> bool {
        if let Some((row, col)) = self.has_number(val) {
            self.mark(row, col);
            true
        } else {
            false
        }
    }

    fn check_col(&self, col: usize) -> bool {
        self.marked.iter().map(|x| x.get(col).unwrap()).all(|y| *y)
    }

    fn cols_win(&self) -> bool {
        let col_len = self.marked.get(0).unwrap().len();
        for i in 0..col_len {
            if self.check_col(i) {
                return true;
            }
        }
        false
    }

    fn has_won(&self) -> bool {
        self.marked.iter().any(|x| x.iter().all(|y| *y)) || self.cols_win()
    }

    fn sum_unmarked(&self) -> i32 {
        self.numbers
            .iter()
            .zip(self.marked.iter())
            .fold(0, |prev, (vals, marks)| {
                prev + vals.iter().zip(marks.iter()).fold(
                    0,
                    |prev, (val, mark)| {
                        if *mark {
                            prev
                        } else {
                            prev + *val
                        }
                    },
                )
            })
    }
}

trait BingoBoards {
    fn pop_winner(&mut self) -> Option<BingoBoard>;
    fn stamp(&mut self, val: &i32) -> bool;
}

impl BingoBoards for Vec<BingoBoard> {
    fn pop_winner(&mut self) -> Option<BingoBoard> {
        for (i, val) in self.iter_mut().enumerate() {
            if val.has_won() {
                return Some(self.remove(i));
            }
        }
        None
    }

    fn stamp(&mut self, val: &i32) -> bool {
        let rv: Vec<bool> = self.iter_mut().map(|x| x.mark_if_present(val)).collect();
        rv.iter().any(|x| *x)
    }
}

fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let input = get_input(2021, 4)?;
    let mut lines = input.lines();
    let calls: Vec<i32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut boards: Vec<BingoBoard> = Vec::new();
    for (_, group) in &lines.group_by(|x| x.trim().is_empty()) {
        let board_strs: Vec<&str> = group.collect();
        if board_strs.len() > 1 {
            boards.push(BingoBoard::new(&board_strs))
        }
    }

    'outer: for val in calls.iter() {
        for board in boards.iter_mut() {
            if board.mark_if_present(val) {
                if board.has_won() {
                    let board_sum = board.sum_unmarked();
                    println!("Answer A - Result {} x {} = {}", val, board_sum, val * board_sum);
                    break 'outer;
                }
            }
        }
    }

    'outer2: for val in calls.iter() {
        if boards.stamp(val) {
            while let Some(x) = boards.pop_winner() {
                if boards.len() < 1 {
                    println!(
                        "Answer B - {} x {} = {}",
                        x.sum_unmarked(),
                        val,
                        x.sum_unmarked() * val
                    );
                    break 'outer2;
                }
            }
        }
    }
    println!("Computed in {}us", start.elapsed().as_micros());
    Ok(())
}
