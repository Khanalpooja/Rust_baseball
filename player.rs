//Player Structure 

struct Player {
   firstName, lastName:string,
   plateAppearances, atBats, singles, doubles, triples, homeRuns, walks, hitByPitch:int
}

enum Result<T, E> {
   Ok(T),
   Err(E),
}

// function to get the batting average of a player
fn getBattingAverage(p Player) -> f64{
    (p.singles+p.doubles+p.triples+p.homeRuns) / p.atBats 
}

//function to get he slugging of a player
fn getSluggingPercentage(p Player) -> f64 {
	(p.singles+2*p.doubles+3*p.triples+4*p.homeRuns) / p.atBats 
}

//function to get Onbase percentage
fn getOnBasePercentage(p Player) -> f64 {
	(p.singles+p.doubles+p.triples+p.hitByPitch+p.walks) / p.plateAppearances 
}

// ByLastName Comparator to sort players by last name
// type ByLastName []Player

fn getPlayer(s []string, lineCount int) -> Player{
	err := checkErrors(s, lineCount)
    match err {
        Ok(ok) => {
            let player =  Player;
            player = Player{
                firstName = s[0],
                lastName=   s[1],
            }
            player.plateAppearances = s[2].parse::<f64>().unwrap();
            player.atBats= s[3].parse::<f64>().unwrap();
            player.singles= s[4].parse::<f64>().unwrap();
            player.doubles = s[5].parse::<f64>().unwrap();
            player.triples= s[6].parse::<f64>().unwrap();
            player.homeRuns= s[7].parse::<f64>().unwrap();
            player.walks= s[8].parse::<f64>().unwrap();
            player.hitByPitch= s[9].parse::<f64>().unwrap();
            player
        }
        Err(e) => Err(e)
    }
	
}

//check for errors in the line
fn checkErrors(s []string, lineCount int) -> Result <(), Err> {
	if s.len() < 10 {
		Err("line {} Line contains not enough data", lineCount)
	}
    
	for i := 2; i < 10; i++ {
        let test = s[i].parse::<f64>();
        match test {
            Err(_) => return Err("Line ({}) : line contains invalid numeric data", linecount),
            Ok(_) => () // do nothing
        }
	}
	Result
}

