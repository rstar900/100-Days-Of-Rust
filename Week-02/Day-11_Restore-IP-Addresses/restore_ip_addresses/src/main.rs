use std::cmp::min;
use std::io::{self, Write};

// Helper function
fn restore_ip_addresses_helper(substring: &str, state: &mut Vec<u8>, octet: u8,  output: &mut Vec<String>) {

    use std::fmt::Write; // Imported here as it causes conflict with the Write trait imported for main()

    // Base case
    if octet == 5{
        if substring.is_empty() {
            // Append the IP address to the output vector with dot notation
            let mut valid_ip_addr = String::new();
            write!(valid_ip_addr, "{}.{}.{}.{}", state[0], state[1], state[2], state[3]).expect("Error writing formatted string!");
            output.push(valid_ip_addr);
        }

    } else if substring.chars().nth(0).expect("Error accessing 0th index of the substring!") == '0' {
        state.push(0);
        restore_ip_addresses_helper(&substring[1..], state, octet + 1, output);
        state.pop();

    } else {
        // We can go forward with a max of 3 digit numbers, depending on a few conditions
        for i in 0..min(3, substring.len()) {
            // Only if enough space is left for other octets to form
            if substring[i+1..].len() >= 4 - octet as usize {
                // If digits chosen are valid u8 (0 to 255)
                if let Ok(ip_part) =  substring[0..=i].parse(){
                    state.push(ip_part);
                    restore_ip_addresses_helper(&substring[i+1..], state, octet + 1, output);
                    state.pop();
                } 
            }
        }
    }

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
    let mut input = String::new();
    print!("Enter the numeric string: ");
    io::stdout().flush().expect("Error flushing STDOUT!");
    io::stdin().read_line(&mut input).expect("Error reading from STDIN!");
    println!("The possible valid IP addresses are: {:?}", restore_ip_addresses(&input.trim()));
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