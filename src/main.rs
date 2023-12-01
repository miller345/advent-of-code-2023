use regex::Regex;
use std::env;
use std::fs;

// cargo run -- 1 eg
// cargo run -- 1

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let day_n = &args[1];
    let example = args.iter().any(|e| e == "eg");

    println!("day {}", day_n);
    println!("example? {}", example);

    if day_n == "1" {
        if example == true {
            let example_input =
                fs::read_to_string("./day1.example.txt").expect("Unable to read file");
            day01(example_input);
        } else {
            let input = fs::read_to_string("./day1.txt").expect("Unable to read file");
            day01(input);
        }
    }
}

fn day01(input: String) {
    println!("input:\n{}", input);
    let lines = input.lines();
    let re = Regex::new(r"\D").unwrap();
    println!("lines:");
    let mut numbers: Vec<i32> = Vec::new();
    for line in lines {
        let digits = re.replace_all(line, "");
        let chars_to_join = vec![
            digits.chars().nth(0).unwrap(),
            digits.chars().last().unwrap(),
        ];
        let result: String = chars_to_join.into_iter().collect();
        let num: i32 = result.parse().unwrap();
        numbers.push(num);
        println!("{}", num)
    }
    let sum: i32 = numbers.iter().sum();
    println!("sum: {}", sum)
}
