use nom::bytes::complete::{tag, take_until};
use nom::*;

fn main() {
    let input = include_str!("../inputs/day19.txt");
    let res1 = process_part_1(input);
    println!("Part 1:{}", res1);
    let res2 = process_part_2(input);
    println!("Part 2:{}", res2);
}

fn process_part_1(input: &str) -> usize {
    let (_, blueprints) = parse_input(input).unwrap();
    let total_time = 24;

    let mut total_sum = 0;
    //starting at minute 1
    let starting_resources: [usize; 4] = [0, 0, 0, 0];

    let mut geode_counts: Vec<usize> = vec![];
    for (i, blueprint) in blueprints.iter().enumerate() {
        // println!("Blueprint {}", i + 1);
        geode_counts.push(maximize_geode(
            vec![
                (&blueprint.ore_robot, 1),
                (&blueprint.clay_robot, 0),
                (&blueprint.obsidian_robot, 0),
                (&blueprint.geode_robot, 0),
            ],
            None,
            vec![],
            starting_resources,
            total_time - 1,
            blueprint,
        ));
        total_sum += geode_counts[i] * (i + 1);
    }
    // for g in geode_counts {
    //     println!("Geode Count {}", g);
    // }

    total_sum
}
fn process_part_2(input: &str) -> usize {
    let (_, blueprints) = parse_input(input).unwrap();
    let total_time = 32;

    //starting at minute 1
    let starting_resources: [usize; 4] = [0, 0, 0, 0];
    blueprints
        .iter()
        .take(3)
        .map(|blueprint| {
            maximize_geode(
                vec![
                    (&blueprint.ore_robot, 1),
                    (&blueprint.clay_robot, 0),
                    (&blueprint.obsidian_robot, 0),
                    (&blueprint.geode_robot, 0),
                ],
                None,
                vec![],
                starting_resources,
                total_time - 1,
                blueprint,
            )
        })
        .product()
}

fn add_mined_resources(
    mining_robots: &[(&Robot, usize)],
    resources: &mut [usize; 4],
    seconds: usize,
) {
    for (robot, number_of_robots) in mining_robots.clone() {
        match robot.robot_type {
            MineralType::Ore => {
                resources[0] += number_of_robots * seconds;
            }
            MineralType::Clay => {
                resources[1] += number_of_robots * seconds;
            }
            MineralType::Obsidian => {
                resources[2] += number_of_robots * seconds;
            }
            MineralType::Geode => {
                resources[3] += number_of_robots * seconds;
            }
        }
    }
}

// fn pretty_print_path(path: &Vec<Option<&Robot>>, total_resources: &[usize; 4]) {
//     for r in path {
//         match r {
//             None => {
//                 print!("🚫->");
//             }
//             Some(r) => match r.robot_type {
//                 MineralType::Ore => print!("🥌->"),
//                 MineralType::Clay => print!("🧱->"),
//                 MineralType::Obsidian => print!("♊->"),
//                 MineralType::Geode => print!("💎->"),
//             },
//         }
//     }
//     print!("{}", total_resources[3]);
//     println!();
// }

