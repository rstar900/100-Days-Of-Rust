use std::io;

fn sock_pairs(s: &str) -> u8 {
    let mut count: u8 = 0;

    if s.len() == 0 || s.len() == 1 {
        return count; // count is 0 in the beginning    

    } else {
        // Since s.len() is not known before runtime, we cannot use simple arrays, hence vector
        let mut paired_indices: Vec<bool> = vec![false; s.len()]; // Keep track of indices whether they are paired or not
        for i in 0..(s.len() - 1)  {
            for j in (i + 1)..s.len() {
                if !paired_indices[i] { // make sure it is not already paired, and only then continue with the further operations
                    if s.chars().nth(i).unwrap() == s.chars().nth(j).unwrap() {
                        paired_indices[j] = true; // no need to set for ith, as it is not going to be traversed again
                        count += 1;
                        break;
                    }
                } 
            }
        }
    }

    count
}

fn main() {
    let mut input: String = String::new();
    println!("Enter the sequence of letters below:");
    io::stdin().read_line(&mut input).expect("Error reading from STDIN!");
    println!("Pairs of socks: {}", sock_pairs(input.trim())); // remove leading and trailing whitespaces from input while passing it
}

// Tests below
#[cfg(test)]

#[test]
fn test_sock_pairs() {
    assert_eq!(sock_pairs("AA"), 1);
    assert_eq!(sock_pairs("ABABC"), 2);
    assert_eq!(sock_pairs("CABBACCC"), 4);
    assert_eq!(sock_pairs("CABBACC"), 3);
    assert_eq!(sock_pairs(""), 0);
    assert_eq!(sock_pairs("A"), 0);
}
