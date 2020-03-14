use std::io;
use std::fs::File;
// use std::io::{self, BufRead};
use std::io::BufRead;
use std::path::Path;

fn main(){
    println!("Input the file name");
    let mut fname = String::new();
    
    match io::stdin().read_line(&mut fname){
        Ok(_) => {
            println!("{}",&fname);
            fname = fname.trim_end().to_string();
            // fname = fname.to_string();
            if let Ok(lines) = read_lines(fname) {
                // Consumes the iterator, returns an (Optional) String
                for line in lines {
                    if let Ok(ip) = line {
                        println!("{}", ip);
                    }
                }
            }
        },
        Err(_) => {
            println!("Error reading the filename")
        }
        
    }




    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

}