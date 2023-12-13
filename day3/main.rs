//day3x
//This needs some recursion, goddamn hack
const DATA: &str = 
    "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n";
const OFFSET: usize = 12;

#[derive(Debug)]
struct Elements {
    is_number: bool,
    has_symbol: bool,
    right_symbol: bool,
    further_right: bool,
}

fn main() {
    /*
    "467..114.."
    "...*......"
    "..35..633."
    "......#..."
    "617*......"
    ".....+.58."
    "..592....."
    "......755."
    "...$.*...."
    ".664.598.."
    */
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
            right_symbol: right_symbol,
            further_right: further_right,
        };
        values.push(element);
    }

    let mut idx_start: usize = 0; 
    let mut idx_end: usize = 0;
    let mut n: usize = 0;
    while n < DATA.len() {
        if (values[n].is_number && values[n+1].is_number && values[n+2].is_number) && (values[n].has_symbol || values[n].right_symbol || values[n].further_right){
            idx_start = n;
            idx_end = n + 3; 
            let result: i32 = (DATA[n..n+1].parse::<i32>().unwrap() * 100) + (DATA[n+1..n+2].parse::<i32>().unwrap() * 10) + (DATA[n+2..n+3].parse::<i32>().unwrap());
            sum.push(result);
            n += 3;
            continue;
        }      
        if (values[n].is_number && values[n+1].is_number) && (values[n].has_symbol || values[n].right_symbol){
            idx_start = n;
            idx_end = n + 2; 
            let result: i32 = (DATA[n..n+1].parse::<i32>().unwrap() * 10) + (DATA[n+1..n+2].parse::<i32>().unwrap());
            sum.push(result);
            n += 2; 
            continue;
        }
        if (values[n].is_number) && (values[n].has_symbol){
            idx_start = n;
            idx_end = n + 1; 
            let result: i32 = (DATA[n..n+1].parse::<i32>().unwrap());
            sum.push(result);
            n += 1; 
            continue;
        }
         n += 1;
        }
        
    let sum: i32 = sum.iter().sum(); 
    println!("{:?}", sum);
  
    
    //if values[n+1].is_number && (values[n].has_symbol || values[n].right_symbol)

    //end of the main function
    }

        //TODO
        //println!("{:?}", each);
        //if number
        //if n2 is number && (n1 has symbol or n2) idx_start and idx_end
        //if n3 is number && (n1, n2, n3 or has symbol) idx_start and idx_end
        //push values to sum
    
    //TODO
    //flatten the list of numbers in sum and get the anwser


//TODO replace numbers hard code or offsets
fn check_for_symbol_nearby(idx: usize) -> bool {
    let mut result: bool = false;
    if idx > 12 {
        let a = &DATA[idx - OFFSET..idx - (OFFSET - 1)]
            .parse::<char>()
            .unwrap();
        let a = is_symbol(*a);
        if a {
            result = true;
        }
    }

    if idx > 11 {
        let b = &DATA[idx - (OFFSET - 1)..idx - (OFFSET - 2)]
            .parse::<char>()
            .unwrap();
        let b = is_symbol(*b);
        if b {
            result = true;
        }
    }
    if idx > 10 {
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
    if idx < &DATA.len() - 12 {
        let f = &DATA[idx + (OFFSET - 2)..idx + (OFFSET - 1)]
            .parse::<char>()
            .unwrap();
        let f = is_symbol(*f);
        if f {
            result = true;
        }
    }
    if idx < &DATA.len() - 11 {
        let g = &DATA[idx + (OFFSET - 1)..(idx + OFFSET)]
            .parse::<char>()
            .unwrap();
        let g = is_symbol(*g);
        if g {
            result = true;
        }
    }
    if idx < &DATA.len() - 13 {
        let h = &DATA[idx + OFFSET..idx + (OFFSET + 1)]
            .parse::<char>()
            .unwrap();
        let h = is_symbol(*h);
        if h {
            result = true;
        }
    }
    return result;
}

fn is_number(sym: char) -> bool {
    if sym.is_digit(10) {
        return true;
    }
    return false;
}
fn is_symbol(sym: char) -> bool {
    let symbols = "#&*%/-=+$@".to_string();
    for each in symbols.chars() {
        if sym == each {
            return true;
        }
    }
    return false;
}
