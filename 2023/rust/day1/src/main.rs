fn main() {
    let raw_text = include_str!("../input/input.txt");
    part1(raw_text);
    part2(raw_text);
}

fn part1(txt: &str) -> u32 {
    txt.lines().map(sum_lines).sum()
}

fn part2(txt: &str) -> u32 {
    replace_numerals(txt)
        .lines()
        .map(sum_lines)
        .sum()
}

fn sum_lines(text: &str) -> u32 {
    let bar = text
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<Vec<char>>();

    if let (Some(x),Some(y)) = (bar.first(), bar.last()) {
       format!("{}{}", x, y).parse::<u32>().unwrap()
    } else {
        0
    }
}

fn replace_numerals(line: &str) -> String {
    line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_part1() {
        let input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;
        let res = part1(input);
        assert_eq!(res, 142);

        let input = "3aao4ou0
3buoe2atoeuh0aoeu";
        let res = part1(input);
        assert_eq!(res, 60);
    }

    #[test]
    fn validate_part2() {
       let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let res = part2(input);
        assert_eq!(res, 281);
    }
}
