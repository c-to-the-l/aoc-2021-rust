use aoc_2021::get_input;
use anyhow::Result;
use petgraph::{algo, graph::NodeIndex, Graph};

fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let input = get_input(2021, 15)?;
    let grid_a: Vec<usize> = input
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .flatten()
        .collect();
    let grid: Vec<usize> = (0..5)
        .map(|y_tile| {
            input
                .lines()
                .map(move |l| {
                    (0..5)
                        .map(move |x_tile| {
                            l.chars().map(move |c| {
                                ((c.to_digit(10).unwrap() as usize + x_tile + y_tile - 1) % 9) + 1
                            })
                        })
                        .flatten()
                })
                .flatten()
        })
        .flatten()
        .collect();

    let a_goal_x = input.lines().next().unwrap().len();
    let width = a_goal_x * 5;
    let len = grid.len();
    let len_a = grid_a.len();

    let mut graph_a = Graph::<usize, usize>::new();
    let nodes_a: Vec<NodeIndex> = grid_a
        .iter()
        .enumerate()
        .map(|(x, _)| graph_a.add_node(x))
        .collect();
    assert_eq!(nodes_a.len(), grid_a.len());
    let mut graph = Graph::<usize, usize>::new();
    let nodes: Vec<NodeIndex> = grid
        .iter()
        .enumerate()
        .map(|(x, _)| graph.add_node(x))
        .collect();
    assert_eq!(nodes.len(), grid.len());
    for (n, (idx, val)) in nodes_a.iter().zip(grid_a.iter()).enumerate() {
        if n % a_goal_x > 0 {
            let prev_x = nodes_a.get(n - 1).unwrap();
            graph_a.add_edge(*prev_x, *idx, *val);
        }
        if n >= a_goal_x {
            let prev_y = nodes_a.get(n - a_goal_x).unwrap();
            graph_a.add_edge(*prev_y, *idx, *val);
        }
        if (n % a_goal_x) + 1 < a_goal_x {
            let next_x = nodes_a.get(n + 1).unwrap();
            graph_a.add_edge(*next_x, *idx, *val);
        }
        if n + a_goal_x < len_a {
            let next_y = nodes_a.get(n + a_goal_x).unwrap();
            graph_a.add_edge(*next_y, *idx, *val);
        }
    }
    for (n, (idx, val)) in nodes.iter().zip(grid.iter()).enumerate() {
        if n % width > 0 {
            let prev_x = nodes.get(n - 1).unwrap();
            graph.add_edge(*prev_x, *idx, *val);
        }
        if n >= width {
            let prev_y = nodes.get(n - width).unwrap();
            graph.add_edge(*prev_y, *idx, *val);
        }
        if (n % width) + 1 < width {
            let next_x = nodes.get(n + 1).unwrap();
            graph.add_edge(*next_x, *idx, *val);
        }
        if n + width < len {
            let next_y = nodes.get(n + width).unwrap();
            graph.add_edge(*next_y, *idx, *val);
        }
    }

    let first = nodes.get(0).unwrap();
    let first_a = nodes_a.get(0).unwrap();

    let last_a = nodes_a.last().unwrap();
    let last = nodes.last().unwrap();

    let costs_a = algo::dijkstra::dijkstra(&graph_a, *first_a, Some(*last_a), |e| *e.weight());
    let cost_a = costs_a.get(last_a).unwrap();

    let costs = algo::dijkstra::dijkstra(&graph, *first, Some(*last), |e| *e.weight());
    let cost = costs.get(last).unwrap();

    println!("Answer A: {}", cost_a);
    println!("Answer B: {}", cost);
    println!("Computed in {}us", start.elapsed().as_micros());
    Ok(())
}
