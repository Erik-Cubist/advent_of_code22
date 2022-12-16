use std::cmp::max;
use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Clone)]
struct Valve { flow: i32, can_be_opened: bool, tunnels: Vec<String> }

fn solve1(data: String) -> i32 {
    let parsed = parse(data);
    let visited = HashSet::new();
    
    crawl(30, &"AA", &parsed, &visited)
}

fn crawl(minutes_left: i32, current_key: &str, parsed: &HashMap::<String, Valve>, visited: &HashSet::<String>) -> i32 {
    if minutes_left <= 0 { return 0;}
    if visited.contains(current_key) { return 0; }
    if parsed.iter().filter(|(_, v)| v.can_be_opened).count() == 0 { return 0;}

    let mut pressure_released = 0;
    let mut sub_result1 = 0;

    if parsed[current_key].can_be_opened {
        let mut copy = parsed.clone();
        let mut current = copy.get_mut(current_key).unwrap();
        current.can_be_opened = false;
        pressure_released += (minutes_left -1) * current.flow;
        sub_result1 = parsed[current_key].tunnels.iter().map(|t| crawl(minutes_left - 2, &t, &copy, &HashSet::new())).max().unwrap();
    }

    let mut visited_copy = visited.clone();
    visited_copy.insert(current_key.to_string());
    let sub_result2 = parsed[current_key].tunnels.iter().map(|t| crawl(minutes_left - 1, &t, parsed, &visited_copy)).max().unwrap();
    
    max(sub_result1 + pressure_released, sub_result2)
}


fn solve2(data: String, _max_coord: i32) -> u64 {
    let _parsed = parse(data);
    1
}

fn parse(data: String) -> HashMap<String, Valve> {
    let re = Regex::new(r"^Valve (..) has flow rate=(\d*); tunnels? leads? to valves? (.*)$").unwrap();
    let mut valves = HashMap::new();
    
    for line in data.split("\n").map(|line| re.captures(line).unwrap()) {
       let key = &line[1];
       let flow = line[2].parse().unwrap();
        let value = Valve { flow, can_be_opened: flow > 0, tunnels: line[3].split(", ").map(|s| s.to_string()).collect() };
       valves.insert(key.to_string(), value);
    }
    
    valves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = solve1(input_example());
        assert_eq!(result, 1651);
    }

    #[test]
    fn exercise1() {
        let result = solve1(input_exercise());
        assert_eq!(result, 0);
    }

    #[test]
    fn example2() {
        let result = solve2(input_example(), 20);
        assert_eq!(result, 0);
    }

    #[test]
    fn exercise2() {
        let result = solve2(input_exercise(), 4_000_000);
        assert_eq!(result, 0);
    }

    fn input_example() -> String {
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II".to_string()
    }


    fn input_exercise() -> String {
        "Valve OQ has flow rate=17; tunnels lead to valves NB, AK, KL
Valve HP has flow rate=0; tunnels lead to valves ZX, KQ
Valve GO has flow rate=0; tunnels lead to valves HR, GW
Valve PD has flow rate=9; tunnels lead to valves XN, EV, QE, MW
Valve NQ has flow rate=0; tunnels lead to valves HX, ZX
Valve DW has flow rate=0; tunnels lead to valves IR, WE
Valve TN has flow rate=24; tunnels lead to valves KL, EI
Valve JJ has flow rate=0; tunnels lead to valves EV, HR
Valve KH has flow rate=0; tunnels lead to valves ZQ, AA
Valve PH has flow rate=0; tunnels lead to valves FN, QE
Valve FD has flow rate=0; tunnels lead to valves SM, HX
Valve SM has flow rate=7; tunnels lead to valves WW, RZ, FD, HO, KQ
Valve PU has flow rate=0; tunnels lead to valves VL, IR
Valve OM has flow rate=0; tunnels lead to valves CM, AA
Valve KX has flow rate=20; tunnel leads to valve PC
Valve IR has flow rate=3; tunnels lead to valves PU, CM, WW, DW, AF
Valve XG has flow rate=0; tunnels lead to valves RX, OF
Valve QE has flow rate=0; tunnels lead to valves PH, PD
Valve GW has flow rate=0; tunnels lead to valves JQ, GO
Valve HO has flow rate=0; tunnels lead to valves SM, TY
Valve WU has flow rate=0; tunnels lead to valves SG, RZ
Valve MS has flow rate=0; tunnels lead to valves UE, OF
Valve JS has flow rate=0; tunnels lead to valves DO, ZX
Valve YQ has flow rate=0; tunnels lead to valves BC, SG
Valve EJ has flow rate=0; tunnels lead to valves AA, LR
Valve EI has flow rate=0; tunnels lead to valves BV, TN
Valve NC has flow rate=0; tunnels lead to valves TS, BC
Valve AF has flow rate=0; tunnels lead to valves IR, HX
Valve OX has flow rate=0; tunnels lead to valves HR, BV
Valve BF has flow rate=0; tunnels lead to valves JQ, SY
Valve CA has flow rate=0; tunnels lead to valves YD, HX
Valve KQ has flow rate=0; tunnels lead to valves HP, SM
Valve NB has flow rate=0; tunnels lead to valves OQ, OF
Valve SY has flow rate=0; tunnels lead to valves BF, BV
Valve AA has flow rate=0; tunnels lead to valves KH, EJ, OM, TY, DO
Valve BC has flow rate=11; tunnels lead to valves WE, RX, YQ, LR, NC
Valve HR has flow rate=14; tunnels lead to valves OX, GO, JJ
Valve WE has flow rate=0; tunnels lead to valves DW, BC
Valve MW has flow rate=0; tunnels lead to valves JQ, PD
Valve DO has flow rate=0; tunnels lead to valves JS, AA
Valve PC has flow rate=0; tunnels lead to valves AK, KX
Valve YD has flow rate=0; tunnels lead to valves CA, OF
Valve RX has flow rate=0; tunnels lead to valves XG, BC
Valve CM has flow rate=0; tunnels lead to valves IR, OM
Valve HX has flow rate=6; tunnels lead to valves ZQ, NQ, AF, FD, CA
Valve ZQ has flow rate=0; tunnels lead to valves KH, HX
Valve BV has flow rate=21; tunnels lead to valves SY, OX, EI
Valve AK has flow rate=0; tunnels lead to valves PC, OQ
Valve UE has flow rate=0; tunnels lead to valves MS, JQ
Valve LR has flow rate=0; tunnels lead to valves BC, EJ
Valve JQ has flow rate=8; tunnels lead to valves MW, UE, BF, GW
Valve VL has flow rate=0; tunnels lead to valves PU, ZX
Valve EV has flow rate=0; tunnels lead to valves JJ, PD
Valve TS has flow rate=0; tunnels lead to valves NC, ZX
Valve RZ has flow rate=0; tunnels lead to valves SM, WU
Valve OF has flow rate=13; tunnels lead to valves XG, YD, NB, MS, XN
Valve WW has flow rate=0; tunnels lead to valves SM, IR
Valve TY has flow rate=0; tunnels lead to valves HO, AA
Valve XN has flow rate=0; tunnels lead to valves OF, PD
Valve SG has flow rate=15; tunnels lead to valves WU, YQ
Valve FN has flow rate=25; tunnel leads to valve PH
Valve KL has flow rate=0; tunnels lead to valves TN, OQ
Valve ZX has flow rate=5; tunnels lead to valves JS, HP, VL, NQ, TS".to_string()
    }
}
