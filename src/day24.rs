use std::collections::HashSet;
use std::mem::swap;

fn solve1(data: String) -> usize {
    let parsed = parse(data);
    let mut current = HashSet::new();
    let mut next = HashSet::new();
    let rows = parsed.len();
    let columns = parsed[0].len();
    
    current.insert((0,1));
    
    for minute in 1.. {
        for pos in current.iter() {
            if pos.0 == rows -2 && pos.1 == columns -2 {
                return minute;
            }
            if pos.0 > 0 && free(&parsed, minute, (pos.0-1, pos.1)) {
                next.insert((pos.0-1, pos.1));
            }
            if free(&parsed, minute, (pos.0+1, pos.1)) {
                next.insert((pos.0+1, pos.1));
            }
            if free(&parsed, minute, (pos.0, pos.1)) {
                next.insert((pos.0, pos.1));
            }
            if free(&parsed, minute, (pos.0, pos.1+1)) {
                next.insert((pos.0, pos.1+1));
            }
            if free(&parsed, minute, (pos.0, pos.1-1)) {
                next.insert((pos.0, pos.1-1));
            }
        }
        

        swap(&mut current, &mut next);
        next.clear();
    }
    
    panic!("unreachable");
}

fn free(map: &Vec<Vec<u8>>, minute: usize, pos: (usize, usize)) -> bool {
    let rows = map.len();
    let columns = map[pos.0].len();
    map[pos.0][pos.1] != b'#'
        && map[pos.0][wrap_n(pos.1, minute, columns)] != b'>'
        && map[pos.0][wrap(pos.1 + minute, columns)] != b'<'
        && map[wrap(pos.0 + minute, rows)][pos.1] != b'^'
        && map[wrap_n(pos.0, minute, rows)][pos.1] != b'v'
}

fn wrap(num: usize, len: usize) -> usize {
    (num - 1) % (len -2) + 1 
}

fn wrap_n(num: usize, subtract: usize, len: usize) -> usize {
    if subtract + 1 > num {
        len - 1 - (subtract + 1 - num) % (len - 2) 
    } else {
        (num - 1 - subtract) % (len - 2) + 1
    }
}

fn solve2(data: String) -> usize {
    let parsed = parse(data);
    let mut current = HashSet::new();
    let mut next = HashSet::new();
    let rows = parsed.len();
    let columns = parsed[0].len();

    current.insert((0,1,0));

    for minute in 1.. {
        for pos in current.iter() {
            if pos.0 == rows -2 && pos.1 == columns -2 && pos.2 == 2 {
                return minute;
            }
            let state = match pos {
                (r, c, 0) if *r == rows-1 && *c == columns-2 => 1,
                (0, 1, 1) => 2,
                _ => pos.2
            };
            
            if pos.0 > 0 && free(&parsed, minute, (pos.0-1, pos.1)) {
                next.insert((pos.0-1, pos.1, state));
            }
            if pos.0 < rows - 1 && free(&parsed, minute, (pos.0+1, pos.1)) {
                next.insert((pos.0+1, pos.1, state));
            }
            if free(&parsed, minute, (pos.0, pos.1)) {
                next.insert((pos.0, pos.1, pos.2));
            }
            if free(&parsed, minute, (pos.0, pos.1+1)) {
                next.insert((pos.0, pos.1+1, pos.2));
            }
            if free(&parsed, minute, (pos.0, pos.1-1)) {
                next.insert((pos.0, pos.1-1, pos.2));
            }
        }


        swap(&mut current, &mut next);
        next.clear();
    }

    panic!("unreachable");}

