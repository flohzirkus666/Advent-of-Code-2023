use regex::Regex;

// Advent of code 2023 day 1
// https://adventofcode.com/2023/day/1

// Split entire string line-by-line
fn split_string(input: String) -> Vec<String> {
    let lines: Vec<String> = input.split("\n").map(|v| v.to_string()).collect();
    lines
}

// Get first and last number of each row
fn get_numbers(input: String) -> u32 {
    // prepare regular expression
    let re = Regex::new(r"[0-9]").unwrap();
    let numbers: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();
    // concatenate first and last element of vector, using pattern matching to skip empty vectors
    let result = match &*numbers {
        [] => "0".to_string(),
        [..] => format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap()),
    };
    result.parse().unwrap()
}

fn main() {
    // our calibration document
    let document = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
        .to_string();

    let lines = split_string(document);
    let mut sum: u32 = 0;
    for line in lines {
        sum += get_numbers(line);
    }
    println!("Result is: {sum}");
}
