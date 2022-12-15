use std::collections::HashSet;
use itertools::Itertools;
use regex::Regex;

struct Coord { x: i32, y: i32 }
struct Pair { sensor: Coord, beacon: Coord }

fn solve1(data: String, target: i32) -> usize {
    let parsed = parse(data);
    let mut pos = HashSet::new();
    
    for pair in parsed.iter() {
        let range = (pair.beacon.x - pair.sensor.x).abs() + (pair.beacon.y - pair.sensor.y).abs();
        let distance = (pair.sensor.y - target).abs();
        for i in 0..=range - distance {
            pos.insert(pair.sensor.x + i);
            pos.insert(pair.sensor.x - i);
        }   
    }
    
    pos.len() - parsed.into_iter().filter(|p| p.beacon.y == target).map(|p| p.beacon.x).unique().count()
}

fn solve2(data: String, max_coord: i32) -> u64 {
    let parsed = parse(data);
    let length = parsed.len();
    let ranges: Vec<i32> = parsed.iter().map(|p| (p.beacon.x - p.sensor.x).abs() + (p.beacon.y - p.sensor.y).abs()).collect();
    let mut x = 0;
    while x <= max_coord {
        let mut increment = 16;
        let mut y = 0;
        let x_distances: Vec<i32> = parsed.iter().map(|p| (x - p.sensor.x).abs()).collect();
        while y <= max_coord {
            let mut new_y = y;
            for i in 0..length {
                let y_dist = (parsed[i].sensor.y - new_y).abs();
                let dist = ranges[i] - x_distances[i] - y_dist;
                if dist >= increment - 1 { new_y += dist + 2 - increment; }
            }
            if y == new_y { increment /= 2; } 
            if increment == 0 { return 4_000_000 * x as u64 + new_y as u64; }
            y = new_y;

            // let y_distances: Vec<i32> = parsed.iter().map(|p| (y - p.sensor.y).abs()).collect();
            // let to_border = (0..length).map(|i| ranges[i] - x_distances[i] - y_distances[i]).max().unwrap();
            // if to_border < 0 { return 4_000_000 * x as u64 + y as u64; }
            // y += to_border + 1;
        }
        
        x+= increment;
    }
    
    panic!("No position for beacon");
}

fn parse(data: String) -> Vec<Pair> {
    let re = Regex::new(r"^Sensor at x=(-?\d*), y=(-?\d*): closest beacon is at x=(-?\d*), y=(-?\d*)$").unwrap();

    data.split("\n").map(|line| re.captures(line).unwrap()).map(|c| Pair{ sensor: Coord { x: c[1].parse().unwrap(), y: c[2].parse().unwrap()}, beacon: Coord { x: c[3].parse().unwrap(), y: c[4].parse().unwrap()} }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = solve1(input_example(), 10);
        assert_eq!(result, 26);
    }

    #[test]
    fn exercise1() {
        let result = solve1(input_exercise(), 2_000_000);
        assert_eq!(result, 4886370);
    }

    #[test]
    fn example2() {
        let result = solve2(input_example(), 20);
        assert_eq!(result, 56000011);
    }

    #[test]
    fn exercise2() {
        let result = solve2(input_exercise(), 4_000_000);
        assert_eq!(result, 11374534948438);
    }

    fn input_example() -> String {
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3".to_string()
    }


    fn input_exercise() -> String {
        "Sensor at x=220580, y=684270: closest beacon is at x=436611, y=263737
Sensor at x=3329538, y=3016377: closest beacon is at x=3355914, y=2862466
Sensor at x=2605308, y=2023938: closest beacon is at x=2197530, y=2271330
Sensor at x=1810202, y=3423309: closest beacon is at x=1829362, y=3182862
Sensor at x=480296, y=3999646: closest beacon is at x=1694700, y=4178942
Sensor at x=46556, y=1283362: closest beacon is at x=-91140, y=1441882
Sensor at x=3741660, y=3959257: closest beacon is at x=3537901, y=3368697
Sensor at x=3399994, y=700264: closest beacon is at x=3748004, y=2000000
Sensor at x=1531981, y=3801761: closest beacon is at x=1694700, y=4178942
Sensor at x=193367, y=2712458: closest beacon is at x=-91140, y=1441882
Sensor at x=3199067, y=2194575: closest beacon is at x=3748004, y=2000000
Sensor at x=1878117, y=2578817: closest beacon is at x=2197530, y=2271330
Sensor at x=2439089, y=3168242: closest beacon is at x=1829362, y=3182862
Sensor at x=273443, y=171076: closest beacon is at x=436611, y=263737
Sensor at x=3680413, y=2477027: closest beacon is at x=3748004, y=2000000
Sensor at x=3620241, y=2904998: closest beacon is at x=3355914, y=2862466
Sensor at x=1728351, y=2895399: closest beacon is at x=1829362, y=3182862
Sensor at x=1894207, y=1168355: closest beacon is at x=2197530, y=2271330
Sensor at x=856867, y=3271314: closest beacon is at x=1829362, y=3182862
Sensor at x=3056788, y=2626224: closest beacon is at x=3355914, y=2862466
Sensor at x=3598024, y=3322247: closest beacon is at x=3537901, y=3368697
Sensor at x=1662543, y=3128823: closest beacon is at x=1829362, y=3182862
Sensor at x=3992558, y=1933059: closest beacon is at x=3748004, y=2000000
Sensor at x=1844282, y=2994285: closest beacon is at x=1829362, y=3182862
Sensor at x=3604375, y=3668021: closest beacon is at x=3537901, y=3368697
Sensor at x=2569893, y=3911832: closest beacon is at x=1694700, y=4178942
Sensor at x=117970, y=37503: closest beacon is at x=436611, y=263737
Sensor at x=3951385, y=3125577: closest beacon is at x=3537901, y=3368697
Sensor at x=2482373, y=2648092: closest beacon is at x=2197530, y=2271330
Sensor at x=915040, y=1835970: closest beacon is at x=-91140, y=1441882
Sensor at x=3047883, y=3301452: closest beacon is at x=3537901, y=3368697
Sensor at x=117432, y=1503889: closest beacon is at x=-91140, y=1441882
Sensor at x=1136011, y=261705: closest beacon is at x=436611, y=263737
Sensor at x=2343111, y=66183: closest beacon is at x=2081841, y=-807749
Sensor at x=608229, y=955721: closest beacon is at x=436611, y=263737
Sensor at x=1189379, y=3999750: closest beacon is at x=1694700, y=4178942
Sensor at x=766640, y=26597: closest beacon is at x=436611, y=263737
Sensor at x=3891093, y=2110588: closest beacon is at x=3748004, y=2000000".to_string()
    }
}
