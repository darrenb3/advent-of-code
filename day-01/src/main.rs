use std::fs::read_to_string;

use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn word_to_num(word_num: &str) -> &str {
    let mut number = "";
    match word_num {
        "one" => number = "1",
        "two" => number = "2",
        "three" => number = "3",
        "four" => number = "4",
        "five" => number = "5",
        "six" => number = "6",
        "seven" => number = "7",
        "eight" => number = "8",
        "nine" => number = "9",
        _ => number = word_num
    };
    return number;
}


fn part1() {
    let re = Regex::new(r"\d").unwrap();
    let input = read_lines("input.txt");
    let mut line_totals = Vec::new();
    for i in input {
        let j: Vec<_> = re.find_iter(&*i).map(|m| m.as_str()).collect();
        let k = [j[0], j[j.len() - 1]].join("");
        line_totals.push(k.parse::<i32>().unwrap())
    }
    let total: i32 = line_totals.iter().sum();
    println!("The answer to part one is: {}", total)
}

fn part2() { //This has a bug involving the one input with shared letters (oneight) Not sure how to fix it.
    let re = Regex::new(r"(\d|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine))").unwrap();
    let input = read_lines("input.txt");
    let mut line_totals = Vec::new();
    for i in input {
        let j: Vec<_> = re.find_iter(&*i).map(|m| m.as_str()).collect();
        let k = [word_to_num(j[0]), word_to_num(j[j.len() - 1])].join("");
        line_totals.push(k.parse::<i32>().unwrap())
    }
    let total: i32 = line_totals.iter().sum();
    println!("The answer to part two is: {}", total)
}

fn main() {
    part1();
    part2();
}
