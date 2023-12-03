//ugh...
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

    let mut least_red = 0;
    let mut least_blue = 0;
    let mut least_green = 0;

        for each in v {
        let idx_red = each.find("red");
        let idx_blue = each.find("blue");
        let idx_green = each.find("green");

        let red: usize = match idx_red {
            Some(x) => each[x - 3..x].trim().parse().unwrap(),
            None => 0,
        };
        if red > least_red {
            least_red = red;
        }
        let green: usize = match idx_green {
            Some(x) => each[x - 3..x].trim().parse().unwrap(),
            None => 0,
        };
        if green > least_green {
            least_green = green;
        }
        let blue: usize = match idx_blue {
            Some(x) => each[x - 3..x].trim().parse().unwrap(),
            None => 0,
        };
        if blue > least_blue{
            least_blue = blue;
        }
    }
    sum += least_green * least_blue * least_red;

    return sum;
}

