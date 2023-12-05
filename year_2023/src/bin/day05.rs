use core::fmt;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/day05.txt");

    let time_start = Instant::now();
    let res1 = process_part_1(input);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> isize {
    let mut x = input.split("\n\n");
    let seeds = x.next().unwrap();
    let seed_to_soil = x.next().unwrap();
    let soil_to_fertilizer = x.next().unwrap();
    let fertilizer_to_water = x.next().unwrap();
    let water_to_light = x.next().unwrap();
    let light_to_temp = x.next().unwrap();
    let temp_to_humidity = x.next().unwrap();
    let humidity_to_location = x.next().unwrap();

    let seeds = parse_seeds(seeds);
    // dbg!(seeds.clone());
    let seed_to_soil = parse_destination_to_source(seed_to_soil);
    // dbg!(seed_to_soil.clone());
    let soil_to_fertilizer = parse_destination_to_source(soil_to_fertilizer);
    let fertilizer_to_water = parse_destination_to_source(fertilizer_to_water);
    let water_to_light = parse_destination_to_source(water_to_light);
    let light_to_temp = parse_destination_to_source(light_to_temp);
    let temp_to_humidity = parse_destination_to_source(temp_to_humidity);
    let humidity_to_location = parse_destination_to_source(humidity_to_location);

    seeds
        .into_iter()
        .map(|seed| {
            // dbg!(seed);
            let mut soil = seed as isize;
            for (seed_range, offset) in seed_to_soil.clone() {
                if seed_range.check_if_inside_range(soil) {
                    // println!("found {} in range {}", soil, seed_range);
                    soil += offset;
                    break;
                }
            }
            // dbg!(soil);

            let mut fertilizer = soil;
            for (soil_range, offset) in soil_to_fertilizer.clone() {
                if soil_range.check_if_inside_range(soil) {
                    fertilizer = soil + offset;
                    break;
                }
            }
            // dbg!(fertilizer);

            let mut water = fertilizer;
            for (fertilizer_range, offset) in fertilizer_to_water.clone() {
                if fertilizer_range.check_if_inside_range(fertilizer) {
                    water = fertilizer + offset;
                    break;
                }
            }
            // dbg!(water);

            let mut light = water;
            for (water_range, offset) in water_to_light.clone() {
                if water_range.check_if_inside_range(water) {
                    light = water + offset;
                    break;
                }
            }
            // dbg!(light);

            let mut temp = light;
            for (light_range, offset) in light_to_temp.clone() {
                if light_range.check_if_inside_range(light) {
                    temp = light + offset;
                    break;
                }
            }
            // dbg!(temp);

            let mut humidity = temp;
            for (temp_range, offset) in temp_to_humidity.clone() {
                if temp_range.check_if_inside_range(temp) {
                    humidity = temp + offset;
                    break;
                }
            }
            // dbg!(humidity);

            let mut location = humidity;
            for (humidity_range, offset) in humidity_to_location.clone() {
                if humidity_range.check_if_inside_range(humidity) {
                    location = humidity + offset;
                    break;
                }
            }
            // dbg!(location);
            location.clone()
        })
        .min()
        .unwrap()
}

