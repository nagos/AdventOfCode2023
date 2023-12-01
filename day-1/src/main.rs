use std::fs;

fn main() {
    let data = fs::read_to_string("data/day-1-input.txt").unwrap();
    let answer = proc(&data);
    println!("Day 1 aswer: {answer}")
}

fn proc_line(line: &str) -> u32 {
    let mut store = vec![];
    for i in line.chars() {
        if let Some(d) = i.to_digit(10) {
            store.push(d);
        }
    }

    let ret = store.first().unwrap() * 10 + store.last().unwrap();

    ret
}

fn proc(data: &str) -> u32 {
    let mut ret = 0;
    for l in data.lines() {
        if !l.is_empty() {
            let d = proc_line(l);
            ret += d;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_test() {
        assert_eq!(proc_line("1abc2"), 12);
        assert_eq!(proc_line("pqr3stu8vwx"), 38);
        assert_eq!(proc_line("a1b2c3d4e5f"), 15);
        assert_eq!(proc_line("treb7uchet"), 77);
    }
    #[test]
    fn test_file() {
        let data = fs::read_to_string("data/day-1-test.txt").unwrap();
        assert_eq!(proc(&data), 142);
    }
}
