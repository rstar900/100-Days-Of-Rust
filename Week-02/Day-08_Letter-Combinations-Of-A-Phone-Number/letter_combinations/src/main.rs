use std::io::{self, Write};
use std::collections::HashMap;

// Helper function with recursion
fn letter_combinations_helper(input: &str, state: &mut String, output: &mut Vec<String>, key_map: &HashMap<char, Vec<char>>) {

    // Base case
    if input.is_empty() {
        // Append the state so far to the output vector
        output.push((*state).clone());
        return;

    } else {
        // Iterate the values of the key which is the first character in input substring
        let letters = key_map.get(&input.chars().nth(0).expect("No character at 0th position")).expect("Undefined key");
        for letter in letters {
            // Append the character to the existing state
            state.push(*letter);
            // start the process for the next character of input substring recursively
            letter_combinations_helper(&input[1..], state, output, key_map);
            // remove the character from the state as state is a common reference
            state.pop();
        }
    }   
}


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

    letter_combinations_helper(input, &mut String::new(), &mut output, &key_map);

    output
}

fn main() {
    let mut input = String::new();
    print!("Enter the input digits: ");
    io::stdout().flush().expect("Error flushing the STDOUT!");
    io::stdin().read_line(&mut input).expect("Error reading input from STDIN!");
    let output = letter_combinations(&input.trim());
    println!("The possible letter combinations are: {:?}", output);
}

// Tests are below
#[cfg(test)]

#[test]
fn test_letter_combinations() {
    assert_eq!(letter_combinations("23"), vec!["ad","ae","af","bd","be","bf","cd","ce","cf"]);
    assert_eq!(letter_combinations(""), vec![""]);
    assert_eq!(letter_combinations("2"), vec!["a","b","c"]);
}
