use itertools::Itertools;
use nom::bytes::complete::{tag, take_until};
use nom::*;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input = include_str!("../../inputs/day16.txt");
    let res1 = process_part_1(input);
    println!("Part 1:{}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2:{}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> u32 {
    let (_, nodes) = parse_input(input).unwrap();
    // dbg!(nodes.clone());
    let mut edges: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut flow_rates: HashMap<usize, u32> = HashMap::new();
    for node in nodes.clone() {
        edges.insert(
            name_to_index(node.name.clone()),
            node.edges.clone().into_iter().map(name_to_index).collect(),
        );
        flow_rates.insert(name_to_index(node.name.clone()), node.flow_rate);
    }
    // dbg!(edges.clone());
    // dbg!(flow_rates.clone());

    let mut valves_with_positive_flow_rate: Vec<usize> = flow_rates
        .clone()
        .into_iter()
        .filter(|(_, flow)| *flow > 0)
        .map(|(name, _)| name)
        .collect_vec();

    valves_with_positive_flow_rate.sort();

    let distances = floyd_warshall(&edges);
    for i in 0..distances.len() {
        for j in 0..distances.len() {
            let n = distances[i][j];
            if n < usize::MAX / 2 && n > 0 {
                // dbg!((i, j, n));
            }
        }
    }
    let mut cache: HashMap<(usize, Vec<usize>, usize), u32> = HashMap::new();
    find_largest_flow_rate(
        &flow_rates,
        &distances,
        valves_with_positive_flow_rate.clone(),
        0,
        30,
        &mut cache,
    )
}

fn find_largest_flow_rate(
    flow_rates: &HashMap<usize, u32>,
    distances: &Vec<Vec<usize>>,
    pending_valves: Vec<usize>,
    current_valve: usize,
    remaining_time: usize,
    cache: &mut HashMap<(usize, Vec<usize>, usize), u32>,
) -> u32 {
    // println!("------------------------");
    // dbg!(current_valve);
    // dbg!(pending_valves.clone());
    // dbg!(remaining_time);
    let mut result = 0;
    let key = (current_valve, pending_valves.clone(), remaining_time);
    if cache.contains_key(&key) {
        result = *cache.get(&key).unwrap();
        // dbg!(result);
        return result;
    }

    for i in 0..pending_valves.len() {
        let next_pending_valve = pending_valves[i];

        // Number of steps required to move from `current_valve` to `next_value`
        let steps = distances[current_valve][next_pending_valve];

        if remaining_time < steps + 1 {
            // dbg!(steps + 1);
            continue;
        }

        // 1 more step to open the valve
        let next_remaining_steps = remaining_time - steps - 1;

        let next_pending_valves = pending_valves[0..i]
            .iter()
            .copied()
            .chain(pending_valves[i + 1..].to_vec())
            .collect();

        let next_result = find_largest_flow_rate(
            flow_rates,
            distances,
            next_pending_valves,
            next_pending_valve,
            next_remaining_steps,
            cache,
        );

        let current_result = next_result
            + flow_rates.get(&next_pending_valve).unwrap() * next_remaining_steps as u32;

        // dbg!(current_result);
        result = result.max(current_result);
    }
    cache.insert(key, result);
    // dbg!(result);
    result
}

fn name_to_index(name: String) -> usize {
    let mut chars = name.chars();
    let mut number = 0;
    number += chars.next().unwrap() as usize - 65;
    number *= 26;
    number += chars.next().unwrap() as usize - 65;
    number
}

fn floyd_warshall(edges: &HashMap<usize, Vec<usize>>) -> Vec<Vec<usize>> {
    let matrix_size = 676;
    let mut valve_distances = vec![vec![usize::MAX / 2; matrix_size]; matrix_size];

    for (name, edges) in edges {
        for leading_edge in edges {
            valve_distances[*name][*leading_edge] = 1;
            valve_distances[*leading_edge][*name] = 1;
        }
    }

    for i in 0..matrix_size {
        valve_distances[i][i] = 0;
    }

    for k in 0..matrix_size {
        for i in 0..matrix_size {
            for j in 0..matrix_size {
                valve_distances[i][j] =
                    valve_distances[i][j].min(valve_distances[i][k] + valve_distances[k][j]);
            }
        }
    }
    valve_distances
}

