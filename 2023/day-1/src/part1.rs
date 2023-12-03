use std::fmt::Error;
use std::fs::read_to_string;


fn process(input: &str) -> Result<String,Error> {
    //Read input
    let lines = input.lines();

    let mut filtered_lines = Vec::new();

    for line in lines {
        let mut filtered_string = String::new();

        for c in line.chars() {
            if c.is_digit(10) {
                filtered_string.push(c)
            };
        }

        filtered_lines.push(filtered_string)
    }

    let mut two_digits = Vec::new();

    for filtered_line in filtered_lines {
        let first_char = filtered_line.chars().next().unwrap();
        let last_char = filtered_line.chars().last().unwrap();

        let combined_string = format!("{}{}", first_char, last_char);
        two_digits.push(combined_string);
    }

    let mut sum: i32 = 0;

    for two_digit in two_digits {
        let number = match two_digit.parse::<i32>() {
            Ok(x) => x,
            Err(_) => continue,
        };

        sum += number;
    }

    Ok(sum.to_string())
}



fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
        1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!("142",main(input)?)
    }
}