use std::{env, fs};

fn value(s: &str, digit_sets: &Vec<&str>) -> usize {
    let mut result = 0;
    let mut fst = s.len();
    let mut lst = 0;
    for digits in digit_sets {
        for (v, digit) in digits.split(' ').enumerate() {
            let mut ind = s.find(digit);
            if ind.is_some() && ind.unwrap() <= fst {
                fst = ind.unwrap();
                result = (v + 1) * 10 + result % 10;
            }
            ind = s.rfind(digit);
            if ind.is_some() && ind.unwrap() >= lst {
                lst = ind.unwrap();
                result = (v + 1) + result / 10 * 10;
            }
        }
    }
    return result.into();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Incorrect number of arguments. expected 2 - part of the day (1 or 2) and path to the input file");
        return;
    }
    let digit_sets = if args[1] == "1" {
        vec!["1 2 3 4 5 6 7 8 9"]
    } else {
        vec![
            "1 2 3 4 5 6 7 8 9",
            "one two three four five six seven eight nine",
        ]
    };
    let content = fs::read_to_string(&args[2]).unwrap();
    let values: usize = content.split('\n').map(|s| value(s, &digit_sets)).sum();
    println!("Result: {}", values);
}
