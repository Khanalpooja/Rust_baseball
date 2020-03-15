use player::Player;
use std::io::BufWriter;
use std::io;
use std::fs::File;
use std::io::Write;

pub fn print_reports(results: &Vec<Result<Player, String>>, out: Option<String>) {
    let mut out_stream = get_output_stream(out);
    let (players, errors): (Vec<_>, Vec<_>)  = results
                                                .into_iter()
                                                .partition(|x| x.is_ok());
    let mut players: Vec<_> = players.into_iter().map(|x| x.as_ref().unwrap()).collect();
    players.sort();
    let errors: Vec<_> = errors.into_iter().map(|x| x.as_ref().unwrap_err()).collect();
    print_player_report(players, &mut out_stream);
    print_error_report(errors, &mut out_stream);
}


pub fn handle_print_to_file(results: &Vec<Result<Player, String>>) {
    if let Some(x) = get_output_file_name() {
        print_reports(results, Some(x));
    }
}

// print the player report
fn print_player_report(players: Vec<&Player>, out: &mut BufWriter<Box<dyn Write>>) {
	let mut sum : f32 = 0.0;

	out.write(format!("\nBASEBALL TEAM REPORT --- {} PLAYERS FOUND IN FILE\n", players.len()).as_bytes()).unwrap();

	for player in &players {
		sum = sum + (player.get_batting_average() as f32);
	}

	let overal_avg = sum / (players.len() as f32);
	out.write(format!("OVERALL BATTING AVERAGE is {}\n", overal_avg).as_bytes()).unwrap();

	out.write(format!("\n      {} {} {} {} {}", "PLAYER NAME", ":", "AVERAGE", "SLUGGING", "ONBASE%").as_bytes()).unwrap();

	out.write(b"\n -------------------------------------------------------------------").unwrap();
	for player in &players {
		out.write(format!("\n {} {} {} {} {}", player.get_name(), ":", player.get_batting_average(), player.get_slugging_percentage(), player.get_on_base_percentage()).as_bytes()).unwrap();
	}

}

fn print_error_report(errors: Vec<&String>, out: &mut BufWriter<Box<dyn Write>>) {
	out.write(format!("\n\n----- {} ERROR LINES FOUND IN INPUT DATA ----\n\n", errors.len()).as_bytes()).unwrap();
	for err in &errors {
		out.write(format!("{} \n", err).as_bytes()).unwrap();
	}
}


fn get_output_file_name() -> Option<String> {
    loop {
        let (mut option, mut outfile) = (String::new(), String::new());
        println!("\nDo you want to write your output to a file? Type Y/N");
        match io::stdin().read_line(&mut option){
            Ok(_) => {
                option = option.trim_end().to_string();
                match option.as_str() {
                    "Y" | "y" => {
                        println!("Enter the output file name");
                        match io::stdin().read_line(&mut outfile){
                            Ok(_) => {
                                outfile = outfile.trim_end().to_string();
                                return Some(outfile)
                            },
                            Err(_) => {
                                println!("Error reading output file name");
                            }
                        };
                    },
                    "N" => {
                        return None
                    },
                    _ => {println!("Invalid option!");}
                };
            },
            Err(_) => {
                println!("Error: Could not read your option");
            }
        }
    }
}

fn get_output_stream(out: Option<String>) -> BufWriter<Box<dyn Write>> {
    let out_writer : BufWriter<Box<dyn Write>> = BufWriter::new(
        if let Some(filename) = out { 
            Box::new(File::create(filename).unwrap())
        } else {
            Box::new(io::stdout())
        });
    out_writer
}