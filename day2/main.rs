//quick and ugly...

fn main() {
    let mut answer: usize = 0;
    let mut data: Vec<_> = include_str!("data.txt").split("\n").collect();

    data.pop();

    for each in data {
        let line: Vec<_> = each.split(&[';', ':']).collect();
        answer += check(&line);
    }

    println!("sum is {}", answer);
}

fn check(v: &Vec<&str>) -> usize {
    let mut sum = 0;
    let game_id: Vec<_> = v[0].split(" ").collect();
    let game_id: usize = game_id[1].trim().parse().unwrap();
    for each in v {
        let idx_red = each.find("red");
        let idx_blue = each.find("blue");
        let idx_green = each.find("green");

        let red: usize = match idx_red {
            Some(x) => each[x - 3..x].trim().parse().unwrap(),
            None => 0,
        };
        if red > 12 {
            return 0;
        }
        let green: usize = match idx_green {
            Some(x) => each[x - 3..x].trim().parse().unwrap(),
            None => 0,
        };
        if green > 13 {
            return 0;
        }
        let blue: usize = match idx_blue {
            Some(x) => each[x - 3..x].trim().parse().unwrap(),
            None => 0,
        };
        if blue > 14 {
            return 0;
        }
    }
    sum += game_id;

    return sum;
}


