use aoc_2021::get_input;
use anyhow::Result;


// len and len_ty are never actually used, so they're
// underscored out to suppress warnings
struct OpPacket {
    ver: u8,
    ty: u8,
    _len_ty: bool,
    _len: u16,
    sub_packets: Vec<BITSPacket>,
}

struct LitPacket {
    ver: u8,
    _ty: u8,
    val: u64,
}

enum BITSPacket {
    Operator(OpPacket),
    Literal(LitPacket),
}

fn get_continuable_int(s: &Vec<u8>, pos: &mut usize) -> (bool, u8) {
    let p = s.get(*pos / 4).unwrap();
    let n = s.get((*pos / 4) + 1).unwrap();
    let v = p << 4 | n;
    let rv = v >> (3 - (*pos % 4));
    *pos += 5;
    (rv & 0x10 > 0, rv & 0x0F)
}

fn get_11bit_val(s: &Vec<u8>, pos: &mut usize) -> u16 {
    // code here
    let align = (*pos) % 4;
    let idx = (*pos)/4;
    let a = *s.get(idx).unwrap() as u16 & 0x000F;
    let b = *s.get(idx + 1).unwrap() as u16 & 0x000F;
    let c = *s.get(idx + 2).unwrap() as u16 & 0x000F;
    let d = *s.get(idx + 3).unwrap() as u16 & 0x000F;
    let rv = a << 12 | b << 8  | c << 4 | d;
    let rv = rv >> (5 - align);
    *pos += 11;
    rv & 0x07FF
}

fn get_15bit_val(s: &Vec<u8>, pos: &mut usize) -> u16 {
    // code here
    let align = (*pos - 1) % 4;
    let idx = (*pos - 1)/4;
    let a = *s.get(idx).unwrap() as u16 & 0x000F;
    let b = *s.get(idx + 1).unwrap() as u16 & 0x000F;
    let c = *s.get(idx + 2).unwrap() as u16 & 0x000F;
    let d = *s.get(idx + 3).unwrap() as u16 & 0x000F;
    let e = *s.get(idx + 4).unwrap() as u16 & 0x000F;
    let rv = a << (12 + align) | b << (8 + align) | c << (4 + align) | d << align | e >> (4 - align);
    *pos += 15;
    rv & 0x7FFF
}

fn get_3bit_val(s: &Vec<u8>, pos: &mut usize) -> u8 {
    let p = s.get(*pos / 4).unwrap();
    let n = s.get((*pos / 4) + 1).unwrap();
    let v = p << 4 | n;
    let rv = (v >> (5 - *pos % 4)) & 0x07;
    *pos += 3;
    rv
}

fn get_bit(s: &Vec<u8>, pos: &mut usize) -> bool {
    let p = s.get(*pos / 4).unwrap();
    let rv = (p & (0x08 >> (*pos % 4))) > 0;
    *pos += 1;
    return rv;
}

fn get_varwidth_val(s: &Vec<u8>, pos: &mut usize) -> u64 {
    let mut rv: u64 = 0;
    loop {
        let (cont, val) = get_continuable_int(s, pos);
        rv = (rv << 4) | val as u64;
        if !cont {
            break;
        }
    }
    rv
}

fn parse_packet_at(s: &Vec<u8>, pos: &mut usize) -> BITSPacket {
    let ver = get_3bit_val(s, pos);
    let ty = get_3bit_val(s, pos);
    match ty {
        4 => {
            let val = get_varwidth_val(s, pos);
            BITSPacket::Literal(LitPacket { ver, _ty: ty, val })
        }
        _ => {
            let len_ty = get_bit(s, pos);
            // if len_type, then "len" is the number of sub-packets
            if len_ty {
                let len = get_11bit_val(s, pos);
                let sub_packets = (0..len).map(|_| parse_packet_at(s, pos)).collect();
                BITSPacket::Operator(OpPacket {
                    ver,
                    ty,
                    _len_ty: len_ty,
                    _len: len,
                    sub_packets,
                })
            } else {
                let len = get_15bit_val(s, pos);
                let start = *pos;
                let mut sub_packets = Vec::new();
                while (*pos - start) < len as usize {
                    sub_packets.push(parse_packet_at(s, pos));
                }
                BITSPacket::Operator(OpPacket {
                    ver,
                    ty,
                    _len_ty: len_ty,
                    _len: len,
                    sub_packets,
                })
            }
        }
    }
}

fn parse_bits_stream(s: &Vec<u8>) -> Vec<BITSPacket> {
    let mut rv = Vec::new();
    let mut pos: usize = 0;
    while pos < s.len() - 11 {
        rv.push(parse_packet_at(s, &mut pos));
    }
    rv
}

fn calc_a(s: &Vec<BITSPacket>) -> usize {
    s.iter().map(|x| 
        {
            match x {
                BITSPacket::Literal(x) => x.ver as usize,
                BITSPacket::Operator(x) => {
                    x.ver as usize + calc_a(&x.sub_packets)
                }
            }
        }
    ).sum()
}

impl BITSPacket {
    fn eval(&self) -> usize {
        match self {
            BITSPacket::Literal(x) => x.val as usize,
            BITSPacket::Operator(x) => {
                match x.ty {
                    0 => x.sub_packets.iter().map(|x| x.eval()).sum(),
                    1 => x.sub_packets.iter().map(|x| x.eval()).product(),
                    2 => x.sub_packets.iter().map(|x| x.eval()).min().unwrap(),
                    3 => x.sub_packets.iter().map(|x| x.eval()).max().unwrap(),
                    5 => (x.sub_packets.get(0).unwrap().eval() > x.sub_packets.get(1).unwrap().eval()) as usize,
                    6 => (x.sub_packets.get(0).unwrap().eval() < x.sub_packets.get(1).unwrap().eval()) as usize,
                    7 => (x.sub_packets.get(0).unwrap().eval() == x.sub_packets.get(1).unwrap().eval()) as usize,
                    n => panic!("Unexpected packet type {}", n),
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let input = get_input(2021, 16)?;

    let input: Vec<u8> = input
        .trim()
        .chars()
        .map(|x| x.to_digit(16).unwrap() as u8)
        .collect();
    
    let packets = parse_bits_stream(&input);

    println!("Answer A: {}", calc_a(&packets));
    println!("Answer B: {}", packets.get(0).unwrap().eval());
    println!("Computed in {}us", start.elapsed().as_micros());
    Ok(())
}
