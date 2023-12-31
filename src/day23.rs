﻿use std::collections::{HashMap, HashSet};

static EMPTY: i32 = -999999;

fn solve1(data: String) -> i32 {
    let mut parsed = parse(data);
    let mut moves = HashMap::new();
    
    for round in 0..10 {
        for (x, y) in parsed.iter() {
            let north = !parsed.contains(&(x-1, y-1)) && !parsed.contains(&(x-1, *y)) && !parsed.contains(&(x-1, y+1));
            let south = !parsed.contains(&(x+1, y-1)) && !parsed.contains(&(x+1, *y)) && !parsed.contains(&(x+1, y+1));
            let west = !parsed.contains(&(x-1, y-1)) && !parsed.contains(&(*x, y-1)) && !parsed.contains(&(x+1, y-1));
            let east = !parsed.contains(&(x-1, y+1)) && !parsed.contains(&(*x, y+1)) && !parsed.contains(&(x+1, y+1));
            if north && east && south && west || !north && !east && !south && !west { continue; }
            let order = [north, south, west, east];
            if order[round % 4] { add_move(&mut moves, round % 4, (*x, *y))}
            else if order[(round + 1) % 4] { add_move(&mut moves, (round + 1) % 4, (*x, *y))}
            else if order[(round + 2) % 4] { add_move(&mut moves, (round + 2) % 4, (*x, *y))}
            else if order[(round + 3) % 4] { add_move(&mut moves, (round + 3) % 4, (*x, *y))}
        }
        
        for (new, old) in &moves {
            if *old == (EMPTY, EMPTY) { continue;}
            parsed.insert(*new);
            parsed.remove(old);
        }
        
        moves.clear();
    }
    
    let x_min = parsed.iter().map(|(x, _)| x).min().unwrap();
    let x_max = parsed.iter().map(|(x, _)| x).max().unwrap();
    let y_min = parsed.iter().map(|(_, y)| y).min().unwrap();
    let y_max = parsed.iter().map(|(_, y)| y).max().unwrap();
    (x_max - x_min + 1) * (y_max - y_min + 1) - parsed.len() as i32
}

fn add_move(moves: &mut HashMap<(i32, i32), (i32, i32)>, direction: usize, position: (i32, i32)) {
    let pos = match direction {
        0 => (position.0 - 1, position.1),
        1 => (position.0 + 1, position.1),
        2 => (position.0, position.1 - 1),
        3 => (position.0, position.1 + 1),
        _ => panic!("Invalid direction"),
    };
    
    if moves.contains_key(&pos) {
        moves.insert(pos, (EMPTY, EMPTY));
    } else {
        moves.insert(pos, position);
    }
}


fn solve2(data: String) -> usize {
    let mut parsed = parse(data);
    let mut moves = HashMap::new();

    for round in 0.. {
        for (x, y) in parsed.iter() {
            let north = !parsed.contains(&(x-1, y-1)) && !parsed.contains(&(x-1, *y)) && !parsed.contains(&(x-1, y+1));
            let south = !parsed.contains(&(x+1, y-1)) && !parsed.contains(&(x+1, *y)) && !parsed.contains(&(x+1, y+1));
            let west = !parsed.contains(&(x-1, y-1)) && !parsed.contains(&(*x, y-1)) && !parsed.contains(&(x+1, y-1));
            let east = !parsed.contains(&(x-1, y+1)) && !parsed.contains(&(*x, y+1)) && !parsed.contains(&(x+1, y+1));
            if north && east && south && west || !north && !east && !south && !west { continue; }
            let order = [north, south, west, east];
            if order[round % 4] { add_move(&mut moves, round % 4, (*x, *y))}
            else if order[(round + 1) % 4] { add_move(&mut moves, (round + 1) % 4, (*x, *y))}
            else if order[(round + 2) % 4] { add_move(&mut moves, (round + 2) % 4, (*x, *y))}
            else if order[(round + 3) % 4] { add_move(&mut moves, (round + 3) % 4, (*x, *y))}
        }

        if moves.is_empty() { return round + 1; }
        
        for (new, old) in &moves {
            if *old == (EMPTY, EMPTY) { continue;}
            parsed.insert(*new);
            parsed.remove(old);
        }

        moves.clear();
    }
    
    panic!("loop should be infinite")
}

