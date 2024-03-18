trait Check {
    fn check(&self, rule: &Draw) -> bool;
}

#[derive(Default)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Check for Draw {
    fn check(&self, rule: &Draw) -> bool {
        self.red <= rule.red && self.green <= rule.green && self.blue <= rule.blue
    }
}

impl Draw {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Check for Game {
    fn check(&self, rule: &Draw) -> bool {
        self.draws.iter().all(|draw| draw.check(rule))
    }
}

impl Game {
    fn min_cubes(&self) -> Draw {
        // Maximum number of shown cubes for each colour within the game equals minimum required
        Draw {
            red: self.draws.iter().map(|draw| draw.red).max().unwrap(),
            green: self.draws.iter().map(|draw| draw.green).max().unwrap(),
            blue: self.draws.iter().map(|draw| draw.blue).max().unwrap(),
        }
    }
}

fn get_colour_count(input: &str, colour: &str) -> u32 {
    // Return string for requested colour if it exists in reveal
    let result = input.split(", ").filter(|str| str.contains(colour)).last();
    if let Some(colour) = result {
        // Strip off text, leaving numeric result
        colour
            .chars()
            .filter(|char| char.is_numeric())
            .collect::<String>()
            .parse::<u32>()
            .unwrap()
    } else {
        0
    }
}

fn parse_line(input: &str) -> Game {
    let (game, draws) = input.split_once(": ").unwrap();
    Game {
        // Get game ID from first slice
        id: game.replace("Game ", "").parse::<u32>().unwrap(),
        // Get draws from second slice
        draws: draws
            .split(";")
            .map(|draw| {
                // Determine number of red, blue and green cubes per reveal
                Draw {
                    red: get_colour_count(draw, "red"),
                    green: get_colour_count(draw, "green"),
                    blue: get_colour_count(draw, "blue"),
                }
            })
            .collect(),
    }
}

pub fn day_2(input: String) -> (u32, u32) {
    // Parse input
    let games: Vec<Game> = input.lines().map(|line| parse_line(line)).collect();

    // Determine which games would have been possible if the bag contained only
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    let rule = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };
    let ids = games
        .iter()
        .filter_map(|game| match game.check(&rule) {
            true => Some(game.id),
            false => None,
        })
        .sum();

    let powers = games.iter().map(|game| game.min_cubes().power()).sum();

    (ids, powers)
}
