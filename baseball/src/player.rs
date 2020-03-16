// Submitted by: Pooja Khanal, Anushka Bhattacharjee

// Prepared for : Project II, CS524, Spring 2020
// Submitted on : 03/15/2020

//Program Description:
// Main tasks covered by this program are
// 1. This program calculates the player statistics, batting avaerage, slugging and base percentage.
// 2. Checks if inconsisent data are present in any player line.
// 3. Calculates overall batting average for players without any error in their input data
// 4. main.rs is the starting point for the program

//Player Structure , also sort the player by lastname and then firstname
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Player {
    last_name: String,
    first_name: String,
    plate_appearances: i32,
    at_bats: i32,
    singles: i32,
    doubles: i32,
    triples: i32,
    home_runs: i32,
    walks: i32,
    hit_by_pitch: i32,
}

impl Player {
    // function to get the batting average of a player
    pub fn get_batting_average(&self) -> f32 {
        let p = self;
        (p.singles as f32 + p.doubles as f32 + p.triples as f32 + p.home_runs as f32)
            / p.at_bats as f32
    }

    //function to get the slugging of a player
    pub fn get_slugging_percentage(&self) -> f32 {
        let p = self;
        (p.singles as f32
            + 2.0 * p.doubles as f32
            + 3.0 * p.triples as f32
            + 4.0 * p.home_runs as f32)
            / p.at_bats as f32
    }

    //function to get Onbase percentage
    pub fn get_on_base_percentage(&self) -> f32 {
        let p = self;
        (p.singles as f32
            + p.doubles as f32
            + p.triples as f32
            + p.hit_by_pitch as f32
            + p.walks as f32)
            / p.plate_appearances as f32
    }

    //function to get the full name of the player
    pub fn get_name(&self) -> String {
        self.last_name.clone() + ", " + &self.first_name
    }
}

//get the details of the player and create an instance of each Player Structure
//Either the Player is returned or the error in the player line is returned
pub fn get_player(s: &Vec<&str>, line_count: i32) -> Result<Player, String> {
    let err = check_errors(s, line_count);
    match err {
        Ok(_) => {
            let player = Player {
                first_name: s[0].to_string(),
                last_name: s[1].to_string(),
                plate_appearances: s[2].parse::<i32>().unwrap(),
                at_bats: s[3].parse::<i32>().unwrap(),
                singles: s[4].parse::<i32>().unwrap(),
                doubles: s[5].parse::<i32>().unwrap(),
                triples: s[6].parse::<i32>().unwrap(),
                home_runs: s[7].parse::<i32>().unwrap(),
                walks: s[8].parse::<i32>().unwrap(),
                hit_by_pitch: s[9].parse::<i32>().unwrap(),
            };
            Ok(player)
        }
        Err(e) => Err(e),
    }
}

//check for errors in the line
// Error is returned if present
fn check_errors(s: &Vec<&str>, line_count: i32) -> Result<(), String> {
    if s.len() < 10 {
        return Err(format!(
            "line ({}) : Line contains not enough data",
            line_count
        ));
    }

    for i in 2..10 {
        let test = s[i].parse::<i32>();
        match test {
            Err(_) => {
                return Err(format!(
                    "line ({}) : Line contains invalid numeric data",
                    line_count
                ))
            }
            Ok(_) => (), // do nothing
        }
    }
    Ok(())
}
