//day3 - to finish

//got into a mess here!
//find symbol
//check 8 around is a number
//if a number scan left and right for other numbers
//collect and sum numbers
//probably need a 'seen' list to avoid double counting numbers

fn main() {
    let data: &str = include_str!("test.txt"); 
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
    print!("{}", data);

    for i in data.chars()  {
    let a = is_number(i); 
    let b = is_newline(i); 
    let c = is_dot(i); 
    let d = is_symbol(i);
        println!("The char is {}, the type: is number {}, is newline {}, is dot {}, is symbol {}", i, a, b, c, d);
    }
}


//is a type
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

fn is_dot(sym:char) -> bool  {
    if matches!(sym, '.')  {
        return true
    }
    return false
}

//possibly no wholly reliable...
fn is_newline(sym: char) -> bool {
    if sym.is_ascii_control() {
        return true;
    }
    return false;
}

