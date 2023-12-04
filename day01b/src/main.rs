use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut sum = 0;

    for line in contents.split("\n") {
        println!("{}", "---");
        println!("{}", line);
        let processed_line = process_line(line);
        println!("{}", "-");
        println!("{}", processed_line);
        sum += processed_line;
    }

    println!("{}", "===");
    println!("{}", sum);
    Ok(())
}

fn process_line(line: &str) -> i32 {
    let mut first_num: Option<char> = None;
    let mut last_num: Option<char> = None;

    for char in replace_from_start(line).chars() {
        if char.is_numeric() {
            if first_num == None {
                first_num = Some(char);
            }
        }
    }

    for char in replace_from_end(line).chars() {
        if char.is_numeric() {
            last_num = Some(char);
        }
    }
    return format!("{}{}", first_num.expect("Missing first num"), last_num.expect("Missing last num")).parse::<i32>().unwrap();
}


fn replace_from_start(line: &str) -> String {
    let line_len = line.len();
    for i in 0..line_len+1 {
        let window = &line[0..i];
        let replacement = replace_str_nums(window);

        // Rely on the fact that successful replace leads to a shorter string
        if replacement.len() < window.len() {
            let replaced_line = format!("{}{}", replacement, &line[i..line_len]);
            println!("{}", replaced_line);
            return replaced_line
        }
    }
    return line.to_string()
}

fn replace_from_end(line: &str) -> String {
    let line_len = line.len();
    for i in 0..line_len+1 {
        let window = &line[line_len-i..line_len];
        let replacement = replace_str_nums(window);

        // Rely on the fact that successful replace leads to a shorter string
        if replacement.len() < window.len() {
            let replaced_line = format!("{}{}", &line[0..line_len-i], replacement);
            println!("{}", replaced_line);
            return replaced_line
        }
    }
    return line.to_string()
}

fn replace_str_nums(line: &str) -> String {
    return line
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
}