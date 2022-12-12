use std::collections::HashSet;
use std::mem::swap;

fn solve1(data: String) -> usize {
    let (map, start, end) = parse(data);
    let mut paths: &mut Vec<Vec<(usize, usize)>>  = &mut vec![vec![start]];
    let mut new_paths: &mut Vec<Vec<(usize, usize)>>  = &mut Vec::new();
    let mut visited = HashSet::new();
    visited.insert(start);

    loop {
        for path in paths.iter() {
            for next in allowed_next_steps(&map, path.last().unwrap()) {
                if next == end {
                    return path.len();
                }
                if !visited.contains(&next) {
                    visited.insert(next);
                    let mut new_path = path.clone();
                    new_path.push(next);
                    new_paths.push(new_path);
                }
            }
        }

        swap(&mut paths, &mut new_paths);
        new_paths.clear();
        if paths.is_empty() { panic!("No path found");}
    }
}


fn allowed_next_steps(map: &Vec<Vec<u8>>, point: &(usize, usize)) -> Vec<(usize, usize)> {
    let (l, r) = point;
    let mut result = Vec::new();
    if *l > 0 && map[*l][*r] + 1 >= map[l-1][*r] { result.push((l-1, *r)); }
    if l+1 < map.len() && map[*l][*r] + 1 >= map[l+1][*r] { result.push((l+1, *r)); }
    if *r > 0 && map[*l][*r] + 1 >= map[*l][r-1] { result.push((*l, r-1)); }
    if r+1 < map[*l].len() && map[*l][*r] + 1 >= map[*l][r+1] { result.push((*l, r+1)); }
    
    result
}

fn allowed_previous_steps(map: &Vec<Vec<u8>>, point: &(usize, usize, usize)) -> Vec<(usize, usize)> {
    let (l, r, _) = point;
    let mut result = Vec::new();
    if *l > 0 && map[*l][*r] <= map[l-1][*r] + 1 { result.push((l-1, *r)); }
    if l+1 < map.len() && map[*l][*r] <= map[l+1][*r] + 1 { result.push((l+1, *r)); }
    if *r > 0 && map[*l][*r] <= map[*l][r-1] + 1 { result.push((*l, r-1)); }
    if r+1 < map[*l].len() && map[*l][*r] <= map[*l][r+1] + 1 { result.push((*l, r+1)); }

    result
}

fn solve2(data: String) -> usize {
    let (map, _, end) = parse(data);
    let mut paths: &mut Vec<(usize, usize, usize)>  = &mut vec![(end.0, end.1, 1)];
    let mut new_paths: &mut Vec<(usize, usize, usize)>  = &mut Vec::new();
    let mut visited = HashSet::new();
    visited.insert(end);

    loop {
        for path in paths.iter() {
            for next in allowed_previous_steps(&map, &path) {
                if map[next.0][next.1] == b'a' {
                    return path.2;
                }
                if !visited.contains(&next) {
                    visited.insert(next);
                    new_paths.push((next.0, next.1, path.2 + 1));
                }
            }
        }

        swap(&mut paths, &mut new_paths);
        new_paths.clear();
        if paths.is_empty() { panic!("No path found");}
    }
}

