#[derive(Clone, Copy, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

struct Number {
    pos: Coordinate,
    length: usize,
    val: u32,
}

#[allow(unused)]
struct Gear {
    pos: Coordinate,
    ratio: u32,
}

pub fn day_3(input: String) -> (u32, u32) {
    let max_coordinate = Coordinate {
        x: input.lines().nth(0).iter().len(),
        y: input.lines().count(),
    };

    let symbols: Vec<Coordinate> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| match !c.is_numeric() && c != '.' {
                    true => Some(Coordinate { x, y }),
                    false => None,
                })
        })
        .collect();

    let numbers: Vec<Number> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let chars = line.chars().collect::<Vec<char>>();
                match c.is_numeric() && (x == 0 || !chars[x - 1].is_numeric()) {
                    true => {
                        let mut i = x;
                        let mut val: u32 = chars[i].to_digit(10).unwrap();
                        while i + 1 < chars.len() && chars[i + 1].is_numeric() {
                            i += 1;
                            val = val * 10 + chars[i].to_digit(10).unwrap();
                        }

                        Some(Number {
                            pos: Coordinate { x, y },
                            length: i - x + 1,
                            val,
                        })
                    }
                    false => None,
                }
            })
        })
        .collect();

    let gears: Vec<Gear> = symbols
        .clone()
        .into_iter()
        .filter_map(|sym| {
            let adjacent = find_adjacent_numbers(&vec![sym], &numbers, &max_coordinate);
            match adjacent.len() == 2 {
                true => Some(Gear {
                    pos: sym,
                    ratio: adjacent.iter().product(),
                }),
                false => None,
            }
        })
        .collect();

    // Get sum of all numbers
    let sum = find_adjacent_numbers(&symbols, &numbers, &max_coordinate)
        .iter()
        .sum();

    // Get sum of all gear ratios
    let ratios = gears.iter().map(|g| g.ratio).sum();

    (sum, ratios)
}

/// Check for adjacent numbers and return their start indices
fn find_adjacent_numbers(
    symbols: &Vec<Coordinate>,
    numbers: &Vec<Number>,
    max: &Coordinate,
) -> Vec<u32> {
    numbers
        .iter()
        .filter_map(|num| {
            match get_box_coordinates(&num.pos, num.length, max)
                .iter()
                .any(|coordinate| symbols.contains(coordinate))
            {
                true => Some(num.val),
                false => None,
            }
        })
        .collect()
}

fn get_box_coordinates(pos: &Coordinate, length: usize, max: &Coordinate) -> Vec<Coordinate> {
    let x_start = match pos.x > 0 {
        true => pos.x - 1,
        false => pos.x,
    };
    let x_end = match pos.x + length < max.x - 1 {
        true => pos.x + length + 1,
        false => pos.x + length,
    };
    let y_start = match pos.y > 0 {
        true => pos.y - 1,
        false => pos.y,
    };
    let y_end = match pos.y < max.y - 1 {
        true => pos.y + 1,
        false => pos.y,
    };

    let mut coordinates = vec![];

    for x in (x_start)..=(x_end) {
        for y in (y_start)..=(y_end) {
            coordinates.push(Coordinate { x, y })
        }
    }
    coordinates
}
