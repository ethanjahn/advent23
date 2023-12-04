use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut sum = 0;

    for line in contents.split("\n") {
        sum += process_line(line);
    }

    println!("{}", sum);
    Ok(())
}

fn process_line(line: &str) -> i32 {
    let mut first_num: Option<char> = None;
    let mut last_num: Option<char> = None;

    for char in line.chars() {
        if char.is_numeric() {
            if first_num == None {
                first_num = Some(char);
            }
            last_num = Some(char);
        }
    }
    return format!(
        "{}{}",
        first_num.expect("Missing first num"),
        last_num.expect("Missing last num")
    )
    .parse::<i32>()
    .unwrap();
}
