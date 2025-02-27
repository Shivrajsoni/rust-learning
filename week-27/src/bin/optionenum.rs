// Custom Result enum to handle multiple positions
enum Option1<T> {
    Some(T),
    None,
}

fn main() {
    let s = String::from("Shirajdakhdaadkhjadyeuryqruyqeruyreqiqryiqwyiuerquiyeuyreqiurqeyuiqeqiyreiv");
    
    // Find all occurrences of 'i'
    let positions = find_all_chars(&s, 'i');
    println!("Found 'i' at positions: {:?}", positions);
    
    // Find first and last occurrence
    let first = find_first_char(&s, 'i');
    let last = find_last_char(&s, 'i');
    
    match (first, last) {
        (Option1::Some(f), Option1::Some(l)) => {
            println!("First 'i' at: {}, Last 'i' at: {}", f, l);
        }
        _ => println!("Character not found"),
    }
    
    // Count occurrences
    let count = count_char(&s, 'i');
    println!("Total occurrences of 'i': {}", count);
}

// Find all occurrences of a character
fn find_all_chars(s: &str, target: char) -> Vec<usize> {
    s.chars()
        .enumerate()
        .filter(|(_, c)| c.to_ascii_lowercase() == target.to_ascii_lowercase())
        .map(|(i, _)| i)
        .collect()
}

// Find first occurrence
fn find_first_char(s: &str, target: char) -> Option1<usize> {
    for (i, c) in s.chars().enumerate() {
        if c.to_ascii_lowercase() == target.to_ascii_lowercase() {
            return Option1::Some(i);
        }
    }
    Option1::None
}

// Find last occurrence
fn find_last_char(s: &str, target: char) -> Option1<usize> {
    let mut last_index = Option1::None;
    for (i, c) in s.chars().enumerate() {
        if c.to_ascii_lowercase() == target.to_ascii_lowercase() {
            last_index = Option1::Some(i);
        }
    }
    last_index
}

// Count occurrences
fn count_char(s: &str, target: char) -> usize {
    s.chars()
        .filter(|c| c.to_ascii_lowercase() == target.to_ascii_lowercase())
        .count()
}

