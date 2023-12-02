use std::collections::HashMap;

struct Bag {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}
impl Bag {
    pub fn new(red: usize, green: usize, blue: usize) -> Self {
        Self { red, green, blue }
    }
}

pub fn day2() {
    let data = include_str!("../../resources/day2.txt");
    let games = data.split('\n');

    //let bag = Bag::new(14,13,12); //For part 1

    //What is the sum of the IDs of the games that fit in this bag?

    let output = games.fold(0u32, |acc, game| {
        if game.is_empty() {
            return acc;
        }

        let mut components = game.split(':');
        //let game_id = components.next().expect("Expected game").split(' ').last().expect("Expected game id"); //For part 1
        //let id = game_id.parse::<u32>().expect("ID was not a number"); //For part 1

        let game = components.last().expect("Expected line to contain colon");
        let shows = game.split(';');

        //For part 1, move these into the for loop
        let mut counts = HashMap::<String, usize>::new();
        let mut current_count = 0;
        //-==-

        for show in shows {
            if show.is_empty() {
                continue;
            }
            for (idx, word) in show.trim().split(' ').enumerate() {
                if idx % 2 == 0 {
                    //It's a number
                    current_count = word.parse::<usize>().expect("Expected word to be numeric");
                } else {
                    //It's a color
                    let color = word
                        .to_lowercase()
                        .chars()
                        .take_while(|char| char.is_alphanumeric())
                        .collect::<String>();
                    if !counts.contains_key(&color) || current_count > counts[&color] {
                        //Ignore this condition in part 1
                        counts.insert(color, current_count);
                    }
                }
            }
            //For part 1, short circuiting match comparing colour values to
        }

        //acc + id //Add ID for part 1
        acc + counts.values().fold(1u32, |acc, count| acc * *count as u32)
    });

    println!("{}", output);
}
