/// Get first and last digit from a line of text
fn get_calibration_val(input: &str) -> u32 {
    let digits: Vec<char> = input.chars().filter(|c| c.is_numeric()).collect();
    let mut num = String::from(*digits.first().unwrap());
    num.push(*digits.last().unwrap());
    num.parse::<u32>().unwrap()
}

/// Replace spelled digits with real ones
fn get_spelled_calibration_val(input: &str) -> u32 {
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    // Get index of each replacement and sort by index to catch overlaps like 'eightwo'
    for (char_index, _) in input.chars().enumerate() {
        if first.is_none() {
            // Store first occurance of digit
            first = get_digit(input, char_index);
        }
        // Store last digit
        if let Some(val) = get_digit(input, char_index) {
            let _ = last.insert(val);
        }
    }
    let res = first.unwrap() * 10 + last.unwrap();
    res
}

/// Get digit for the current position
fn get_digit(input: &str, char_index: usize) -> Option<u32> {
    let replacements = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let char = input.chars().nth(char_index).unwrap();
    let mut digit: Option<u32> = None;

    if char.is_numeric() {
        digit = char.to_digit(10);
    } else {
        for (i_num, str) in replacements.iter().enumerate() {
            let i_str: Vec<usize> = input.match_indices(str).map(|(i, _)| i).collect();

            // Check if indices of string contain current char index
            if i_str.contains(&char_index) {
                digit = Some(i_num as u32 + 1);
            }
        }
    }
    digit
}

pub fn day_1(input: String) -> (u32, u32) {
    let first_task: u32 = input.lines().map(|line| get_calibration_val(line)).sum();

    let second_task: u32 = input
        .lines()
        .map(|line| get_spelled_calibration_val(line))
        .sum();

    // Return results
    (first_task, second_task)
}
