use std::io;

// Returns true or false depending on whether s1 and s2 are valid anagrams or not
fn valid_anagram(s1: &str, s2: &str) -> bool {
    
    // Check for length, if equal only then proceed further, else return false
    if s1.len() == s2.len() {
        // Store the frequencies of characters for both strings
        let mut freq_s1: Vec<u8> = vec![0; 26];
        let mut freq_s2: Vec<u8> = vec![0; 26];

        // We use the length of s1 in this case
        let string_len = s1.len();

        for i in 0..string_len {
            freq_s1[s1.chars().nth(i).expect("Error: Index out of bounds!") as usize - 'a' as usize] += 1;
            freq_s2[s2.chars().nth(i).expect("Error: Index out of bounds!") as usize - 'a' as usize] += 1;
        }

        // Check the frequencies of characters in both strings, if mismatch, then return false, else continue till end
        for i in 0..26 {
            if freq_s1[i] != freq_s2[i] {
                return false;
            }
        }
        return true;
    }
    
    false
}

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the two strings one in each line below:");
    io::stdin().read_line(&mut input1).expect("Error: Cannot read from STDIN!");
    io::stdin().read_line(&mut input2).expect("Error: Cannot read from STDIN!");

    let result = valid_anagram(input1.trim(), input2.trim());

    if result {
        println!("They are valid anagrams :)");
    } else {
        println!("They are not anagrams :(");
    }
}

// Tests start below
#[cfg(test)]

#[test]
fn test_valid_anagram() {
    assert_eq!(true, valid_anagram("anagram", "nagaram"));
    assert_eq!(false, valid_anagram("rat", "car"));
}
