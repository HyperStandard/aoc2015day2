#![feature(string_remove_matches)]
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = get_arguments(&args);

    let mut content = String::new();

    if path != None {
        let file_path = path.unwrap();
        println!("Using supplied file: {}", &file_path);
        content = fs::read_to_string(file_path)
            .expect("Error, unable to read file. Deleting your vacation pictures.");
        //len = content.chars().count()
    } else {
        print!("Enter \"map\": ");
        //len = std::io::stdin().read_line(&mut content).unwrap();
    }
    let mut height: u64 = 0;
    let mut width: u64 = 0;
    let mut length: u64 = 0;
    let mut total_paper: u64 = 0;
    //TODO remember how linebreaks work on every OS
    for (i, dimensions) in content.split("\r\n").enumerate() {
        println!("lindy: [{}] dimensions = {}", i, dimensions);
        for (j, mut char) in dimensions.split("x").enumerate() {
            //char.remove_matches((|x| x.is_whitespace()));
            String::remove_matches(mut char, (|x| x.is_whitespace()));
            dbg!(char);
            match j {
                0 => height = char.parse().unwrap(),
                1 => width = char.parse().unwrap(),
                2 => length = char.parse().unwrap(),
                _ => println!("what did you DO"),
            }
            total_paper += 2 * length * width + 2 * width * height + 2 * height * length;
            total_paper += smallest_of(vec![height, width, length]);
        }
    }

    println!("The elves need {} feet of paper", total_paper);
}

fn get_arguments(args: &Vec<String>) -> Option<String> {
    let pos = args.iter().position(|x| x == "-f" || x == "--file");
    if pos != None {
        let path = Some(args[pos.unwrap() + 1].clone());
        return path;
    } else {
        return None;
    }
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
    return baby;
}
