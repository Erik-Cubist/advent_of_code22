
fn solve1(data: String) -> i32 {
    let cycles = parse(data);
    
    let mut signal = 0;
    for i in (20..221).step_by(40) {
        signal += cycles.get(i).unwrap() * (i as i32);
    }
    
    assert!(cycles.len() < 260);
    signal
}

fn solve2(data: String) -> String {
    let cycles = parse(data);
    let width:i32 = 40;
    let hight:i32 = 6;
    let mut display = String::new();
    for y in 0..hight {
        for x in 1..=width {
            let index = cycles[(x + width * y) as usize];
            display += if x > index + 2 || index > x { "." } else { "#" };
        }
        display += "\n";
    }
    
    display
}

fn parse(data: String) -> Vec<i32> {
    let lines = data.split("\n").map(|line| line.split(" "));
    let mut x = 1;
    let mut cycles = vec![1];
    for line in lines {
        cycles.push(x);
        match line.collect::<Vec<&str>>().as_slice() {
            ["noop"] => (),
            ["addx", v] => { cycles.push(x); x += v.parse::<i32>().unwrap(); },
            _ => panic!("unknown instructions")
        }
    }
    cycles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = solve1(input_example());
        assert_eq!(result, 13140);
    }

    #[test]
    fn exercise1() {
        let result = solve1(input_exercise());
        assert!(result < 13940);
        assert_eq!(result, 10760);
    }

    #[test]
    fn example2() {
        let result = solve2(input_example());
        println!("{}", result);
        assert_eq!(result, "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
");
    }

    #[test]
    fn exercise2() {
        let result = solve2(input_exercise());
        println!("{}", result);
        assert_eq!(result, "####.###...##..###..#..#.####..##..#..#.
#....#..#.#..#.#..#.#..#.#....#..#.#..#.
###..#..#.#....#..#.####.###..#....####.
#....###..#.##.###..#..#.#....#.##.#..#.
#....#....#..#.#....#..#.#....#..#.#..#.
#....#.....###.#....#..#.#.....###.#..#.
");
    }

    fn input_example() -> String {
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop".to_string()
    }


    fn input_exercise() -> String {
        "addx 1
addx 4
noop
noop
addx 30
addx -24
addx -1
noop
addx 4
addx 1
addx 5
addx -4
addx 5
addx 4
addx 1
noop
addx 5
addx -1
addx 5
addx 3
noop
addx -38
addx 9
addx -4
noop
addx 3
noop
addx 2
addx 3
noop
addx 2
addx 3
noop
addx 2
addx 3
noop
addx 2
addx -17
addx 22
addx -2
addx 5
addx 2
addx 3
addx -2
addx -36
noop
addx 5
noop
addx 3
noop
addx 2
addx -5
noop
addx 10
addx 3
addx -2
addx 3
addx 2
addx 4
noop
noop
noop
noop
addx 3
noop
noop
addx 7
addx 1
noop
noop
addx -38
addx 39
addx -32
noop
noop
noop
addx 5
addx 2
addx -1
addx 4
noop
addx 5
addx -2
addx 5
addx 2
addx -26
addx 31
addx -2
addx 4
addx 3
addx -18
addx 19
addx -38
addx 7
noop
noop
addx 34
addx -39
addx 8
addx 5
addx 2
addx 10
addx -5
addx -2
addx 5
addx 2
addx 11
addx -6
noop
addx 3
noop
addx 2
addx 3
addx -2
addx -38
noop
noop
noop
addx 5
addx 11
noop
addx -3
noop
noop
noop
addx 2
noop
addx -11
addx 16
noop
addx 3
addx 2
addx 8
noop
noop
noop
noop
noop
addx 4
addx 3
noop
addx -20
noop".to_string()
    }
}