fn maximize_geode(
    mining_robots: Vec<(&Robot, usize)>,
    built_robot: Option<&Robot>,
    path: Vec<Option<&Robot>>,
    resources: [usize; 4],
    time_remaining: usize,
    blueprint: &Blueprint,
) -> usize {
    // let mut path = path;
    let mut resources = resources;

    add_mined_resources(&mining_robots, &mut resources, 1);
    if time_remaining == 0 {
        // if resources[3] > 0 {
        // pretty_print_path(&path, &resources);
        // }
        return resources[3];
    }

    let mining_robots: Vec<(&Robot, usize)> = mining_robots
        .iter()
        .map(|(robot, n)| {
            if built_robot.is_some() && robot.robot_type == built_robot.unwrap().robot_type {
                (*robot, n + 1)
            } else {
                (*robot, *n)
            }
        })
        .collect();

    let ore_stock = resources[0];
    let clay_stock = resources[1];
    let obsidian_stock = resources[2];
    let geode_stock = resources[3];

    let ore_robot_count = mining_robots[0].1;
    let clay_robot_count = mining_robots[1].1;
    let obsidian_robot_count = mining_robots[2].1;
    let geode_robot_count = mining_robots[3].1;

    let ore_robot_cost = blueprint.ore_robot.costs.ore_cost;
    let clay_robot_cost = blueprint.clay_robot.costs.ore_cost;
    let obsidian_ore_cost = blueprint.obsidian_robot.costs.ore_cost;
    let obsidian_clay_cost = blueprint.obsidian_robot.costs.clay_cost;
    let geode_ore_cost = blueprint.geode_robot.costs.ore_cost;
    let geode_obsidian_cost = blueprint.geode_robot.costs.obsidian_cost;

    let max_ore_robots_needed =
        ore_robot_cost.max(clay_robot_cost.max(obsidian_ore_cost.max(geode_ore_cost)));
    let max_clay_robots_needed = obsidian_clay_cost;
    let max_obsidian_robots_needed = geode_obsidian_cost;

    let mut next_robots: Vec<&Robot> = vec![];

    if !max_robots_reached(
        ore_robot_count,
        time_remaining,
        ore_stock,
        max_ore_robots_needed,
    ) {
        next_robots.push(&blueprint.ore_robot);
    }
    if !max_robots_reached(
        clay_robot_count,
        time_remaining,
        clay_stock,
        max_clay_robots_needed,
    ) {
        next_robots.push(&blueprint.clay_robot);
    }
    if clay_robot_count > 0
        && !max_robots_reached(
            obsidian_robot_count,
            time_remaining,
            obsidian_stock,
            max_obsidian_robots_needed,
        )
    {
        next_robots.push(&blueprint.obsidian_robot);
    }
    if obsidian_robot_count > 0 {
        next_robots.push(&blueprint.geode_robot);
    }

    // let mut robots_built: usize = 0;
    let mut current_highest_geode: usize = geode_stock + geode_robot_count * time_remaining;
    for built_robot in next_robots {
        let mut local_time_remaining = time_remaining;
        let mut local_resources = resources;
        let mut local_path = path.clone();

        let mut seconds_to_build_enough_stock = 0;
        match built_robot.robot_type {
            MineralType::Ore => {
                if ore_stock < ore_robot_cost {
                    let d = (ore_robot_cost as f32 - ore_stock as f32) / ore_robot_count as f32;
                    seconds_to_build_enough_stock = d.ceil() as usize;
                }
            }
            MineralType::Clay => {
                if ore_stock < clay_robot_cost {
                    let d = (clay_robot_cost as f32 - ore_stock as f32) / ore_robot_count as f32;
                    seconds_to_build_enough_stock = d.ceil() as usize;
                }
            }
            MineralType::Obsidian => {
                let mut seconds_to_build_enough_ore_stock = 0;
                let mut seconds_to_build_enough_clay_stock = 0;
                if ore_stock < obsidian_ore_cost {
                    let d = (obsidian_ore_cost as f32 - ore_stock as f32) / ore_robot_count as f32;
                    seconds_to_build_enough_ore_stock = d.ceil() as usize;
                }
                if clay_stock < obsidian_clay_cost {
                    let d =
                        (obsidian_clay_cost as f32 - clay_stock as f32) / clay_robot_count as f32;
                    seconds_to_build_enough_clay_stock = d.ceil() as usize;
                }
                seconds_to_build_enough_stock =
                    seconds_to_build_enough_ore_stock.max(seconds_to_build_enough_clay_stock);
            }
            MineralType::Geode => {
                let mut seconds_to_build_enough_ore_stock = 0;
                let mut seconds_to_build_enough_obsidian_stock = 0;
                if ore_stock < geode_ore_cost {
                    let d = (geode_ore_cost as f32 - ore_stock as f32) / ore_robot_count as f32;
                    seconds_to_build_enough_ore_stock = d.ceil() as usize;
                }
                if obsidian_stock < geode_obsidian_cost {
                    let d = (geode_obsidian_cost as f32 - obsidian_stock as f32)
                        / obsidian_robot_count as f32;
                    seconds_to_build_enough_obsidian_stock = d.ceil() as usize;
                }
                seconds_to_build_enough_stock =
                    seconds_to_build_enough_ore_stock.max(seconds_to_build_enough_obsidian_stock);
            }
        }
        if seconds_to_build_enough_stock > 0 {
            if seconds_to_build_enough_stock > local_time_remaining - 1 {
                continue;
            }
            local_time_remaining -= seconds_to_build_enough_stock;
            for _ in 0..seconds_to_build_enough_stock {
                local_path.push(None);
            }
            add_mined_resources(
                &mining_robots,
                &mut local_resources,
                seconds_to_build_enough_stock,
            );
        }
        remove_resources_to_build_robot(built_robot, &mut local_resources);
        local_path.push(Some(built_robot));
        // pretty_print_path(&local_path, &local_resources);
        current_highest_geode = current_highest_geode.max(maximize_geode(
            mining_robots.clone(),
            Some(built_robot),
            local_path.clone(),
            local_resources,
            local_time_remaining - 1,
            blueprint,
        ));
    }
    current_highest_geode
}

