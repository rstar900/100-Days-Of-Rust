use std::io;
use std::collections::HashMap;


fn letter_combinations(input: &str) -> Vec<String> {
    let mut  output: Vec<String> = vec![]; 

    let mut key_map: HashMap<char, Vec<char>> = HashMap::new();
    key_map.insert('2', vec!['a', 'b', 'c']);
    key_map.insert('3', vec!['d', 'e', 'f']);
    key_map.insert('4', vec!['g', 'h', 'i']);
    key_map.insert('5', vec!['j', 'k', 'l']);
    key_map.insert('6', vec!['m', 'n', 'o']);
    key_map.insert('7', vec!['p', 'q', 'r', 's']);
    key_map.insert('8', vec!['t', 'u', 'v']);
    key_map.insert('9', vec!['w', 'x', 'y', 'z']);

    println!("{key_map:?}");

    // TODO: complete the function definition

    output
}

fn main() {
    // Just testing what the created map looks like
    letter_combinations("123");
}

// Tests are below
#[cfg(test)]

#[test]
fn test_letter_combinations() {
    // TODO testing logic
}