fn process_part_2(input: &str) -> isize {
    let mut x = input.split("\n\n");
    let seeds = x.next().unwrap();
    let seed_to_soil = x.next().unwrap();
    let soil_to_fertilizer = x.next().unwrap();
    let fertilizer_to_water = x.next().unwrap();
    let water_to_light = x.next().unwrap();
    let light_to_temp = x.next().unwrap();
    let temp_to_humidity = x.next().unwrap();
    let humidity_to_location = x.next().unwrap();

    let seed_to_soil = parse_destination_to_source(seed_to_soil);
    // dbg!(seed_to_soil.clone());
    let soil_to_fertilizer = parse_destination_to_source(soil_to_fertilizer);
    let fertilizer_to_water = parse_destination_to_source(fertilizer_to_water);
    let water_to_light = parse_destination_to_source(water_to_light);
    let light_to_temp = parse_destination_to_source(light_to_temp);
    let temp_to_humidity = parse_destination_to_source(temp_to_humidity);
    let humidity_to_location = parse_destination_to_source(humidity_to_location);

    let seeds = &seeds[7..];
    let seeds = seeds
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut i = 0;

    let mut min_location = isize::MAX;
    while i < seeds.len() {
        let start = seeds[i];
        for seed in start..start + seeds[i + 1] {
            // dbg!(seed);
            let mut soil = seed as isize;
            for (seed_range, offset) in seed_to_soil.clone() {
                if seed_range.check_if_inside_range(soil) {
                    // println!("found {} in range {}", soil, seed_range);
                    soil += offset;
                    break;
                }
            }
            // dbg!(soil);

            let mut fertilizer = soil;
            for (soil_range, offset) in soil_to_fertilizer.clone() {
                if soil_range.check_if_inside_range(soil) {
                    fertilizer = soil + offset;
                    break;
                }
            }
            // dbg!(fertilizer);

            let mut water = fertilizer;
            for (fertilizer_range, offset) in fertilizer_to_water.clone() {
                if fertilizer_range.check_if_inside_range(fertilizer) {
                    water = fertilizer + offset;
                    break;
                }
            }
            // dbg!(water);

            let mut light = water;
            for (water_range, offset) in water_to_light.clone() {
                if water_range.check_if_inside_range(water) {
                    light = water + offset;
                    break;
                }
            }
            // dbg!(light);

            let mut temp = light;
            for (light_range, offset) in light_to_temp.clone() {
                if light_range.check_if_inside_range(light) {
                    temp = light + offset;
                    break;
                }
            }
            // dbg!(temp);

            let mut humidity = temp;
            for (temp_range, offset) in temp_to_humidity.clone() {
                if temp_range.check_if_inside_range(temp) {
                    humidity = temp + offset;
                    break;
                }
            }
            // dbg!(humidity);

            let mut location = humidity;
            for (humidity_range, offset) in humidity_to_location.clone() {
                if humidity_range.check_if_inside_range(humidity) {
                    location = humidity + offset;
                    break;
                }
            }
            // dbg!(location);
            // println!("\n\n----------");
            min_location = min_location.min(location.clone());
        }
        i += 2;
    }

    min_location
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct MyRange {
    start: usize,
    end: usize,
}

impl MyRange {
    fn new(start: usize, end: usize) -> MyRange {
        MyRange { start, end }
    }

    fn check_if_inside_range(self: &Self, check: isize) -> bool {
        self.start <= check as usize && self.end >= check as usize
    }
}

impl fmt::Display for MyRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyRange {{ start: {}, end: {}}}", self.start, self.end)
    }
}

fn parse_destination_to_source(dest_to_source: &str) -> HashMap<MyRange, isize> {
    let mut out: HashMap<MyRange, isize> = HashMap::new();
    dest_to_source.lines().skip(1).for_each(|line| {
        let x = line
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let destination_start = x[0];
        let source_start = x[1];
        let size = x[2];
        // dbg!(destination_start);
        // dbg!(source_start);
        // dbg!(size);
        out.insert(
            MyRange::new(source_start, source_start + size),
            destination_start as isize - source_start as isize,
        );
    });
    out
}

fn parse_seeds(seeds: &str) -> Vec<usize> {
    let seeds = &seeds[7..];
    seeds
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn parse_seeds_2(seeds: &str) -> Vec<usize> {
    let seeds = &seeds[7..];
    let seeds = seeds
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut i = 0;
    let mut out: Vec<usize> = vec![];
    while i < seeds.len() {
        let start = seeds[i];
        for j in start..start + seeds[i + 1] {
            out.push(j);
        }
        i += 2;
    }
    out
}