fn max_robots_reached(
    robot_count: usize,
    time_remaining: usize,
    stock: usize,
    max_robots_needed: usize,
) -> bool {
    robot_count * time_remaining + stock >= time_remaining * max_robots_needed
}

fn remove_resources_to_build_robot(built_robot: &Robot, resources: &mut [usize; 4]) {
    resources[0] -= built_robot.costs.ore_cost;
    resources[1] -= built_robot.costs.clay_cost;
    resources[2] -= built_robot.costs.obsidian_cost;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MineralType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy)]
struct Cost {
    ore_cost: usize,
    clay_cost: usize,
    obsidian_cost: usize,
}
impl Cost {
    fn new(ore_cost: usize, clay_cost: usize, obsidian_cost: usize) -> Cost {
        Cost {
            ore_cost,
            clay_cost,
            obsidian_cost,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    robot_type: MineralType,
    costs: Cost,
}

#[derive(Debug, Clone)]
struct Blueprint {
    ore_robot: Robot,
    clay_robot: Robot,
    obsidian_robot: Robot,
    geode_robot: Robot,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Blueprint>> {
    let mut blueprints: Vec<Blueprint> = vec![];
    for line in input.lines() {
        let (_, o) = parse_line(line)?;
        let (ore_robot, clay_robot, obsidian_robot, geode_robot) = o;
        blueprints.push(Blueprint {
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        });
    }
    Ok(("", blueprints))
}

fn parse_line(input: &str) -> IResult<&str, (Robot, Robot, Robot, Robot)> {
    let (input, _) = take_until(": ")(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, _) = tag("Each ore robot costs ")(input)?;
    let (input, ore_robot_cost) =
        combinator::map_res(character::complete::digit1, str::parse::<usize>)(input)?;
    let (input, _) = tag(" ore. Each clay robot costs ")(input)?;

    let (input, clay_robot_cost) =
        combinator::map_res(character::complete::digit1, str::parse::<usize>)(input)?;
    let (input, _) = tag(" ore. Each obsidian robot costs ")(input)?;

    let (input, obsidian_robot_ore_cost) =
        combinator::map_res(character::complete::digit1, str::parse::<usize>)(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, obsidian_robot_clay_cost) =
        combinator::map_res(character::complete::digit1, str::parse::<usize>)(input)?;
    let (input, _) = tag(" clay. Each geode robot costs ")(input)?;

    let (input, geode_robot_ore_cost) =
        combinator::map_res(character::complete::digit1, str::parse::<usize>)(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, geode_robot_obsidian_cost) =
        combinator::map_res(character::complete::digit1, str::parse::<usize>)(input)?;
    let (input, _) = tag(" obsidian.")(input)?;

    let ore_robot = Robot {
        robot_type: MineralType::Ore,
        costs: Cost::new(ore_robot_cost, 0, 0),
    };
    let clay_robot = Robot {
        robot_type: MineralType::Clay,
        costs: Cost::new(clay_robot_cost, 0, 0),
    };
    let obsidian_robot = Robot {
        robot_type: MineralType::Obsidian,
        costs: Cost::new(obsidian_robot_ore_cost, obsidian_robot_clay_cost, 0),
    };
    let geode_robot = Robot {
        robot_type: MineralType::Geode,
        costs: Cost::new(geode_robot_ore_cost, 0, geode_robot_obsidian_cost),
    };

    Ok((input, (ore_robot, clay_robot, obsidian_robot, geode_robot)))
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_process_part_1() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        assert_eq!(process_part_1(input), 33);
    }

    #[test]
    fn test_process_part_2() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        assert_eq!(process_part_2(input), 3472);
    }

    #[test]
    fn test_add_mined_resources() {
        let mut input_resources: [usize; 4] = [0, 0, 0, 0];
        let robots: [(&Robot, usize); 4] = [
            (
                &Robot {
                    robot_type: MineralType::Ore,
                    costs: Cost::new(4, 0, 0),
                },
                1,
            ),
            (
                &Robot {
                    robot_type: MineralType::Clay,
                    costs: Cost::new(2, 0, 0),
                },
                0,
            ),
            (
                &Robot {
                    robot_type: MineralType::Obsidian,
                    costs: Cost::new(3, 14, 0),
                },
                0,
            ),
            (
                &Robot {
                    robot_type: MineralType::Geode,
                    costs: Cost::new(2, 0, 7),
                },
                0,
            ),
        ];
        let output = [1, 0, 0, 0];
        add_mined_resources(&robots, &mut input_resources, 1);
        assert_eq!(input_resources, output);

        let mut input_resources: [usize; 4] = [1, 3, 4, 5];
        let robots: [(&Robot, usize); 4] = [
            (
                &Robot {
                    robot_type: MineralType::Ore,
                    costs: Cost::new(4, 0, 0),
                },
                1,
            ),
            (
                &Robot {
                    robot_type: MineralType::Clay,
                    costs: Cost::new(2, 0, 0),
                },
                1,
            ),
            (
                &Robot {
                    robot_type: MineralType::Obsidian,
                    costs: Cost::new(3, 14, 0),
                },
                1,
            ),
            (
                &Robot {
                    robot_type: MineralType::Geode,
                    costs: Cost::new(2, 0, 7),
                },
                1,
            ),
        ];
        let output = [2, 4, 5, 6];
        add_mined_resources(&robots, &mut input_resources, 1);
        assert_eq!(input_resources, output);

        let mut input_resources: [usize; 4] = [0, 7, 0, 0];
        let robots: [(&Robot, usize); 4] = [
            (
                &Robot {
                    robot_type: MineralType::Ore,
                    costs: Cost::new(4, 0, 0),
                },
                2,
            ),
            (
                &Robot {
                    robot_type: MineralType::Clay,
                    costs: Cost::new(2, 0, 0),
                },
                7,
            ),
            (
                &Robot {
                    robot_type: MineralType::Obsidian,
                    costs: Cost::new(3, 14, 0),
                },
                1,
            ),
            (
                &Robot {
                    robot_type: MineralType::Geode,
                    costs: Cost::new(2, 0, 7),
                },
                0,
            ),
        ];
        let output = [2, 14, 1, 0];
        add_mined_resources(&robots, &mut input_resources, 1);
        assert_eq!(input_resources, output);

        let output = [6, 28, 3, 0];
        add_mined_resources(&robots, &mut input_resources, 2);
        assert_eq!(input_resources, output);
    }
}
