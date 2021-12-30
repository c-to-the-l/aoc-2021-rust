use anyhow::Result;
use aoc_2021::get_input;
use std::collections::{HashMap, HashSet};

fn find_substr_by_len(s: &str, l: usize) -> HashSet<char> {
    s.split("|")
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .filter(|x| x.len() == l)
        .next()
        .unwrap()
        .chars()
        .collect()
}

fn find_all_substr_by_len(s: &str, l: usize) -> Vec<HashSet<char>> {
    s.split("|")
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .filter(|x| x.len() == l)
        .map(|x| x.chars().collect())
        .collect()
}

#[derive(Debug)]
struct SegmentMap {
    numbers: HashMap<usize, HashSet<char>>,
}

impl SegmentMap {
    fn from(s: &str) -> Self {
        let mut numbers = HashMap::new();

        let one = find_substr_by_len(&s, 2);
        let seven = find_substr_by_len(&s, 3);
        let four = find_substr_by_len(&s, 4);
        let eight = find_substr_by_len(&s, 7);

        let mut five_segs = find_all_substr_by_len(s, 5);
        let (three_pos, _) = five_segs
            .iter()
            .enumerate()
            .filter(|(_, val)| val.is_superset(&seven))
            .next()
            .unwrap();
        let three = five_segs.swap_remove(three_pos);
        assert_eq!(five_segs.len(), 2);
        let (two_pos, _) = five_segs
            .iter()
            .enumerate()
            .filter(|(_, val)| four.intersection(val).count() == 2)
            .next()
            .unwrap();
        let two = five_segs.swap_remove(two_pos);
        assert_eq!(five_segs.len(), 1);
        let five = five_segs.pop().unwrap();

        let mut six_segs = find_all_substr_by_len(s, 6);
        let (nine_pos, _) = six_segs
            .iter()
            .enumerate()
            .filter(|(_, val)| val.is_superset(&four))
            .next()
            .unwrap();
        let nine = six_segs.swap_remove(nine_pos);
        let (zero_pos, _) = six_segs
            .iter()
            .enumerate()
            .filter(|(_, val)| val.is_superset(&seven))
            .next()
            .unwrap();
        let zero = six_segs.swap_remove(zero_pos);
        let six = six_segs.pop().unwrap();

        numbers.insert(1, one);
        numbers.insert(2, two); // five segments
        numbers.insert(3, three);
        numbers.insert(4, four);
        numbers.insert(5, five); // five segments
        numbers.insert(6, six); // six segments
        numbers.insert(7, seven);
        numbers.insert(8, eight);
        numbers.insert(9, nine); // six segments
        numbers.insert(0, zero); // six segments

        Self { numbers }
    }

    fn get_seg_for(&self, seg: &HashSet<char>) -> usize {
        let mut maybe_seg = self.numbers.iter().filter(|(_, val)| &seg == val);
        let (rv, _) = maybe_seg.next().unwrap();
        assert_eq!(maybe_seg.count(), 0);
        rv.clone()
    }

    fn get_segs(&self, s: &str) -> usize {
        let to_find: Vec<HashSet<char>> = s
            .split("|")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|x| x.chars().collect())
            .collect();

        let as_str: String = to_find
            .iter()
            .map(|x| format!("{}", self.get_seg_for(x)))
            .collect();
        as_str.parse().unwrap()
    }
}

fn main() -> Result<()> {
    let input = get_input(2021, 8)?;
    let start = std::time::Instant::now();
    let a_lens: HashSet<usize> = HashSet::from([2, 4, 3, 7]);
    let answer_a: usize = input
        .lines()
        .map(|a| {
            a.split("|")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .filter(|b| a_lens.contains(&b.len()))
        })
        .flatten()
        .count();
    let answer_b: usize = input.lines().map(|x| SegmentMap::from(x).get_segs(x)).sum();
    println!("Answer A: {}", answer_a);
    println!("Answer B {}", answer_b);
    println!("Computed in {}us", start.elapsed().as_micros());
    Ok(())
}
