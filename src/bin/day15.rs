use petgraph::{algo::dijkstra, graph::NodeIndex, Graph};

const DIM: usize = 100;
const MULTIPLIER: usize = 5;
const DIM_MULT: usize = DIM * MULTIPLIER;

fn parse_data<const MULT: usize, const DIM_MULT: usize>(
) -> (Graph<(u16, u16), u8>, NodeIndex, NodeIndex) {
    let mut graph = Graph::new();
    let mut array = [[(NodeIndex::default(), 0); DIM_MULT]; DIM_MULT];

    for (i, line) in include_str!("../../inputs/day15.txt").lines().enumerate() {
        for (j, num) in line.bytes().map(|c| c - b'0').enumerate() {
            for k in 0..MULT {
                for l in 0..MULT {
                    let y = i + k * DIM;
                    let x = j + l * DIM;
                    let idx = graph.add_node((y as u16, x as u16));
                    let weight = num + (k + l) as u8;

                    array[y][x] = (idx, weight % 10 + (weight - 1) / 9);
                }
            }
        }
    }

    for i in 0..DIM_MULT {
        for j in 0..DIM_MULT {
            if let Some((idx, left)) = (j > 0).then(|| array[i][j - 1]) {
                graph.add_edge(array[i][j].0, idx, left);
            }

            if let Some((idx, right)) = (j < DIM_MULT - 1).then(|| array[i][j + 1]) {
                graph.add_edge(array[i][j].0, idx, right);
            }

            if let Some((idx, up)) = (i > 0).then(|| array[i - 1][j]) {
                graph.add_edge(array[i][j].0, idx, up);
            }

            if let Some((idx, down)) = (i < DIM_MULT - 1).then(|| array[i + 1][j]) {
                graph.add_edge(array[i][j].0, idx, down);
            }
        }
    }

    (graph, array[0][0].0, array[DIM_MULT - 1][DIM_MULT - 1].0)
}

fn main() {
    let (cave, start, end) = parse_data::<1, DIM>();
    let (cave_2, start_2, end_2) = parse_data::<MULTIPLIER, DIM_MULT>();
    let cost = dijkstra(&cave, start, Some(end), |edge| *edge.weight() as u32);
    let cost_2 = dijkstra(&cave_2, start_2, Some(end_2), |edge| *edge.weight() as u32);

    println!("Part 1: {}", cost.get(&end).unwrap());
    println!("Part 2: {}", cost_2.get(&end_2).unwrap());
}
