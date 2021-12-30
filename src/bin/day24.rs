use anyhow::Result;

include!(concat!(env!("OUT_DIR"), "/day24-gen.rs"));

fn main() -> Result<()> {
    let t_start = std::time::Instant::now();
    println!("Warning: This will take a long time. Up to ten minutes.");
    println!("Go make a cup of tea or something.");
    let mut ans_a: u128 = 0;
    'outer: 
    for a in (1..10).rev() {
        let (x1, y1, z1) = alu_step1(a, 0, 0, 0);
        for b in (1..10).rev() {
            let (x2, y2, z2) = alu_step2(b, x1, y1, z1);
            for c in (1..10).rev() {
                let (x3, y3, z3) = alu_step3(c, x2, y2, z2);
                for d in (1..10).rev() {
                    println!("{}{}{}{}",a,b,c,d);
                    let (x4, y4, z4) = alu_step4(d, x3, y3, z3);
                    for e in (1..10).rev() {
                        let (x5, y5, z5) = alu_step5(e, x4, y4, z4);
                        for f in (1..10).rev() {
                            let (x6, y6, z6) = alu_step6(f, x5, y5, z5);
                            for g in (1..10).rev() {
                                let (x7, y7, z7) = alu_step7(g, x6, y6, z6);
                                for h in (1..10).rev() {
                                    let (x8, y8, z8) = alu_step8(h, x7, y7, z7);
                                    for i in (1..10).rev() {
                                        let (x9, y9, z9) = alu_step9(i, x8, y8, z8);
                                        for j in (1..10).rev() {
                                            let (x10, y10, z10) = alu_step10(j, x9, y9, z9);
                                            for k in (1..10).rev() {
                                                let (x11, y11, z11) = alu_step11(k, x10, y10, z10);
                                                for l in (1..10).rev() {
                                                    let (x12, y12, z12) = alu_step12(l, x11, y11, z11);
                                                    for m in (1..10).rev() {
                                                        let (x13, y13, z13) = alu_step13(m, x12, y12, z12);
                                                        for n in (1..10).rev() {
                                                            if alu_step14(n, x13, y13, z13) == 0 {
                                                                ans_a = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}",a,b,c,d,e,f,g,h,i,j,k,l,m,n).parse().unwrap();
                                                                break 'outer
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut ans_b: u128 = 0;

    'outer2: 
    for a in 1..10 {
        let (x1, y1, z1) = alu_step1(a, 0, 0, 0);
        for b in 1..10 {
            let (x2, y2, z2) = alu_step2(b, x1, y1, z1);
            for c in 1..10 {
                let (x3, y3, z3) = alu_step3(c, x2, y2, z2);
                for d in 1..10 {
                    println!("{}{}{}{}",a,b,c,d);
                    let (x4, y4, z4) = alu_step4(d, x3, y3, z3);
                    for e in 1..10 {
                        let (x5, y5, z5) = alu_step5(e, x4, y4, z4);
                        for f in 1..10 {
                            let (x6, y6, z6) = alu_step6(f, x5, y5, z5);
                            for g in 1..10 {
                                let (x7, y7, z7) = alu_step7(g, x6, y6, z6);
                                for h in 1..10 {
                                    let (x8, y8, z8) = alu_step8(h, x7, y7, z7);
                                    for i in 1..10 {
                                        let (x9, y9, z9) = alu_step9(i, x8, y8, z8);
                                        for j in 1..10 {
                                            let (x10, y10, z10) = alu_step10(j, x9, y9, z9);
                                            for k in 1..10 {
                                                let (x11, y11, z11) = alu_step11(k, x10, y10, z10);
                                                for l in 1..10 {
                                                    let (x12, y12, z12) = alu_step12(l, x11, y11, z11);
                                                    for m in 1..10 {
                                                        let (x13, y13, z13) = alu_step13(m, x12, y12, z12);
                                                        for n in 1..10 {
                                                            if alu_step14(n, x13, y13, z13) == 0 {
                                                                ans_b = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}",a,b,c,d,e,f,g,h,i,j,k,l,m,n).parse().unwrap();
                                            
                                                                break 'outer2
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Answer A: {}", ans_a);
    println!("Answer B: {}", ans_b);


    println!("Computed in {}us", t_start.elapsed().as_micros());
    Ok(())
}
