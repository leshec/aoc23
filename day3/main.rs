//why does day 3 feel like day 900?
//well this could be better...

const DATA: &str = include_str!("data.txt");

const OFFSET: usize = 142;

#[derive(Debug)]
struct Elements {
    is_number: bool,
    has_symbol: bool,
    right_symbol: bool,
    further_right: bool,
}

fn main() {
    let mut values: Vec<Elements> = Vec::new();
    let mut sum: Vec<i32> = Vec::new();

    for (index, character) in DATA.chars().enumerate() {
        let mut right_symbol: bool = false;
        let mut further_right: bool = false;
        if index < DATA.len() - 1 {
            right_symbol = check_for_symbol_nearby(index + 1);
        }
        if index < DATA.len() - 2 {
            further_right = check_for_symbol_nearby(index + 2);
        }
        let element = Elements {
            is_number: is_number(character),
            has_symbol: check_for_symbol_nearby(index),
            right_symbol,
            further_right,
        };
        values.push(element);
    }

    let mut n: usize = 0;

    while n < DATA.len() {
        if (values[n].is_number && values[n + 1].is_number && values[n + 2].is_number)
            && (values[n].has_symbol || values[n].right_symbol || values[n].further_right)
        {
            let result: i32 = (DATA[n..n + 1].parse::<i32>().unwrap() * 100)
                + (DATA[n + 1..n + 2].parse::<i32>().unwrap() * 10)
                + (DATA[n + 2..n + 3].parse::<i32>().unwrap());
            sum.push(result);
            n += 3;
            continue;
        }
        if (values[n].is_number && values[n + 1].is_number)
            && (values[n].has_symbol || values[n].right_symbol)
        {
            let result: i32 = (DATA[n..n + 1].parse::<i32>().unwrap() * 10)
                + (DATA[n + 1..n + 2].parse::<i32>().unwrap());
            sum.push(result);
            n += 2;
            continue;
        }
        if (values[n].is_number) && (values[n].has_symbol) {
            let result: i32 = DATA[n..n + 1].parse::<i32>().unwrap();
            sum.push(result);
            n += 1;
            continue;
        }
        n += 1;
    }

    let sum: i32 = sum.iter().sum();
    println!("The answer is {:?}", sum);
}

fn check_for_symbol_nearby(idx: usize) -> bool {
    let mut result: bool = false;

    if idx > OFFSET {
        let a = &DATA[idx - OFFSET..idx - (OFFSET - 1)]
            .parse::<char>()
            .unwrap();
        let a = is_symbol(*a);
        if a {
            result = true;
        }
    }

    if idx > (OFFSET - 1) {
        let b = &DATA[idx - (OFFSET - 1)..idx - (OFFSET - 2)]
            .parse::<char>()
            .unwrap();
        let b = is_symbol(*b);
        if b {
            result = true;
        }
    }

    if idx > (OFFSET - 2) {
        let c = &DATA[idx - (OFFSET - 2)..idx - (OFFSET - 3)]
            .parse::<char>()
            .unwrap();
        let c = is_symbol(*c);
        if c {
            result = true;
        }
    }

    if idx > 1 {
        let d = &DATA[idx - 1..idx].parse::<char>().unwrap();
        let d = is_symbol(*d);
        if d {
            result = true;
        }
    }

    if idx < &DATA.len() - 2 {
        let e = &DATA[idx + 1..idx + 2].parse::<char>().unwrap();
        let e = is_symbol(*e);
        if e {
            result = true;
        }
    }

    if idx < &DATA.len() - OFFSET {
        let f = &DATA[idx + (OFFSET - 2)..idx + (OFFSET - 1)]
            .parse::<char>()
            .unwrap();
        let f = is_symbol(*f);
        if f {
            result = true;
        }
    }

    if idx < &DATA.len() - (OFFSET - 1) {
        let g = &DATA[idx + (OFFSET - 1)..(idx + OFFSET)]
            .parse::<char>()
            .unwrap();
        let g = is_symbol(*g);
        if g {
            result = true;
        }
    }

    if idx < &DATA.len() - (OFFSET + 1) {
        let h = &DATA[idx + OFFSET..idx + (OFFSET + 1)]
            .parse::<char>()
            .unwrap();
        let h = is_symbol(*h);
        if h {
            result = true;
        }
    }
    result
}

fn is_number(sym: char) -> bool {
    if sym.is_ascii_digit() {
        return true;
    }
    false
}

fn is_symbol(sym: char) -> bool {
    let symbols = "#&*%/-=+$@".to_string();
    for each in symbols.chars() {
        if sym == each {
            return true;
        }
    }
    false
}
