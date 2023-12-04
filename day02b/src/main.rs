use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut sum = 0;

    for line in contents.split("\n") {
        let power = game_power(get_all_pulls(line));
        sum += power
    }
    println!("{}", sum);
    Ok(())
}

fn get_all_pulls(line: &str) -> Vec<String> {
    let replaced = line.splitn(2, ':').nth(1).unwrap_or("").replace(";", ",");
    let output: Vec<String> = replaced
        .split(',')
        .map(str::trim)
        .map(String::from)
        .collect();
    return output;
}

fn game_power(pulls: Vec<String>) -> i32 {
    let mut min_green: i32 = 0;
    let mut min_red: i32 = 0;
    let mut min_blue: i32 = 0;

    for pull in pulls {
        let (num, color) = pull.split_once(" ").unwrap();
        let i_num = num.parse::<i32>().unwrap();

        if color == "green" && i_num > min_green {
            min_green = i_num;
        }

        if color == "red" && i_num > min_red {
            min_red = i_num;
        }

        if color == "blue" && i_num > min_blue {
            min_blue = i_num;
        }
    }
    return min_green * min_red * min_blue;
}
