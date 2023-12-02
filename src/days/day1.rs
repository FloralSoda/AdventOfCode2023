fn get_value<I>(line: I, flip: bool) -> u32
where
    I: Iterator<Item = char>,
{
    let names = if flip {
        [
            "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
        ]
    } else {
        [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
    };

    let mut word = String::from("");
    for char in line {
        if char.is_ascii_digit() {
            return char.to_digit(10).expect("Expected digit");
        }
        word = word + &char.to_string();
        for (idx, txt) in names.iter().enumerate() {
            if word.ends_with(txt) {
                return idx as u32;
            }
        }
    }
    println!("Line without numbers! {}", word);
    0
}
pub fn day1() {
    //Combine first digit and last digit to form two-digit number

    let input = include_str!("../../resources/day1.txt");
    let lines = input.split('\n');

    let total = lines.fold(0u32, |acc, line: &str| {
        if line.is_empty() {
            return acc;
        }

        let first_digit = get_value(line.chars(), false);
        let last_digit = get_value(line.chars().rev(), true);

        acc + (first_digit * 10) + last_digit
    });

    println!("{}", total);
}
