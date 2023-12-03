
fn main() {
    let mut sum: u32 = 0;
    let mut data: Vec<_> = include_str!("data.txt")
            .split("\n")
            .collect();

    data.pop();

    for each in data {
    let  v: Vec<&str> = each.matches(char::is_numeric).collect();
    let number = v[0].to_string() + v[v.len()-1];
    let number = number.parse::<u32>().unwrap();
    sum += number;
    }

    println!("{}", sum);
}
// /Users/richard/rust_/day1/src/
