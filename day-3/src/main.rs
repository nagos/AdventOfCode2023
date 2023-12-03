use std::fs;

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let (digits, symbols) = data_parser(&data);
    let res = find_items(digits, symbols);
    println!("Day 3 part one: {res}");
}

#[derive(Debug)]
enum ParserState {
    Start,
    Digit,
    Symbol,
}

#[derive(Debug)]
struct Digits {
    value: u32,
    x: u32,
    x_end: u32,
    y: u32,
}

#[derive(Debug)]
struct Symbols {
    x: u32,
    y: u32,
}

fn data_parser(input: &str) -> (Vec<Digits>, Vec<Symbols>) {
    let mut state = ParserState::Start;
    let mut number_tmp = String::default();
    let mut number_start = 0;
    let mut x = 0;
    let mut y = 0;
    let mut itr = input.chars().peekable();

    let mut digits: Vec<Digits> = vec![];
    let mut symbols: Vec<Symbols> = vec![];

    while let Some(c) = itr.peek() {
        let mut repeat = false;
        match state {
            ParserState::Start => {
                if c.is_ascii_digit() {
                    state = ParserState::Digit;
                    number_tmp.clear();
                    repeat = true;
                    number_start = x;
                } else if *c != '.' && *c != '\n' {
                    state = ParserState::Symbol;
                    repeat = true;
                }
            }
            ParserState::Digit => {
                if c.is_ascii_digit() {
                    number_tmp.push(*c);
                } else {
                    state = ParserState::Start;
                    digits.push(Digits {
                        value: number_tmp.parse().unwrap(),
                        x: number_start,
                        y,
                        x_end: number_start + (number_tmp.len() - 1) as u32,
                    });
                    repeat = true;
                }
            }
            ParserState::Symbol => {
                symbols.push(Symbols { x, y });
                if *c != '.' {
                    state = ParserState::Start;
                }
            }
        }
        if !repeat {
            if *c == '\n' {
                x = 0;
                y += 1;
            } else {
                x += 1;
            }
            itr.next();
        }
    }

    (digits, symbols)
}

fn find_items(digits: Vec<Digits>, symbols: Vec<Symbols>) -> u32 {
    let mut ret = 0;
    for d in digits {
        for s in &symbols {
            // left - right from symbol
            if d.y == s.y && (d.x_end + 1 == s.x || d.x == s.x + 1) {
                ret += d.value;
                break;
            }
            // up - down
            if s.y != 0
                && (d.y + 1 >= s.y && d.y <= s.y + 1)
                && (s.x + 1 >= d.x && s.x <= d.x_end + 1)
            {
                ret += d.value;
                break;
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (digits, symbols) = data_parser(&data);
        let res = find_items(digits, symbols);
        assert_eq!(res, 4361);
    }
}
