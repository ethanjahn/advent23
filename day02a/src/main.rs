use std::fs::File;
use std::io::prelude::*;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut sum = 0;

    for line in contents.split("\n") {
        let games_possible = get_games(line).iter().all(|&game| is_possible(game));
        if games_possible {
            sum += get_id(line);
        }
    }
    println!("{}", sum);
    Ok(())
}

fn get_id(line: &str) -> i32 {
    // Returns the ID of the given line
    return line.split(":").collect::<Vec<&str>>()[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap()
}

fn get_games(line: &str) -> Vec<&str> {
    line.splitn(2, ':').nth(1).unwrap_or("").split(';').map(str::trim).collect()
}

fn is_possible(game: &str) -> bool {
    for pull in game.split(",").map(str::trim) {
        let (num, color) = pull.split_once(" ").unwrap();
        if num.parse::<i32>().unwrap() > max(color) {
            return false
        }
    }
    return true
}

fn max(color: &str) -> i32 {
    match color {
        "green" => MAX_GREEN,
        "blue" => MAX_BLUE,
        "red" => MAX_RED,
        _ => 0,
    }
}