use std::io;
use std::fs::File;
// use std::io::{self, BufRead};
use std::io::BufRead;
use std::path::Path;

mod player;
use player::{get_player, Player};

mod report;
use report::{print_reports, handle_print_to_file};

fn main(){
    println!("Input the file name"); 
    match get_all_players() {
        Err(error) => println!("{}", error),
        Ok(results) => {
            print_reports(&results, None);
            handle_print_to_file(&results);
        }
    }
}

fn get_all_players() -> Result<Vec<Result<Player,String>>, String> {
    let mut fname = String::new();
    let mut results: Vec<Result<Player, String>> = Vec::new();
    match io::stdin().read_line(&mut fname){
        Ok(_) => {
            fname = fname.trim_end().to_string();
            match read_lines(&fname) {
                Ok(lines) => {
                    // Consumes the iterator, returns an (Optional) String
                    let mut line_count: i32 = 0;
                    for line_res in lines {
                        line_count += 1;
                        if let Ok(line) = line_res {
                            let result = get_player(&line.split(" ").collect::<Vec<&str>>(), line_count);
                            results.push(result);
                        }
                    }
                    Ok(results)
                },
                Err(_) => Err(format!("Could not read file {}", &fname))
            }
        },
        Err(_) => {
            Err("Error reading the file name".to_string())
        }  
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}