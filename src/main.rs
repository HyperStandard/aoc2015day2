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
    let mut total_ribbon: u64 = 0;
    for dimensions in content.split('\n') {
        let nums = line_amount(String::from(dimensions));
        total_paper += nums.0;
        total_ribbon += nums.1;
    }

    //we merely need to output this information
    println!("The elves need {total_paper} feet of paper, and {total_ribbon} feet of ribbon.");
}

fn get_arguments(args: &Vec<String>) -> Option<String> {
    let pos = args.iter().position(|x| x == "-f" || x == "--file");
    if pos.is_some() {
        Some(args[pos.unwrap() + 1].clone())
    } else {
        None
    }
}

fn line_amount(input: String) -> (u64, u64) {
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

    //These are actually half sized so I don't have to do a bunch of 2* to all the dimensions which are perimeters
    //should be fine just need to *2 for the output
    let ribbons = (lwh[0] + lwh[1], lwh[1] + lwh[2], lwh[2] + lwh[0]);

    let smallest_perimeter = smallest_of(vec![ribbons.0, ribbons.1, ribbons.2]);

    //the 3 side areas out of 6 total sides (hence the *2 later)
    let papers = (lwh[0] * lwh[1], lwh[1] * lwh[2], lwh[2] * lwh[0]);

    //this is the formula for area of all sides + the spare bits
    (
        (2 * papers.0)
            + 2 * papers.1
            + 2 * papers.2
            + smallest_of(vec![papers.0, papers.1, papers.2]),
        2 * smallest_perimeter + (lwh[0] * lwh[1] * lwh[2]),
    )
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