fn process_part_2(input: &str) -> u32 {
    let (_, nodes) = parse_input(input).unwrap();
    // dbg!(nodes.clone());
    let mut edges: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut flow_rates: HashMap<usize, u32> = HashMap::new();
    for node in nodes.clone() {
        edges.insert(
            name_to_index(node.name.clone()),
            node.edges.clone().into_iter().map(name_to_index).collect(),
        );
        flow_rates.insert(name_to_index(node.name.clone()), node.flow_rate);
    }
    // dbg!(edges.clone());
    // dbg!(flow_rates.clone());

    let mut valves_with_positive_flow_rate: Vec<usize> = flow_rates
        .clone()
        .into_iter()
        .filter(|(_, flow)| *flow > 0)
        .map(|(name, _)| name)
        .collect_vec();

    valves_with_positive_flow_rate.sort();

    let distances = floyd_warshall(&edges);
    for i in 0..distances.len() {
        for j in 0..distances.len() {
            let n = distances[i][j];
            if n < usize::MAX / 2 && n > 0 {
                // dbg!((i, j, n));
            }
        }
    }
    let mut cache: HashMap<(usize, Vec<usize>, usize), u32> = HashMap::new();
    let partitions = get_partitions(valves_with_positive_flow_rate.clone());
    partitions
        .into_iter()
        .map(|(left, right)| {
            let left_sum = find_largest_flow_rate(&flow_rates, &distances, left, 0, 26, &mut cache);
            let right_sum =
                find_largest_flow_rate(&flow_rates, &distances, right, 0, 26, &mut cache);
            left_sum + right_sum
        })
        .max()
        .unwrap()
}

fn get_partitions(valves: Vec<usize>) -> Vec<(Vec<usize>, Vec<usize>)> {
    // We can assume the first value is always assigned to the left (due to symmetry)
    let mut result: Vec<Vec<usize>> = vec![vec![valves[0]]];

    for value in &valves[1..] {
        let mut new_result = result.clone();
        for r in result {
            let mut cloned = r.clone();
            cloned.push(*value);
            new_result.push(cloned);
        }
        result = new_result;
    }

    result
        .into_iter()
        .map(|left| {
            let values_cloned = valves.clone();
            let right = values_cloned
                .into_iter()
                .filter(|x| !left.contains(x))
                .collect_vec();
            (left, right)
        })
        .collect_vec()
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    edges: Vec<String>,
    flow_rate: u32,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Node>> {
    let mut nodes: Vec<Node> = vec![];
    for line in input.lines() {
        let (_, o) = parse_line(line)?;
        let (name, flow_rate, tunnels) = o;
        nodes.push(Node {
            name: name.to_string(),
            edges: tunnels,
            flow_rate,
        });
    }
    Ok((input, nodes))
}

fn parse_line(input: &str) -> IResult<&str, (&str, u32, Vec<String>)> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, valve_name) = take_until(" ")(input)?;
    let (input, _) = take_until("=")(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, flow_rate) =
        combinator::map_res(character::complete::digit1, str::parse::<u32>)(input)?;

    let (input, _) = tag("; tunnel")(input)?;
    let (input, _) = take_until(" ")(input)?;
    let (input, _) = tag(" lead")(input)?;
    let (input, _) = take_until(" ")(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, _) = take_until(" ")(input)?;
    let mut tunnels: Vec<String> = vec![input.to_string()];
    if input.len() != 2 {
        let t = input.replace(' ', "");
        tunnels = t.split(',').map(|x| x.clone().to_owned()).collect();
    }

    Ok(("", (valve_name, flow_rate, tunnels)))
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_process_part_1() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(process_part_1(input), 1651);
    }

    #[test]
    fn test_process_part_2() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(process_part_2(input), 1707);
    }

    #[test]
    fn test_name_to_index() {
        let mut input = "AA".to_string();
        assert_eq!(name_to_index(input), 0);

        input = "ZZ".to_string();
        assert_eq!(name_to_index(input), 675);
    }
    #[test]
    fn test_partition() {
        let input = vec![0, 1, 2];
        let output = vec![
            (vec![0], vec![1, 2]),
            (vec![0, 1], vec![2]),
            (vec![0, 2], vec![1]),
            (vec![0, 1, 2], vec![]),
        ];
        assert_eq!(get_partitions(input), output);
    }
}