fn parse(data: String) -> HashSet<(i32, i32)> {
    let mut elves = HashSet::new();
    data.split("\n").enumerate().for_each(|(i, l)| l
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .for_each(|(j, _)| {elves.insert((i as i32, j as i32)); }));

    elves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1a() {
        let result = solve1(input_example1a());
        assert_eq!(result, 25);
    }

    #[test]
    fn example1() {
        let result = solve1(input_example());
        assert_eq!(result, 110);
    }

    #[test]
    fn exercise1() {
        let result = solve1(input_exercise());
        assert_eq!(result, 3931);
    }

    #[test]
    fn example2a() {
        let result = solve2(input_example1a());
        assert_eq!(result, 4);
    }

    #[test]
    fn example2() {
        let result = solve2(input_example());
        assert_eq!(result, 20);
    }

    #[test]
    fn exercise2() {
        let result = solve2(input_exercise());
        assert_eq!(result, 944);
    }

    fn input_example1a() -> String {
        ".....
..##.
..#..
.....
..##.
.....".to_string()
    }

    fn input_example() -> String {
        "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............".to_string()
    }


    fn input_exercise() -> String {
        "#...##..#.#.#.#...#....###.#.#.#.##..###.#.##.#.#...#.##.#.#.#########.#
#..#..##....###.###...#.#...#..#......#...#..###.####.#..#####..#.##...#
.#..##..#..#...#.##.#..#..######..##.#..#.##...#.#..#..#.###.#.#..#.....
###.#..#.#...#.#####.#..##.#.##.#.#.#.##.##...####..#....##...#.######..
....######....##.#.##.#.##.#..#.#.#.#.....###.###.#..#....#..##...#..#.#
#..###...#..##...#######....#.#..#.#..####...#..#.##..##.##.#.##.#.##..#
#.#.######.#.###....#.#.#..#.#...##.#.##..#..#####.##.#...#..###...###.#
..#..#.#.#.#.....#..#.#.##..####.#.#.###..#....#..##.##..#...####.#.#.#.
#.#.#.....#.##..........#.#.#...#.......#..##...##.#..#..###..##..#.#..#
.#.#...#..##..#...#.##...#.#.#.#.##.####.....#...##..#..##....##...#####
#.##.#.#..#####.#.#....#.#..#...#....##.##.##.#..##.##.#.###..##...#.###
#.#.##.....##..#.#....######.###...##..##...#.#..##.###..###....#.#.###.
...###..#..##.#.#.#..##.#..#...#.#...#.###.####.....#..###.#..##..#####.
..##.....##..##..###.##.###..#####.#..#.#.###..#.###.####..#.###..#....#
#########.#..##.#.##...##.#.#.#......##...###..#.......##..#.###...#...#
.#...#.....##......#..##.##.#......#.####..#...#.##..#..##..###.###...##
###....#..#.#...##...#...#.#......#.##..#.##..####.#....#....##.##..#...
#####.#..#.#..#.#.#.###.###.##...##..##..#..###...######..#..#.#..###...
.....#...#.#..#.####.#..#....###..#...##..#.....#..####....#.#.#..##....
#..#.#.#.#...#####.#...#.#####......#.##.####...#....##.#..#.##.##...###
##..#.##.##...######.#.##....#.#...##..#.#.##.#.#...#...#.#..###...###.#
....###..#..#.##...#...#.##.###..#..#.##.#.####...###...######.###.###..
#......#..#.#.####....###....#.###.#.###..##..####.#....###...#.#.......
#...##...##..##.#...###.#.#.#....##.###..#..#####.#.#...##....########.#
#..####..#.#........#####..#.#.###..#.###..#..#....#......#.#.......#...
#.#.#.#.#...#.##..#.....#..#...#.##..#.########......##.#.#.#####..#....
######..##..#.#.....#..##.#.##.#.#.####..##..#..##.##.#..#.#.####.#.#.#.
....##.#.#.......###.##...#..###.#.##.#..##..#####.#.##...#..###.#.##.#.
..##..##..#.#..##...#.#.####.......###..##..###.#.##..#..#.#.#####...##.
...#....####.##..#..####..#.#..###.....##.##.#..#....#####..####.#.....#
#.##.#.#..#.##..#.#####..#..###..##....#...#..#...#..#.##.###.#.....#...
..#.#.#.##...##.#....##...#..##.#.#..#..#....###...##....#....#.#..##..#
..###.#####..#...#.###...#..#.###.##....#.#.....###.##..#####.#.#.##..#.
#.###..##.###.#..#...##..#..#...#.#.....#.#..##...##.###.#.###...###..##
###.....#.#........#.####.#.#.#.#..#.###.##....##.#.#..##.##...##..#....
#.#.##.###....#.###...##..#####....#####..###..##.###.#..####.##..#.....
##.#..##.#.#.##..#######.##...##...###..##..####.#.#.###..#.###.#.###..#
...####.#..#.##...#..##..####.#....#.##...###..#.##.####.....#.....#.###
#..#.....#.########..###....#.#.#....##....#.###..#.....###.##.###....##
#.#.#.#....#.##.#.#...#.#.#.#.#...#..#.##......#.#...#...#######.##....#
##.########.####.####.#.##.#.#.##...#.#.###..#.###....#.##...##...#..##.
..###...##...##.#...###.###.###....###...###.###...#.............####.#.
..#....#..#.##.#..#.##.#.####.##....#....#.##.#..#..##.##.###..#.#..###.
..##...#.#...#..#.#...........#..######.#.##...##.#..#....#.##.#######.#
##.#.####.#.##..####..####..#.#.#######....#.#..#.#...#.#.#...#####.#.##
#.....#.#.##.#..##..#####.##.#.#.#.#......###.##..##.#####..######.#..#.
.###....##..##.#.####..#.#.#.####..###...#.##...###..#..#....##.#...#..#
##.######....#.#....#.##..#.##......##.#..#.###..#..##.##.###...#..#....
..#.##.##..##...#...##..#.###....######....#.#....###.#####...#.#######.
###..#.#.###.#..#..###.....#...#..##.....#.##...#.#######.##.#..##......
#....##.#...#####...##.#..######.####..####..#..#.##.######..##.#.######
#.#.....#..##..#.#.#..##.##....######..####..###...####..#.#.##########.
.#.#....##..##.###..####......#.....###........##.#...#..###.#......#.##
#...##...###.#.#....#.####.#..#.##.#.##........####...#..##.##.#...##.#.
##.#####.#####...#.#####..#..#.##..##.#.##.###.#..###..####.#####.#...#.
.#.#####...#.#.##....#.#.########...#...###..##.#.##....#..#####.######.
.#..#.###....#....#.#####.####.#.##..#.###.##.#...#....##.##.#.#..#..##.
..###..##.#.####..#.#.#...#.#.#..#..###..##.#..###.#...#..###....#.#.###
##.##....#########..###..#..##...#.#.##..####....#####.#.....##...#.#.##
..###.........#.##.#...##.##.###.#.###.#.#.#...##.###..#......##....#.#.
...#.#######.##..###..#.#.###..##...###.#..##...##.######.##..#..#....#.
..#...#....##.##.##..#.#...##.....#..#######....#...####.#.###..#..#.###
.#..#.###..#.##.#....#.#..##.#...##..#..####..##..#..#.##.#..####...#..#
#.##..##..#.###..#...#.##...#.##..###.....####..##..#.##.###.#.#.##.....
..#.##..#####......###.#..#.#..##.##.##.###.#..##.####....#.#.#....###.#
..##...#......##.##....#.##.###.#.#.#.##...###....#.#...#..#....###.....
...####.#..#.#.#....##.####.#.##.#..###.#....#.....#.#..#.#...##########
##.#####......##.########....#.#.#.#.#.####..##.#..###..#.#..##...#..#.#
#.#...##.##..#####.#...###.##..#.#....####.##...#.####..####..#...##..##
.##..##.#####.#.#.#.##.#.#....#....##.#############....#######..###.##.#
.#...###..##.#....##.####..###.##...#...#.#.####..###.###.#...##...##.#.
#...####.#.....##.....##.###.##.####.#..#.#..##.##.#.#...#..#.##..###.##".to_string()
    }
}
