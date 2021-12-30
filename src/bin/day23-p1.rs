
use aoc_2021::get_input;
use anyhow::Result;
use petgraph::{graphmap::GraphMap, Directed, algo::dijkstra};
use bimap::BiHashMap;

fn score(a: usize) -> usize {
    match a {
        0 => 0,
        1 => 1,
        2 => 10,
        3 => 100,
        4 => 1000,
        x => panic!("Not a valid state: {}", x),
    }
}

fn num_for(s: char) -> usize {
    match s {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        x => panic!("Unexpected character {}", x),
    }
}

fn hallway_path(cave: usize, hallway: usize) -> std::ops::RangeInclusive<usize> {
    if hallway > (cave + 1) {
        (cave+2)..=hallway
    } else {
        hallway..=(cave+1)
    }
}

fn dist(cave: usize, hall: usize) -> usize {
    match cave {
        0 => match hall {
            0 => 2,
            1 => 1,
            2 => 1,
            3 => 3,
            4 => 5,
            5 => 7,
            6 => 8,
            x => panic!("Invalid cave-hall 0 {}", x),
        },
        1 => match hall {
            0 => 4,
            1 => 3,
            2 => 1,
            3 => 1,
            4 => 3,
            5 => 5,
            6 => 6,
            x => panic!("Invalid cave-hall 1 {}", x),
        },
        2 => match hall {
            0 => 6,
            1 => 5,
            2 => 3,
            3 => 1,
            4 => 1,
            5 => 3,
            6 => 4,
            x => panic!("Invalid cave-hall 2 {}", x),
        },
        3 => match hall {
            0 => 8,
            1 => 7,
            2 => 5,
            3 => 3,
            4 => 1,
            5 => 1,
            6 => 2,
            x => panic!("Invalid cave-hall 3 {}", x),
        },
        x => panic!("Invalid cave {}", x),
    }
}


// #############
// #12.3.4.5.67#
// ###0#0#0#0###
//   #1#1#1#1#
//   #########
//    1 2 3 4
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Burrow {
    // cave[x][0] = upper space
    // cave[x][1] = lower space
    caves: [[usize; 2]; 4],
    // the seven movable hallway spaces.
    hallway: [usize; 7],
}

impl Burrow {
    fn from_str(s: &str) -> Self {
        let mut si = s.lines().skip(2);
        let mut u = si.next().unwrap().chars();
        let mut l = si.next().unwrap().chars();
        Burrow {
            hallway: [0; 7],
            caves: [
                [num_for(u.nth(3).unwrap()), num_for(l.nth(3).unwrap())],
                [num_for(u.nth(1).unwrap()), num_for(l.nth(1).unwrap())],
                [num_for(u.nth(1).unwrap()), num_for(l.nth(1).unwrap())],
                [num_for(u.nth(1).unwrap()), num_for(l.nth(1).unwrap())],
            ],
        }
    }
    

    fn can_path(&self, cavenum: usize, cavelevel: usize, hallway: usize, in_hall: bool ) -> Option<usize> {
        if in_hall {
            // target space occupied
            if self.caves[cavenum][cavelevel] > 0 {
                return None
            }
            // cave entrance occupied
            if self.caves[cavenum][0] > 0 {
                return None
            }
            // base of cave occupied by non-resident, or is empty
            if cavelevel == 0 && self.caves[cavenum][1] != (cavenum + 1) {
                return None
            }
            // any blocks in hallway path
            for n in hallway_path(cavenum, hallway) {
                if self.hallway[n] > 0 {
                    if n != hallway {
                        return None
                    }
                }
            }
        } else {
            // target space occupied
            if self.hallway[hallway] > 0 {
                return None
            }
            // cave entrance blocked
            if cavelevel == 1 && self.caves[cavenum][0] > 0 {
                return None
            }
            // trying to move correct value out of base of cave
            if cavelevel == 1 && self.caves[cavenum][1] == cavenum + 1 {
                return None
            }   
            // block in hallway
            for n in hallway_path(cavenum, hallway) {
                if self.hallway[n] > 0 {
                    return None
                }
            }
            
        }
        Some(1 + cavelevel + dist(cavenum, hallway)) 
    }

