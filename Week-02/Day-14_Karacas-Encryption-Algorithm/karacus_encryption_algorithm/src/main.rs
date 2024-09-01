use std::{io::{self, Write}, collections::HashMap};

fn encrypt(input: &str) -> String {
    // Define HashMap for mapping vowels to their replacements
    let vowels_map = HashMap::from([
        ('a', '0'), ('e', '1'), ('i', '2'), ('o', '2'), ('u', '3')
    ]);

    let mut output = String::new();

    // Reverse the string and create a new one with replaced vowels which is then the output
    for ch in input.chars().rev() {
        if let Some(x) = vowels_map.get(&ch) {
            output.push(*x);

        } else {
            output.push(ch);
        }
    }

    // Add "aca" to the end of the output
    output.push_str("aca");

    output
}

fn main() {
    let mut input = String::new();
    print!("Enter the input string: ");
    io::stdout().flush().expect("Error flushing STDOUT!");
    io::stdin().read_line(&mut input).expect("Error reading STDIN!");
    println!("The output string: {}", encrypt(&input.trim()));
}

// Tests below
#[cfg(test)]

#[test]
fn test_encrypt() {
    assert_eq!(encrypt("banana"), "0n0n0baca");
    assert_eq!(encrypt("karaca"), "0c0r0kaca");
    assert_eq!(encrypt("burak"), "k0r3baca");
    assert_eq!(encrypt("alpaca"), "0c0pl0aca");
}