fn parse(data: String) -> Vec<Vec<u8>> {
    data.split("\n").map(|l| l.chars().map(|c| c as u8).collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = solve1(input_example());
        assert_eq!(result, 18);
    }

    #[test]
    fn exercise1() {
        // >285
        let result = solve1(input_exercise());
        assert_eq!(result, 326);
    }

    #[test]
    fn example2() {
        let result = solve2(input_example());
        assert_eq!(result, 54);
    }

    #[test]
    fn exercise2() {
        let result = solve2(input_exercise());
        assert_eq!(result, 976);
    }

    fn input_example() -> String {
        "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#".to_string()
    }

    fn input_exercise() -> String {
        "#.######################################################################################################################################################
#>v<<.^vvv^vv.^^>>v<>><v^<<.>^>v>v<v^vvvvvv^><^<vv<^^^v<vv>vvv.^v>.<>>v^v><<>^<<^vv<<>><<v<v<^v<..^>>vv^><<<v<^^><<<>>>vvv^.^.v>.^v^^.vv>^vv^>^^^><v<<<#
#.<<v^><<.<>>>>^.<^<^^<>>^.^^.^v<vvv>>vv<^>.>>^^<<<^^>><^><<<v^>v.^^<.<<^v^>^^>.^.v^>.><<vv^>><>^....^<vv^><<^>v<v<^vvv<v<<><^vv<^>v^^^^^^^.^vv<^^<vvv<#
#<>.v^.v<^><><v^^>>^><.v>^v^vv><><>>^>^v<^>>v^>^vvv^v..>vv>.^vv^<^.<.<v>>^<><v>^>^v..^>^><>vvv<v>^.>>^^<v^v>^<^vv.>.^v><v>v^>.^v<v>.<<vvv><^v^<^<^<vv<>#
#>v..v^<^^v<><^^>vv>v^^^<^.^<<><^^^v>^<^v<<^>^<<.^><<^>>v^.v.><>>^<<><^^>>v>.^<^<vv><<>..>>v^^^>v>^<vv>.^>^<.^^v<>v>^.^>^^>.^v^>>^v>>>^<<^.^>><>v^>v>v.#
#>^^^^^v^>>>^^v^v^v>v^v<v^.><<.^..v<^<^<^><<><<<>>v<<>><<^v^^>v>v<>.<vv>^<<<><.v<v<>>v^v<>>^v><.v>><<v><v<vv<<^^<><.^>v>^<^<^>^^.<.v>v^>>>><<<^v.vv<.<>#
#>vv<vv^v<.<<v><<<^v>v<><vv<<vv<<.^.v>>v>.>v<^<<>v>>vv>..>v<^.v<.^<.>.<<<>>>^v>.^>>>>><<<^>..>>>^<^<>^>><>^><><v>v<<<v^.vvvv<v>>vv^>>>^.v^<>^><.<v>>v>>#
#>>v.>><v^>v^v.<^.<vvv<<<v<>.^^v<<^^>>^^v^^.^.<>^vv^>>v<>^>v^.vv^v>.v^<>>v<<v<^>^vv^^v^><>v>.^<v<v<.^>>vvv>v^<^vv^<v^<<>^<v<>v<<v<<>v^v><>^vv^<<<^v>^>>#
#<^<^>><<.<<.><<v>vv>v^v<<>..v<^>v^<.vv><v^<<<.v>v>>><<^.<v^vvv^<^v<<^^^^<v<<.<<^><v.<>>^><v>..<><vvv<.<>><.v>.v><<>vv<>.<<v.>><>>v<v^>^>>..>.<<^>^>^.>#
#<v.>v>v<>>v^v>^v<^vv>^^>^vv>v<^^vv><^.<v<^<v>^>^v<v.<^<>^>>>vvvv<v>>^^<v.^v<^^<.<<<><v^<>^^<vvv><<>...^<><>^>vvv^^v..v..v^><^<>^>vv><v<^^vv^vvv>v^<v><#
#<^^^^.>v>><<>v<vv<^<<^>v..vv^vvv>>>v<<^vv^^><^>^>^v.<<^v^^><vv>.>v..v.^v><^<^>>^>vv<<v.v>.<.>^.>>>^>>^^v^>^<.<><v>v^.^..<>.>^<^^<v.^v>^^^<<^v>>.>vv<v>#
#>^^v>v.<v<<v..>><>..^v><>.^<v.>vv<>v^^>v.^v<<<vv>>v.^>><<^<v><v^.>>>^v>^>>v<<...<^^^vv.v^><<<>^^v<v^>v<v>.vv^vv.>>v<>><>v><vv>^>>v^..vv>v>^^>vvv>.>^v<#
#<v>^^.<.^vv^<>^vv<<<<<vv^<^^<>v>^vv<>v<<>vv^<<v><<.^^..>.^>vv<^<>>^.<<<<<>.v>v><>.>v>v^<>^>vvv>.<^v^.>^<<v<^vvv^>v><<v^>.^><^><>v.^^<><vv<>.v..^<<v^>>#
#<>^.>.><><v^<v>v>^<<>^v..<.<><>>^.vv>v.vv.>><>v^>.^><.<v.>v..^^v.^v^>^<^^<>>>v<^>>v<^<v^^^v<^v.vv<v^.><^^v^>^^^<.>.<v<.<v>^.><^^<v^<<^^v<.>>v>.vvv>v>>#
#<v><v.^^<>v>.>^^^^>v<<v..v>><>v^.>^^<^>v<>v^v<v^<><^<>v>v^.<.^v^<^<>v^^<<^v><^.>^.v><><>>^v^v><.>v><<>v>v^.vvv><>^>>v^<<^v>^vv><><.^^>>vv^^v><^..>^v<>#
#>v><^<vv^v>v><>v^<<><<^>v^v<^<^<v>><v^vv<<.vv<^v.><<<v.v.^<v<v.^^>><>^vv>v<v^>v^.<<><^<<.<vv<^.^>^^vv<^vv^..<^>^^<^^vvvv<><^^^<v<vv<v^>^v<^v><^^<..<v>#
#>v^<<><<<^v.vvv>v<<v>>.^>^<^><..>>>^v.v.<<v<>>^^..><<v>vv^^<v<<^<>^v<^<<^.v>^<^>vv.>>>^>^^.<^<v<<^^v^^<><.>^.<><vv^<^>^>>^^vv^<^^^<^>^.v^<.<^^<.>v>v>>#
#>^<v>v^<^>>..v<.><<<^><^v>vv<>^^v<v^vv><<<><^v<^.v<^>v<v<^<v>v>^v^^>>^<<^.<<v<vvv><^>v>^<>v>v^^>.<vv<<>>^^<>vv>^v^.><^^^vv>v<<<<v^v<vv><.<v<^v>>^v>v<>#
#<vv^>^^<v>^v>>.^.<<^<^^.v>>vv>^<><><>>^>>>vv<.v<^<<<vvv.>v<>>^>>v<>>..>^v^>^>^v<<>v^v^.^>^v^^v>.><v<<.v<v>vv>>.<<v.v.^<>><>vv>^^>><^^.<>>>vv<<>^>><<<.#
#<^<^^.>>>^<>^v<^v<<>v^<>^vv<^^v<v<<^^.><<vv>v^<^^<>^^^v<v>^.^^<<^v<v^.^>>>><v>><>>>><v<^^<v>v^v>^>v>v^^^<>v><v><.>><>^vvvvv>vv<<v^^<v^v>v.^v<vv<^.^^v<#
#..>^>>^^^^v<>>^><^^v>^^>v.><>><vv<><^.<><^v.^vv<><v>v^<<v>v<vv<><>v^.^<<<vv>>^^<<>^.v<^.v^^<>>>^v<.>^..^^^<<<>..v<><v<>v<>>..<v><>^^>>.<>v<<>v>.<v^vv.#
######################################################################################################################################################.#".to_string()
    }
}
