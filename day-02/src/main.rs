use std::fs::read_to_string;

use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> { // Reads in all lines as a mega vector
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn check_game(lines: Vec<String>) {
    let re = Regex::new(r"(Game \d+)|(\d+ green)|(\d+ red)|(\d+ blue)").unwrap();
    for i in lines {
        let j: Vec<_> = re.find_iter(&*i).map(|m| m.as_str()).collect(); // Will create a vector with will all the numbers from the line in it
        let game = j[0];
        let mut game_possible; // If this is 1 then the game is possible.
        for k in j{
            let re = Regex::new(r"\d|(\w+)").unwrap();
            let color_amount: Vec<_> = re.find_iter(&*k).map(|m| m.as_str()).collect();
            println!("{:?}", color_amount);
            match color_amount[1] {
                "green" => if  color_amount[0].parse::<i32>().unwrap() > 13 {
                    game_possible = 0
                } else {
                    game_possible = 1
                }

            }
        }

        // println!("{:?}",j)
        //for k in j {

        //}
    }
}



fn main() {
    let lines = read_lines("input.txt");
    parse_lines(lines);
    //println!("{:?}", lines)
}
