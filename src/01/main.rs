use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;

lazy_static! {
    static ref L_REGEX: Regex =
        Regex::new("one|two|three|four|five|six|seven|eight|nine|\\d").unwrap();
    static ref R_REGEX: Regex =
        Regex::new("enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|\\d").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should be able to read a file");

    println!(
        "PART 1: {}",
        contents
            .lines()
            .map(get_calibration_value_part_1)
            .sum::<u32>()
    );
    println!(
        "PART 2: {}",
        contents
            .lines()
            .map(get_calibration_value_part_2)
            .sum::<u32>()
    );
}

fn get_calibration_value_part_1(str: &str) -> u32 {
    let l_digit = str.find(is_decimal_digit).unwrap();
    let r_digit = str.rfind(is_decimal_digit).unwrap();

    return 10 * str.chars().nth(l_digit).unwrap().to_digit(10).unwrap()
        + str.chars().nth(r_digit).unwrap().to_digit(10).unwrap();
}

fn is_decimal_digit(c: char) -> bool {
    return c.is_digit(10);
}

fn get_calibration_value_part_2(str: &str) -> u32 {
    let reversed_str = reverse(str);

    let l_match = L_REGEX.find(str).map(|x| x.as_str()).unwrap();
    let r_match = R_REGEX
        .find(reversed_str.as_str())
        .map(|x| x.as_str())
        .unwrap();

    return 10 * get_number_from_string(l_match)
        + get_number_from_string(reverse(r_match).as_str());
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn get_number_from_string(str: &str) -> u32 {
    return match str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => str.parse::<u32>().unwrap(),
    };
}
