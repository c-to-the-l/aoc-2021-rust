use aoc_2021::get_input;
use anyhow::Result;
use image::{RgbaImage, Rgba, Frame, Pixel};
use image::codecs::gif::GifEncoder;

const PIX_SCALE: u32 = 4;

fn match_colour(val: &usize) -> Rgba<u8> {
    match *val {
        0..=8 => Rgba::from([(*val as u8)*15 + 15, (*val as u8)*15 + 15, (*val as u8)*15 + 15, 255]),
        _ => Rgba::from([0, 0, 0, 255])
    }
}

struct GeneratorContext {
    frames: Vec<Frame>,
    c_choice: usize,
}
impl GeneratorContext {

    fn colour_searched(&mut self, x: usize, y: usize) {
        let x = x as u32 * PIX_SCALE;
        let y = y as u32 * PIX_SCALE;
        let mut next = self.frames.last().unwrap().clone().into_buffer();
        
        for i in x..x+PIX_SCALE{
            for j in y..y+PIX_SCALE {
                let mut px = next.get_pixel_mut(i, j).channels_mut();
                px[self.c_choice] = 0;
            }
        }
        self.frames.push(Frame::new(next));
    }


    fn search_at(&mut self, map: &Vec<Vec<usize>>, searched: &mut Vec<Vec<bool>>, x: usize, y: usize) -> usize {
        if y >= map.len() || x >= map.get(0).unwrap().len() {
            return 0;
        }
        let marker = searched.get_mut(y).unwrap().get_mut(x).unwrap();
        if *marker {
            return 0;
        } else {
            *marker = true;
        }
        self.colour_searched(x, y);
        let val = map.get(y).unwrap().get(x).unwrap();
        if *val >= 9 {
            return 0;
        }
        let up = if y > 0 {
            self.search_at(map, searched, x, y - 1)
        } else {
            0
        };
        let left = if x > 0 {
            self.search_at(map, searched, x - 1, y)
        } else {
            0
        };
        let right = self.search_at(map, searched, x + 1, y);
        let down = self.search_at(map, searched, x, y + 1);
        1 + up + left + right + down
    }

    fn maybe_search(
        &mut self,
        map: &Vec<Vec<usize>>,
        searched: &mut Vec<Vec<bool>>,
        x: usize,
        y: usize,
    ) -> Option<usize> {
        if *searched.get(y).unwrap().get(x).unwrap() {
            None
        } else {
            Some(self.search_at(map, searched, x, y))
        }
    }
}

fn main() -> Result<()> {
    let input = get_input(2021, 9)?;
    let start = std::time::Instant::now();
    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let y_len = map.len();
    let x_len = map.get(0).unwrap().len();
    let img = RgbaImage::from_fn((x_len as u32) * PIX_SCALE, (y_len as u32) * PIX_SCALE,|x, y| {
        match_colour(map.get((y / PIX_SCALE) as usize ).unwrap().get((x / PIX_SCALE) as usize).unwrap())
    });
    let mut gen = GeneratorContext {
        frames: Vec::from([Frame::new(img)]),
        c_choice: 0,
    };
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

    let mut searched: Vec<Vec<bool>> = vec![vec![false; x_len + 1]; y_len + 1];
    let mut results_b: Vec<usize> = Vec::new();
    for y in 0..y_len {
        for x in 0..x_len {
            if let Some(val) = gen.maybe_search(&map, &mut searched, x, y) {
                results_b.push(val);
                gen.c_choice = (gen.c_choice + 1) % 3;
            }
        }
    }
    results_b.sort();
    results_b.reverse();
    let mut file_out = std::fs::File::create("out.gif")?;
    let mut encoder = GifEncoder::new_with_speed(file_out, 1);
    encoder.encode_frames(gen.frames.into_iter());
    println!("Answer A {}", sum_a);
    println!("Answer B {}", results_b[..3].iter().product::<usize>());
    println!("Computed in {}us", start.elapsed().as_micros());
    Ok(())
}
