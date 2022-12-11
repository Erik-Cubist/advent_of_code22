use std::cell::RefCell;
use regex::Regex;

struct Monkey {
    items: Vec<u64>,
    square: bool,
    addition: u64,
    multiplication: u64,
    test: u64,
    true_target: usize,
    false_target: usize,
    inspections: usize
}

fn solve1(data: String) -> usize {
    let mut monkeys = parse(data);
    
    for _rounds in 0..20 {
        for mut monkey in monkeys.iter().map(|m| m.borrow_mut()) {
            for item in monkey.items.iter() {
                let new_item = (item * if monkey.square {item} else { &monkey.multiplication} + monkey.addition) / 3;
                let new_index = if new_item % monkey.test == 0 {monkey.true_target} else { monkey.false_target};
                monkeys[new_index].borrow_mut().items.push(new_item);
            }
            monkey.inspections += monkey.items.len();
            monkey.items.clear();
        }
    }
    
    monkeys.sort_by_key(|m| -(m.borrow().inspections as i64));
    let x = monkeys[0].borrow().inspections * monkeys[1].borrow().inspections;
    x
}

fn solve2(data: String) -> usize {
    let mut monkeys = parse(data);
    let modulus: u64 = monkeys.iter().map(|m| m.borrow().test).product();

    for _rounds in 0..10000 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].borrow_mut();
            for item in monkey.items.iter() {
                let new_item = (item * if monkey.square {item} else { &monkey.multiplication} + monkey.addition) % modulus;
                let new_index = if new_item % monkey.test == 0 {monkey.true_target} else { monkey.false_target};
                monkeys[new_index].borrow_mut().items.push(new_item);
            }
            monkey.inspections += monkey.items.len();
            monkey.items.clear();
        }
    }

    monkeys.sort_by_key(|m| -(m.borrow().inspections as i64));
    let x = monkeys[0].borrow().inspections * monkeys[1].borrow().inspections;
    x
}

fn parse(data: String) -> Vec<RefCell<Monkey>> {
    let monkeys = data.split("\n\n").map(|line| line.split("\n"));
    let mut result = Vec::new();
    let operations_re = Regex::new(r"Operation: new = old (.) (.*)").unwrap();
    for mut monkey in monkeys {
        monkey.next();
        let items = monkey.next().unwrap()["  Starting items: ".len()..].split(", ").map(|item| item.parse().unwrap()).collect();
        let captures = operations_re.captures(monkey.next().unwrap()).unwrap();
        let op = &captures[1];
        let val = &captures[2];
        let (square, add, mul) = if val == "old" {
            (true, 0u64, 1u64)
        } else if op == "*" {
            (false, 0, val.parse::<u64>().unwrap())
        } else {
            (false, val.parse::<u64>().unwrap(), 1)
        };
       let test = monkey.next().unwrap()["  Test: divisible by ".len()..].parse::<u64>().unwrap();
       let true_target = monkey.next().unwrap()["    If true: throw to monkey ".len()..].parse::<usize>().unwrap();
       let false_target = monkey.next().unwrap()["    If false: throw to monkey ".len()..].parse::<usize>().unwrap();
        
        result.push(RefCell::new(Monkey { items, square, addition: add, multiplication: mul, test, true_target, false_target, inspections: 0 }));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = solve1(input_example());
        assert_eq!(result, 10605);
    }

    #[test]
    fn exercise1() {
        let result = solve1(input_exercise());
        assert_eq!(result, 54036);
    }

    #[test]
    fn example2() {
        let result = solve2(input_example());
        assert_eq!(result, 2713310158);
    }

    #[test]
    fn exercise2() {
        let result = solve2(input_exercise());
        assert_eq!(result, 13237873355);
    }

    fn input_example() -> String {
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1".to_string()
    }


    fn input_exercise() -> String {
        "Monkey 0:
  Starting items: 98, 97, 98, 55, 56, 72
  Operation: new = old * 13
  Test: divisible by 11
    If true: throw to monkey 4
    If false: throw to monkey 7

Monkey 1:
  Starting items: 73, 99, 55, 54, 88, 50, 55
  Operation: new = old + 4
  Test: divisible by 17
    If true: throw to monkey 2
    If false: throw to monkey 6

Monkey 2:
  Starting items: 67, 98
  Operation: new = old * 11
  Test: divisible by 5
    If true: throw to monkey 6
    If false: throw to monkey 5

Monkey 3:
  Starting items: 82, 91, 92, 53, 99
  Operation: new = old + 8
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 2

Monkey 4:
  Starting items: 52, 62, 94, 96, 52, 87, 53, 60
  Operation: new = old * old
  Test: divisible by 19
    If true: throw to monkey 3
    If false: throw to monkey 1

Monkey 5:
  Starting items: 94, 80, 84, 79
  Operation: new = old + 5
  Test: divisible by 2
    If true: throw to monkey 7
    If false: throw to monkey 0

Monkey 6:
  Starting items: 89
  Operation: new = old + 1
  Test: divisible by 3
    If true: throw to monkey 0
    If false: throw to monkey 5

Monkey 7:
  Starting items: 70, 59, 63
  Operation: new = old + 3
  Test: divisible by 7
    If true: throw to monkey 4
    If false: throw to monkey 3".to_string()
    }
}
