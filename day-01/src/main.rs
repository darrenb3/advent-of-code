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
    match word_num{
        "one"=> number = "1",
        "two"=> number = "2",
        "three"=> number = "3",
        "four"=> number = "4",
        "five"=> number = "5",
        "six"=> number = "6",
        "seven"=> number = "7",
        "eight"=> number = "8",
        "nine"=> number = "9",
        _ => number = word_num
    };
    return number
}
fn word_to_num_reverse(word_num: &str) -> &str {
    let mut number = "";
    match word_num{
        "eno"=> number = "1",
        "owt"=> number = "2",
        "eerht"=> number = "3",
        "ruof"=> number = "4",
        "evif"=> number = "5",
        "xis"=> number = "6",
        "neves"=> number = "7",
        "thgie"=> number = "8",
        "enin"=> number = "9",
        _ => number = word_num
    };
    return number
}

fn part1() {
    let re = Regex::new(r"[1-9]").unwrap();
    let input = read_lines("input.txt");
    let mut line_totals = Vec::new();
    for i in input {
        let reverse: String = i.chars().rev().collect();
        let j = re.find(&*i).unwrap();
        let k = j.as_str();
        let l = re.find(&*reverse).unwrap();
        let m = l.as_str();
        let n = [k, m].join("");
        line_totals.push(n.parse::<i32>().unwrap())
    }
    let total: i32 = line_totals.iter().sum();
    println!("The answer to part one is: {}", total)
}

fn part2() {
    let re = Regex::new(r"([1-9]|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine))").unwrap();
    let re_reverse = Regex::new(r"([1-9]|(enin)|(thgie)|(neves)|(xis)|(evif)|(ruof)|(eerht)|(owt)|(eno))").unwrap();
    let input = read_lines("input.txt");
    let mut line_totals = Vec::new();
    for i in input {
        let reverse: String = i.chars().rev().collect();
        let j = re.find(&*i).unwrap();
        let k = j.as_str();
        let k1 = word_to_num(k);
        let l = re_reverse.find(&*reverse).unwrap();
        let m = l.as_str();
        let m1 = word_to_num_reverse(m);
        let n = [k1, m1].join("");
        line_totals.push(n.parse::<i32>().unwrap())
    }
    let total: i32 = line_totals.iter().sum();
    println!("The answer to part two is: {}", total)
}

fn main() {
    part1();
    part2();
}
