use std::io::{self, Write};

// Recursion concept taken from https://www.baeldung.com/cs/get-number-of-binary-search-trees-n-distinct-elements

fn unique_bst(n: u32) -> u32 {
    // Base case
    if n == 0 {
        return 1;
    }

    let mut total_bst = 0; // Total unique bst for the respective subtree

    // Place the respective i on the subtree's root node
    for i in 1..=n {
        // Left subtree contains elements [1..i-1], i.e. i-1-1+1 = i-1 possibilities
        // Right subtree contains elements [i+1..n], i.e. n-(i+1)+1 = n-i possibilities
        total_bst += unique_bst(i - 1) * unique_bst(n - i);
    }

    total_bst
}

fn main() {
    let mut n = String::new();
    print!("Enter n: ");
    io::stdout().flush().expect("Error flushing STDOUT!");
    io::stdin().read_line(&mut n).expect("Error reading from STDIN!");
    println!("Unique BSTs: {}", unique_bst(n.trim().parse().expect("Error parsing n as u32")));

}

#[cfg(test)]

#[test]
fn test_unique_bst() {
    assert_eq!(unique_bst(0), 1);
    assert_eq!(unique_bst(1), 1);
    assert_eq!(unique_bst(2), 2);
    assert_eq!(unique_bst(3), 5);
    assert_eq!(unique_bst(10), 16796);
}