    fn has_won(&self) -> bool{
        self == &Burrow {
            hallway: [0;7],
            caves: [
                [1, 1],
                [2, 2],
                [3, 3],
                [4, 4]
            ]
        }
    }

    fn cave_complete(&self, cave: usize) -> bool {
        self.caves[cave] == [cave+1, cave+1]
    }
}

// returns true if it inserted the next node, as an indication that the recursor should recurse into the node.
fn maybe_insert(g: &mut GraphMap<usize, usize, Directed>, states: &mut BiHashMap<usize, Burrow>, prev: &Burrow, next: &Burrow, edge_weight: usize) -> bool {
    let mut rv = true;
    let pn = *states.get_by_right(prev).unwrap();
    assert!(g.contains_node(pn));
    let nn = if let Some(n) = states.get_by_right(next) {
        assert!(g.contains_node(*n));
        rv = false;
        *n
    } else {
        let n = states.left_values().max().unwrap() + 1;
        states.insert_no_overwrite(n, next.clone()).unwrap();
        g.add_node(n);
        n
    };
    if !g.contains_edge(pn, nn) {
        g.add_edge(pn, nn, edge_weight);
    }
    rv
}



fn make_graph(g: &mut GraphMap<usize, usize, Directed>, states: &mut BiHashMap<usize, Burrow>, state: Burrow) {
    if state.has_won() {
        println!("State wins! {:?}", state);
        return;
    }

    for h in 0..7 {
        if state.hallway[h] > 0 {
            let amph = state.hallway[h];
            if let Some(dist) = state.can_path(amph-1, 1, h, true) {
                let mut new = state.clone();
                new.hallway[h] = 0;
                new.caves[amph - 1][1] = amph;
                if maybe_insert(g, states, &state, &new, dist*score(amph)) {
                    make_graph(g, states, new);
                }
            }
            if let Some(dist) = state.can_path(amph-1, 0, h, true) {
                let mut new = state.clone();
                new.hallway[h] = 0;
                new.caves[amph - 1][0] = amph;
                if maybe_insert(g, states, &state, &new, dist*score(amph)) {
                    make_graph(g, states, new);
                }
            }
        }
    }
    for c in 0..4 {
        if !state.cave_complete(c) {
            for h in 0..7 {
                for l in 0..2 {
                    if let Some(dist) = state.can_path(c, l, h, false) {
                        let amphi = state.caves[c][l];
                        let mut new = state.clone();
                        new.hallway[h] = amphi;
                        new.caves[c][l] = 0;
                        if maybe_insert(g, states, &state, &new, dist*score(amphi)) {
                            make_graph(g, states, new);
                        }
                    }
                }
            }
        }
    }

}

fn main() -> Result<()> {
    let t_start = std::time::Instant::now();
    let input = get_input(2021, 23)?;
    let burrow = Burrow::from_str(&input);
    let mut states: BiHashMap<usize, Burrow> = BiHashMap::new();
    states.insert(0, burrow.clone());
    let mut graph: GraphMap<usize, usize, Directed> = GraphMap::new();
    graph.add_node(0);
    make_graph(&mut graph, &mut states, burrow.clone());
    println!("Graph Complete. Nodes {}. Edges {}. States {}", graph.node_count(), graph.edge_count(), states.len());

    let winner = states.get_by_right(&Burrow{
        hallway: [0;7],
        caves: [[1,1], [2,2], [3,3], [4,4]]
    }).unwrap();
    let costs = dijkstra(&graph, 0, Some(*winner), |(_, _, e)| *e);
    let cost = costs.get(winner).unwrap();
    println!("Answer A: {}", cost);

    println!("Computed in {}us", t_start.elapsed().as_micros());
    Ok(())
}
