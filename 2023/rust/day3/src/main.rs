use std::collections::HashMap;

fn main() {
    let file = include_str!("../input/input.txt");
    let res = process_part1(file);
    println!("Part 1: {res}");
}

fn process_part1(_input: &str) -> String {
    todo!()
}

enum Value {
    Empty,
    Symbol(char),
    Number(i32),
}

struct Coord {
    x: i32,
    y: i32,
}

fn read_file(input: &str) {
    let char_list = HashMap::<Coord, Value>::new();
    input.lines()
        .map(|line| line.chars().for_each(|c| {
            use Value as V;
            match c {
                '.' => V::Empty,
                ch if  ch.is_ascii_digit() => V::Number(ch),
                ch if ch.is_ascii_alphabetic() => V::Symbol(ch),
                _ => unreachable!(),
            };
        }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error;

    #[test]
    fn test_part1() -> Result<(), Error> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process_part1(input));
        Ok(())
    }
}
