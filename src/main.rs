#![feature(string_remove_matches)]
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = get_arguments(&args);

    let mut content: String = String::new();

    if path.is_some() {
        let file_path = path.unwrap();
        println!("Using supplied file: {}", &file_path);
        content = fs::read_to_string(file_path)
            .expect("Error, unable to read file. Deleting your vacation pictures.");
    } else {
        //todo!("write this code for stdin")
    }

    let mut total_paper: u64 = 0;
    for dimensions in content.split('\n') {
        total_paper += line_amount(String::from(dimensions));
    }

    println!("The elves need {total_paper} feet of paper");
}

fn get_arguments(args: &Vec<String>) -> Option<String> {
    let pos = args.iter().position(|x| x == "-f" || x == "--file");
    if pos.is_some() {
        Some(args[pos.unwrap() + 1].clone())
    } else {
        None
    }
}

fn line_amount(input: String) -> u64 {
    let mut lwh: Vec<u64> = vec![0, 0, 0];
    let mut dims: String = input;

    //for some reason there's like a spare linebreak somwhere in here so i may as well put this here
    //to keep the demons at bay in future
    dims.remove_matches(|x: char| x.is_whitespace());

    //split "nnnxnnnxnnn" into a vector of 3 u32s", ignore everything else just as a side effect
    for (index, dim) in dims.split('x').enumerate() {
        if index < 3 {
            lwh[index] = dim.parse().unwrap_or(0)
        }
    }
    let sides = (lwh[0] * lwh[1], lwh[1] * lwh[2], lwh[2] * lwh[0]);

    //this is probably the formula for area of all sides + the spare bits
    (2 * sides.0) + 2 * sides.1 + 2 * sides.2 + smallest_of(vec![sides.0, sides.1, sides.2])
}

fn smallest_of(input: Vec<u64>) -> u64 {
    let mut baby: u64 = 0;
    for number in input {
        if baby == 0 {
            baby = number
        }
        if number < baby {
            baby = number;
        }
    }
    baby
}
