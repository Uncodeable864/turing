use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str,
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

    // Print regexes
    let basic_print_regex = Regex::new(r"(?<=PRINT )\d*").unwrap();
    let second_print_param = Regex::new(r"(?<=PRINT (0|1) ).*").unwrap();
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
        // Print to command line
        else if basic_print_regex.is_match(&line).unwrap() {
            // Basic BIT print
            if line.contains("BIT") {
                let position_string = basic_print_regex.find(&line).unwrap().unwrap().as_str();
                let position = position_string.parse::<usize>().unwrap();
                println!("{}", memory_line[position] as usize);
            }
            // Basic BIN (binary) print
            else if line.contains("BIN") {
                let first_position_string =
                    basic_print_regex.find(&line).unwrap().unwrap().as_str();
                let first_position = first_position_string.parse::<usize>().unwrap();
                let second_position_string =
                    second_print_param.find(&line).unwrap().unwrap().as_str();
                let second_position = second_position_string.parse::<usize>().unwrap();
                for i in (first_position..(second_position + 1)).rev() {
                    print!("{}", memory_line[i] as usize);
                }
                print!("\n");
            } else if line.contains("NUM") {
                let first_position_string =
                    basic_print_regex.find(&line).unwrap().unwrap().as_str();
                let first_position = first_position_string.parse::<usize>().unwrap();
                let second_position_string =
                    second_print_param.find(&line).unwrap().unwrap().as_str();
                let second_position = second_position_string.parse::<usize>().unwrap();
                let segment = &memory_line[first_position..(second_position + 1)];
                println!("{}", binary_vector_to_int(segment.to_vec()));
            }

            continue;
        }
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

fn binary_vector_to_int(vector: Vec<bool>) -> usize {
    let mut result: usize = 0;
    for i in 0..(vector.len()) {
        result += usize::pow(2, i as u32) * bool_to_int(vector[i]);
    }
    return result;
}
fn bool_to_int(bool: bool) -> usize {
    if bool {
        1
    } else {
        0
    }
}
