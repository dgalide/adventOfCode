use std::fs;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    let splitted = contents.split("\n");

    let mut total = 0;

    splitted.for_each(|line| {
        let mut first_digit: Option<char> = None;
        let mut last_digit: Option<char> = None;

        if line.is_empty() {
            return;
        }
        line.chars().for_each(|c| {
            if c.is_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(c);
                }
                last_digit = Some(c);
            }
        });

        if last_digit.is_none() {
            return;
        }

        let combined = format!("{}{}", first_digit.unwrap(), last_digit.unwrap());

        let as_number = combined.parse::<u32>();

        if as_number.is_err() {
            return;
        }

        let as_number = as_number.unwrap();

        total += as_number;
    });

    println!("Total: {}", total);
}
