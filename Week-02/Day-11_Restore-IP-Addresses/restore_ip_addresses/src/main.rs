use std::io;

// Helper function
fn restore_ip_addresses_helper(substring: &str, state: &mut Vec<u8>, octet: u8,  output: &Vec<String>) {

    // Base case
    if substring.len() == 0 {
        todo!("Probably append the overall string from a state string into a vector");

    } else if substring.chars().nth(0).expect("Error accessing 0th index of the substring!") == '0' {
        todo!("No need to combine with other numbers as 0 is separate in IP address");
    }

    todo!("Implement logic based on the substring passed to it as input");
}

fn restore_ip_addresses(input: &str) -> Vec<String> {

    let mut output: Vec<String> = vec![];

    // Cannot generate IP Addresses if input < 4 or greater than 12
    if input.len() < 4 || input.len() > 12 {
        return output; // Empty vector
    }

    restore_ip_addresses_helper(input, &mut vec![], 1, &mut output);

    output

} 

fn main() {
    println!("Hello, world!");
}


// Tests below
#[cfg(test)]

#[test]
fn test_restore_ip_addresses() {
    assert_eq!(restore_ip_addresses("25525511135"), vec!["255.255.11.135","255.255.111.35"]);
    assert_eq!(restore_ip_addresses("0000"), vec!["0.0.0.0"]);
    assert_eq!(restore_ip_addresses("1111"), vec!["1.1.1.1"]);
    assert_eq!(restore_ip_addresses("010010"), vec!["0.10.0.10","0.100.1.0"]);
    assert_eq!(restore_ip_addresses("101023"), vec!["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]);
}