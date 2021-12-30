use aoc_2021::get_input;
use anyhow::Result;

#[derive(Clone)]
enum SFItem {
    Literal(usize),
    Number(Box<SFNumber>),
}

#[derive(Clone)]
struct SFNumber {
    left: SFItem,
    right: SFItem,
}

fn parse_sfnumber(s: &str) -> (usize, SFNumber) {
    assert_eq!(&s[0..1], "[");
    let mut sp: usize = 1;
    let left: SFItem = match &s[sp..=sp] {
        "[" => {
            let (n, l) = parse_sfnumber(&s[sp..]);
            sp += n;
            SFItem::Number(Box::new(l))
        }
        x => {
            sp += 1;
            SFItem::Literal(x.parse().unwrap())
        }
    };
    assert_eq!(&s[sp..=sp], ",");
    sp += 1;
    let right: SFItem = match &s[sp..=sp] {
        "[" => {
            let (n, l) = parse_sfnumber(&s[sp..]);
            sp += n;
            SFItem::Number(Box::new(l))
        }
        x => {
            sp += 1;
            SFItem::Literal(x.parse().unwrap())
        }
    };
    assert_eq!(&s[sp..=sp], "]");
    sp += 1;
    (sp, SFNumber { left, right })
}

fn add_sfnumber(left: SFNumber, right: SFNumber) -> SFNumber {
    let mut rv = SFNumber {
        left: SFItem::Number(Box::new(left)),
        right: SFItem::Number(Box::new(right)),
    };
    rv.reduce();
    rv
}

fn refadd_sfnumber(left: &SFNumber, right: &SFNumber) -> SFNumber {
    let mut rv = SFNumber {
        left: SFItem::Number(Box::new(left.clone())),
        right: SFItem::Number(Box::new(right.clone())),
    };
    rv.reduce();
    rv
}

impl SFNumber {
    fn from_str(s: &str) -> SFNumber {
        let (_, rv) = parse_sfnumber(s);
        rv
    }

    fn reduce(&mut self) {
        loop {
            let (b, _, _) = self.explode(0);
            if b {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn split(&mut self) -> bool {
        if self.left.split() {
            return true;
        }
        self.right.split()
    }

    fn explode(&mut self, depth: usize) -> (bool, Option<usize>, Option<usize>) {
        if depth >= 4 {
            if let SFItem::Literal(l) = self.left {
                if let SFItem::Literal(r) = self.right {
                    return (true, Some(l), Some(r));
                }
            }
        }

        let (b, l, r) = self.left.explode(depth + 1);
        if b {
            if let Some(r) = r {
                self.right.add_left(r);
                if l.is_some() {
                    self.left = SFItem::Literal(0);
                }
            }
            return (b, l, None);
        }
        

        let (b, l, r) = self.right.explode(depth + 1);
        if b {
            if let Some(l) = l {
                self.left.add_right(l);
                if r.is_some() {
                    self.right = SFItem::Literal(0);
                }
            }
            return (b, None, r);
        }
        assert_eq!(r, None);
        (false, None, None)
    }

    fn add_left(&mut self, n: usize) {
        self.left.add_left(n);
    }
    fn add_right(&mut self, n: usize) {
        self.right.add_right(n);
    }
    fn magnitude(&self) -> usize {
        (self.left.magnitude() * 3) + (self.right.magnitude() * 2)
    }
}

impl SFItem {
    fn add_left(&mut self, n: usize) {
        match self {
            SFItem::Literal(x) => *x += n,
            SFItem::Number(x) => x.add_left(n),
        }
    }
    fn add_right(&mut self, n: usize) {
        match self {
            SFItem::Literal(x) => *x += n,
            SFItem::Number(x) => x.add_right(n),
        }
    }
    fn explode(&mut self, depth: usize) -> (bool, Option<usize>, Option<usize>) {
        match self {
            SFItem::Literal(_) => (false, None, None),
            SFItem::Number(x) => x.explode(depth),
        }
    }
    fn split(&mut self) -> bool {
        match self {
            SFItem::Number(x) => x.split(),
            SFItem::Literal(x) => {
                if *x > 9 {
                    *self = SFItem::Number(Box::new(SFNumber {
                        left: SFItem::Literal(*x / 2),
                        right: SFItem::Literal(*x - *x / 2),
                    }));
                    true
                } else {
                    false
                }
            }
        }
    }
    fn magnitude(&self) -> usize {
        match self {
            SFItem::Literal(x) => *x,
            SFItem::Number(x) => x.magnitude(),
        }
    }
}

fn main() -> Result<()> {
    let t_start = std::time::Instant::now();
    let input = get_input(2021, 18)?;
    let l1 = input.lines().next().unwrap();
    let start = SFNumber::from_str(l1);
    let mut numbers_a: Vec<SFNumber> = input.lines().map(|s| SFNumber::from_str(s)).collect();
    let numbers_b = numbers_a.clone();
    let result = numbers_a.drain(1..).fold(start, |prev, next| {
        add_sfnumber(prev, next)
    });
    println!("Answer A: {}", result.magnitude());
    let ans_b: usize = numbers_b.iter().enumerate().map(|(nl, l)| {
        numbers_b.iter().enumerate().filter_map(|(nr, r)| {
            if nl == nr {
                None
            } else {
                Some(refadd_sfnumber(l, r).magnitude())
            }
        }).max().unwrap()
    }).max().unwrap();
    println!("Answer B: {}", ans_b);
    println!("Computed in {}us", t_start.elapsed().as_micros());
    Ok(())
}
