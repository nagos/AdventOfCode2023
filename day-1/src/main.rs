use std::{fs, vec};

const DIGITS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let data = fs::read_to_string("data/day-1-input.txt").unwrap();
    let answer_1 = proc(&data, false);
    println!("Day 1 part one: {answer_1}");

    let answer_2 = proc(&data, true);
    println!("Day 1 part two: {answer_2}");
}

fn check_digit(line: &str) -> Option<u32> {
    line.chars().next().unwrap().to_digit(10)
}

fn check_letters(line: &str) -> Option<u32> {
    for (i, v) in DIGITS.iter().enumerate() {
        if line.starts_with(v) {
            return Some((i + 1) as u32);
        }
    }
    None
}

fn proc_line(line: &str, words: bool) -> u32 {
    let mut tmp = line;
    let mut store = vec![];
    while !tmp.is_empty() {
        if let Some(d) = check_digit(tmp) {
            store.push(d);
        } else if words {
            if let Some(d) = check_letters(tmp) {
                store.push(d);
            }
        }

        tmp = &tmp[1..];
    }
    store.first().unwrap() * 10 + store.last().unwrap()
}

fn proc(data: &str, letters: bool) -> u32 {
    data.lines().map(|l| proc_line(l, letters)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_test() {
        assert_eq!(proc_line("1abc2", false), 12);
        assert_eq!(proc_line("pqr3stu8vwx", false), 38);
        assert_eq!(proc_line("a1b2c3d4e5f", false), 15);
        assert_eq!(proc_line("treb7uchet", false), 77);
    }

    #[test]
    fn test_file() {
        let data = fs::read_to_string("data/day-1-test.txt").unwrap();
        assert_eq!(proc(&data, false), 142);
    }

    #[test]
    fn test_word_digits() {
        assert_eq!(proc_line("two1nine", true), 29);
        assert_eq!(proc_line("eightwothree", true), 83);
        assert_eq!(proc_line("abcone2threexyz", true), 13);
        assert_eq!(proc_line("xtwone3four", true), 24);
        assert_eq!(proc_line("4nineeightseven2", true), 42);
        assert_eq!(proc_line("zoneight234", true), 14);
        assert_eq!(proc_line("7pqrstsixteen", true), 76);
        assert_eq!(proc_line("twoneight", true), 28);
    }
}
