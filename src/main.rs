use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use fancy_regex::Regex;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    println!("Program file: {:?}", file_name);

    let lines: Vec<String> = lines_from_file(file_name);

    // Init regular expessions
    let comment_regex: Regex = Regex::new(r"((//)|(#)|(;--)).*").unwrap(); // The regex to check for comments
    let initialization_regex: Regex = Regex::new(r"(?<=INIT )\d*").unwrap();
    let goto_regex: Regex = Regex::new(r"(?<=GOTO )\d*").unwrap();
    let write_current_regex: Regex = Regex::new(r"(?<=WRITE )\d*").unwrap();
    let write_selected_regex: Regex = Regex::new(r"(?<=WRITE (0|1) )\d*").unwrap();
    // To resize the vector see: https://stackoverflow.com/a/54887778
    let mut memory_line = vec![false; 8];
    let mut current_location: usize = 0;

    for line in lines {
        // Check if the line is a comment, if so, ignore it
        if comment_regex.is_match(&line).unwrap() {
            continue;
        }
        //INIT line check, that will setup the main line
        else if initialization_regex.is_match(&line).unwrap() {
            let length_string = initialization_regex.find(&line).unwrap().unwrap().as_str();
            let length = length_string.parse::<usize>().unwrap();
            // println!(
            //     "Setting length of memory to {}, this will reset the memory.",
            //     length,
            // );
            memory_line.clear();
            memory_line.resize(length, false);
            // Panic if the new size and the expected size are not equal. Failure of true modification may cause crashes later.

            if length != memory_line.len() {
                panic!("Memory line resize failed. This is NOT a problem with your code. Expected size was {}, actual resize was {}", length, memory_line.len())
            }
            println!(
                "Memory line sucsecfully reset and changed to length of {}",
                length
            );
            continue;
        }
        // Moving though memory
        else if goto_regex.is_match(&line).unwrap() {
            // TODO: Add line functionality
            let length_string = goto_regex.find(&line).unwrap().unwrap().as_str();
            current_location = length_string.parse::<usize>().unwrap();
            continue;
        }
        // Writing to memory
        else if write_current_regex.is_match(&line).unwrap() {
            // TODO: Add line functionality
            let length_string = write_current_regex.find(&line).unwrap().unwrap().as_str();
            let write_value_int = length_string.parse::<usize>().unwrap();
            let value: bool = write_value_int != 0;
            // If location is out of bounds, panic
            if current_location >= memory_line.len() {
                panic!(
                    "Memory location {} is out of bounds. This IS a problem with your code.",
                    current_location
                );
            }
            // If the location parameter is not provided
            if !(write_selected_regex.is_match(&line).unwrap()) {
                memory_line[current_location] = value;
                continue;
            }

            let location_string = write_selected_regex.find(&line).unwrap().unwrap().as_str();
            let location = location_string.parse::<usize>().unwrap();
            // If location is out of bounds, panic
            if location >= memory_line.len() {
                panic!(
                    "Memory location {} is out of bounds. This IS a problem with your code.",
                    location
                );
            }
            memory_line[location] = value;
            continue;
        }
        // TODO: Print to command line
        println!("Unhandled command: {}", line);
    }

    Ok(())
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
