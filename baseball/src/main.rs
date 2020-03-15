// Submitted by: Pooja Khanal, Anushka Bhattacharjee

// Prepared for : Project II, CS524, Spring 2020
// Submitted on : 03/15/2020

//Program Description:
// Main tasks covered by this program are
// 1. This program calculates the player statistics, batting avaerage, slugging and base percentage.
// 2. Checks if inconsisent data are present in any player line.
// 3. Calculates overall batting average for players without any error in their input data
// 4. This is the starting point of the program.

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

mod player;
use player::{get_player, Player};

mod report;
use report::{handle_print_to_file, print_reports};

extern crate colour;
use colour::{dark_magenta, dark_cyan};

// Entry point of the program // main function
fn main() {
    dark_magenta!("Prepared by : Pooja Khanal/Anushka Bhattacharjee\n");
    dark_cyan!(
        "Welcome to the player statistics calculator test program. 
I am going to read players from an input data file.  
You will tell me the name of your input file.  
I will store all of the players in a list,compute each player's averages
and then write the resulting team report to your output file.\n"
    );
    println!("Provide the name of your input file and press ENTER");
    match get_all_players() {
        Err(error) => println!("{}", error),
        Ok(results) => {
            print_reports(&results, None);
            handle_print_to_file(&results);
        }
    }
    println!("End of Program! GoodBye!");
}

// Read from the file and get each player
fn get_all_players() -> Result<Vec<Result<Player, String>>, String> {
    let mut fname = String::new();
    let mut results: Vec<Result<Player, String>> = Vec::new();
    match io::stdin().read_line(&mut fname) {
        Ok(_) => {
            fname = fname.trim_end().to_string();
            match read_lines(&fname) {
                Ok(lines) => {
                    let mut line_count: i32 = 0;
                    for line_res in lines {
                        line_count += 1;
                        if let Ok(line) = line_res {
                            let result =
                                get_player(&line.split(" ").collect::<Vec<&str>>(), line_count);
                            results.push(result);
                        }
                    }
                    Ok(results)
                }
                Err(_) => Err(format!("Could not read file {}", &fname)),
            }
        }
        Err(_) => Err("Error reading the file name".to_string()),
    }
}

// Read line by line from the input file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
