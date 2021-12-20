use aoc_runner_derive::{aoc};

#[aoc(day12, part1)]
fn count_paths(input: &str) -> usize {
    let _edges = input.lines().map(|l| {
        l.split_once("-").unwrap()
    }).collect::<Vec<(&str, &str)>>();

    // let mut graph = DiGraphMap::default();
    //
    // for (a, b) in edges {
    //     graph.add_edge(a, b, ());
    //     graph.add_edge(b, a, ());
    // }
    //
    // let res = all_simple_paths(graph, "start".into(), "end".into(), 0, None).collect::<Vec<_>>();
    //
    // println!("res: {:?}", res);

    0
}