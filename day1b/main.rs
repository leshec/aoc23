fn main() {
    let mut answer: u32 = 0;
   let mut data: Vec<_> = include_str!("/Users/richard/rust_/day1b/src/data.txt")
            .split("\n")
            .collect();

    data.pop();

    for word in data {
        let word = word.replace("one", "o1ne");
        let word = word.replace("two", "t2wo");
        let word = word.replace("three", "t3hree");
        let word = word.replace("four", "f4our");
        let word = word.replace("five", "f5ive");
        let word = word.replace("six", "s6ix");
        let word = word.replace("seven", "s7even");
        let word = word.replace("eight", "e8ight");
        let word = word.replace("nine", "n9ine");
        let  v: Vec<&str> = word.matches(char::is_numeric).collect();
        let number = v[0].to_string() + v[v.len()-1];
        let number = number.parse::<u32>().unwrap();
        answer += number;
    }

    println!("{}", answer);
}


