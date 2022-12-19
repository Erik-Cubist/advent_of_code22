use std::cmp::max;
use regex::Regex;

struct Blueprint {
    id: i32,
    ore_ore: i32,
    clay_ore: i32,
    obsidian_ore: i32,
    obsidian_clay: i32,
    geode_ore: i32,
    geode_obsidian: i32
}

struct Robots {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

fn solve1(data: String) -> i32 {
    let parsed = parse(data);
    let mut result = 0;
    
    for blueprint in parsed {
        let robots = Robots { ore: 1, clay: 0, obsidian: 0, geode: 0 };
        let resources = Robots { ore: 0, clay: 0, obsidian: 0, geode: 0 };

        let cracked = solve(&blueprint, &robots, &resources, 24);
        
        result += blueprint.id * cracked;
    }
    
    result
}

fn solve(blueprint: &Blueprint, robots: &Robots, resources: &Robots, time_left: i32) -> i32{
    let times: Robots = get_time_to_produce(blueprint, robots, resources);
    let geodes_if_no_more_robots = resources.geode + robots.geode * time_left;
    max(
        max(
            if times.ore < time_left && robots.ore <= blueprint.clay_ore { solve(blueprint, &Robots { ore: robots.ore + 1, ..*robots}, &new_resources(blueprint.ore_ore, 0, 0, robots, resources, times.ore), time_left - times.ore)} else{ geodes_if_no_more_robots },
            if times.clay < time_left && robots.clay <= blueprint.obsidian_clay { solve(blueprint, &Robots { clay: robots.clay + 1, ..*robots}, &new_resources(blueprint.clay_ore, 0, 0, robots, resources, times.clay), time_left - times.clay)} else{ geodes_if_no_more_robots }),
        max(
            if times.obsidian < time_left { solve(blueprint, &Robots { obsidian: robots.obsidian + 1, ..*robots}, &new_resources(blueprint.obsidian_ore, blueprint.obsidian_clay, 0, robots, resources, times.obsidian), time_left - times.obsidian)} else{ geodes_if_no_more_robots },
            if times.geode < time_left { solve(blueprint, &Robots { geode: robots.geode + 1, ..*robots}, &new_resources(blueprint.geode_ore, 0, blueprint.geode_obsidian, robots, resources, times.geode), time_left - times.geode)} else { geodes_if_no_more_robots })
    )
}

fn new_resources(ore_cost: i32, clay_cost: i32, obsidian_cost: i32, robots: &Robots, resources: &Robots, time: i32) -> Robots {
    Robots {
        ore: resources.ore + robots.ore * time - ore_cost,
        clay: resources.clay + robots.clay * time - clay_cost,
        obsidian: resources.obsidian + robots.obsidian * time - obsidian_cost,
        geode: resources.geode + robots.geode * time,
    }
}

fn get_time_to_produce(blueprint: &Blueprint, robots: &Robots, resources: &Robots) -> Robots {
    Robots {
        ore: max((blueprint.ore_ore - resources.ore + robots.ore - 1) / robots.ore, 0) + 1,
        clay: max((blueprint.clay_ore - resources.ore + robots.ore - 1) / robots.ore, 0) + 1,
        obsidian: if robots.clay == 0 { 999 } else { max((blueprint.obsidian_ore - resources.ore + robots.ore - 1) / robots.ore, max((blueprint.obsidian_clay - resources.clay + robots.clay - 1) / robots.clay, 0)) + 1},
        geode: if robots.obsidian == 0 { 999 } else { max((blueprint.geode_ore - resources.ore + robots.ore - 1) / robots.ore, max((blueprint.geode_obsidian - resources.obsidian + robots.obsidian - 1) / robots.obsidian, 0)) + 1},
    }
}


fn solve2(data: String) -> i32 {
    let parsed = parse(data);
    let mut result = 1;

    for blueprint in parsed.iter().take(3) {
        let robots = Robots { ore: 1, clay: 0, obsidian: 0, geode: 0 };
        let resources = Robots { ore: 0, clay: 0, obsidian: 0, geode: 0 };

        let cracked = solve(&blueprint, &robots, &resources, 32);

        result *= cracked;
    }

    result
}


fn parse(data: String) -> Vec<Blueprint> {
    let re = Regex::new(r"Blueprint (\d*):\s* Each ore robot costs (\d*) ore.\s* Each clay robot costs (\d*) ore.\s* Each obsidian robot costs (\d*) ore and (\d*) clay.\s* Each geode robot costs (\d*) ore and (\d*) obsidian.").unwrap();
    re.captures_iter(&data).map(|capt| Blueprint {
        id: capt[1].parse().unwrap(),
        ore_ore: capt[2].parse().unwrap(),
        clay_ore: capt[3].parse().unwrap(),
        obsidian_ore: capt[4].parse().unwrap(),
        obsidian_clay: capt[5].parse().unwrap(),
        geode_ore: capt[6].parse().unwrap(),
        geode_obsidian: capt[7].parse().unwrap(),
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = solve1(input_example());
        assert_eq!(result, 33);
    }

    #[test]
    fn exercise1() {
        let result = solve1(input_exercise());
        assert_eq!(result, 1599);
    }

    #[test]
    fn example2() {
        let result = solve2(input_example());
        assert_eq!(result, 56*62);
    }

    #[test]
    fn exercise2() {
        let result = solve2(input_exercise());
        assert_eq!(result, 14112);
    }

    fn input_example() -> String {
        "Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.".to_string()
    }


    fn input_exercise() -> String {
        "Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 10 obsidian.
Blueprint 2: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 19 clay. Each geode robot costs 4 ore and 11 obsidian.
Blueprint 3: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 4 ore and 9 obsidian.
Blueprint 4: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 4 ore and 15 obsidian.
Blueprint 5: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 18 clay. Each geode robot costs 4 ore and 8 obsidian.
Blueprint 6: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 9 clay. Each geode robot costs 3 ore and 7 obsidian.
Blueprint 7: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 4 ore and 11 obsidian.
Blueprint 8: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 12 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 9: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 12 clay. Each geode robot costs 3 ore and 17 obsidian.
Blueprint 10: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 19 clay. Each geode robot costs 2 ore and 14 obsidian.
Blueprint 11: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 3 ore and 18 obsidian.
Blueprint 12: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 15 clay. Each geode robot costs 2 ore and 20 obsidian.
Blueprint 13: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 3 ore and 7 obsidian.
Blueprint 14: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 17 clay. Each geode robot costs 2 ore and 13 obsidian.
Blueprint 15: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 16: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 4 ore and 8 obsidian.
Blueprint 17: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 20 obsidian.
Blueprint 18: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 16 clay. Each geode robot costs 3 ore and 14 obsidian.
Blueprint 19: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 3 ore and 8 obsidian.
Blueprint 20: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 18 clay. Each geode robot costs 4 ore and 20 obsidian.
Blueprint 21: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 4 ore and 15 obsidian.
Blueprint 22: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 2 ore and 11 obsidian.
Blueprint 23: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 11 clay. Each geode robot costs 4 ore and 7 obsidian.
Blueprint 24: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 12 clay. Each geode robot costs 3 ore and 8 obsidian.
Blueprint 25: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 7 clay. Each geode robot costs 3 ore and 8 obsidian.
Blueprint 26: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 11 clay. Each geode robot costs 2 ore and 19 obsidian.
Blueprint 27: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 3 ore and 20 obsidian.
Blueprint 28: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 4 ore and 12 obsidian.
Blueprint 29: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 6 clay. Each geode robot costs 2 ore and 16 obsidian.
Blueprint 30: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 13 clay. Each geode robot costs 3 ore and 12 obsidian.".to_string()
    }
}