fn parse(data: String) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0,0);
    let map = data.split("\n").enumerate().map(|(l, line)| line.chars().enumerate().map(|(r, c)| match c {
        'S' => {start = (l, r); b'a'},
        'E'=> {end = (l, r); b'z'},
        x => x as u8
    }).collect()).collect();

    (map, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = solve1(input_example());
        assert_eq!(result, 31);
    }

    #[test]
    fn exercise1() {
        let result = solve1(input_exercise());
        assert_eq!(result, 370);
    }

    #[test]
    fn example2() {
        let result = solve2(input_example());
        assert_eq!(result, 29);
    }

    #[test]
    fn exercise2() {
        let result = solve2(input_exercise());
        assert_eq!(result, 363);
    }

    fn input_example() -> String {
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi".to_string()
    }


    fn input_exercise() -> String {
        "abaacccccccccccccaaaaaaaccccccccccccccccccccccccccccccccccaaaaaa
abaaccccccccccccccaaaaaaaaaaccccccccccccccccccccccccccccccccaaaa
abaaaaacccccccccaaaaaaaaaaaaccccccccccccccccccccccccccccccccaaaa
abaaaaaccccccccaaaaaaaaaaaaaacccccccccccccccccdcccccccccccccaaaa
abaaaccccccccccaaaaaaaaccacacccccccccccccccccdddcccccccccccaaaaa
abaaacccccccccaaaaaaaaaaccaaccccccccccccciiiiddddcccccccccccaccc
abcaaaccccccccaaaaaaaaaaaaaaccccccccccciiiiiijddddcccccccccccccc
abccaaccccccccaccaaaaaaaaaaaacccccccccciiiiiijjddddccccaaccccccc
abccccccccccccccaaacaaaaaaaaaaccccccciiiiippijjjddddccaaaccccccc
abccccccccccccccaacccccaaaaaaacccccciiiippppppjjjdddddaaaaaacccc
abccccccccccccccccccccaaaaaaccccccckiiippppppqqjjjdddeeeaaaacccc
abccccccccccccccccccccaaaaaaccccckkkiippppuupqqjjjjdeeeeeaaccccc
abccccccccccccccccccccccccaaccckkkkkkipppuuuuqqqjjjjjeeeeeaccccc
abccccccccccccccccccccccccccckkkkkkoppppuuuuuvqqqjjjjjkeeeeccccc
abcccccccccccccccccccccccccckkkkooooppppuuxuvvqqqqqqjkkkeeeecccc
abccaaccaccccccccccccccccccckkkoooooopuuuuxyvvvqqqqqqkkkkeeecccc
abccaaaaacccccaaccccccccccckkkoooouuuuuuuxxyyvvvvqqqqqkkkkeecccc
abcaaaaacccccaaaacccccccccckkkooouuuuxxxuxxyyvvvvvvvqqqkkkeeeccc
abcaaaaaaaaaaaaacccccccccccjjjooottuxxxxxxxyyyyyvvvvrrrkkkeecccc
abcccaaaacaaaaaaaaacaaccccccjjoootttxxxxxxxyyyyyyvvvrrkkkfffcccc
SbccaacccccaaaaaaaaaaaccccccjjjooottxxxxEzzzyyyyvvvrrrkkkfffcccc
abcccccccccaaaaaaaaaaaccccccjjjooootttxxxyyyyyvvvvrrrkkkfffccccc
abcaacccccaaaaaaaaaaaccccccccjjjooottttxxyyyyywwvrrrrkkkfffccccc
abaaacccccaaaaaaaaaaaaaacccccjjjjonnttxxyyyyyywwwrrlllkfffcccccc
abaaaaaaaaaaacaaaaaaaaaaccccccjjjnnnttxxyywwyyywwrrlllffffcccccc
abaaaaaaaaaaaaaaaaaaaaaaccccccjjjnntttxxwwwwwywwwrrlllfffccccccc
abaaccaaaaaaaaaaaaaaacccccccccjjjnntttxwwwsswwwwwrrlllfffccccccc
abaacccaaaaaaaacccaaacccccccccjjinnttttwwsssswwwsrrlllgffacccccc
abccccaaaaaaccccccaaaccccccccciiinnntttsssssssssssrlllggaacccccc
abccccaaaaaaaccccccccccaaccccciiinnntttsssmmssssssrlllggaacccccc
abccccaacaaaacccccccaacaaaccccciinnnnnnmmmmmmmsssslllgggaaaacccc
abccccccccaaacccccccaaaaacccccciiinnnnnmmmmmmmmmmllllgggaaaacccc
abaaaccccccccccccccccaaaaaacccciiiinnnmmmhhhmmmmmlllgggaaaaccccc
abaaaaacccccccccccaaaaaaaaaccccciiiiiiihhhhhhhhmmlgggggaaacccccc
abaaaaaccccaaccccaaaaaaacaacccccciiiiihhhhhhhhhhggggggcaaacccccc
abaaaaccccaaaccccaaaacaaaaacccccccciiihhaaaaahhhhggggccccccccccc
abaaaaaaacaaacccccaaaaaaaaaccccccccccccccaaaacccccccccccccccccaa
abaacaaaaaaaaaaaccaaaaaaaaccccccccccccccccaaaccccccccccccccccaaa
abcccccaaaaaaaaacccaaaaaaaccccccccccccccccaacccccccccccccccccaaa
abccccccaaaaaaaaaaaaaaaaacccccccccccccccccaaacccccccccccccaaaaaa
abcccccaaaaaaaaaaaaaaaaaaaaaccccccccccccccccccccccccccccccaaaaaa".to_string()
    }
}
