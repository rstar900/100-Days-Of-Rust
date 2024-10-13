use std::io::{self, Write};

// Helper function to calculate the win or lose recursively
// Here the player 1 represents you, and player 2 your friend
// n represents the number of stones left
fn nim_helper(player: u8, n: u8) -> bool {
    // Base Case
    if (n >= 1) && (n <= 3) {
        if player == 1 {
            return true;
        } else {
            return false;
        }
    // Recursive case    
    } else {
        let next_player: u8;
        if player == 1 {
            next_player = 2;
        } else {
            next_player = 1;
        }
        // Need to consider all the possibilities
        nim_helper(next_player, n-1) || nim_helper(next_player, n-2) || nim_helper(next_player, n-3)
    }
}

fn nim(n: u8) -> bool {
    if n == 0 {
        // Invalid input
        return false; 
    } else if (n >= 1) && (n <= 3) {
        // If the total stones are between 1 and 3 inclusive, then you win
        return true; 
    } else {
        nim_helper(1, n)
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter the number of stones: ");
    io::stdout().flush().expect("Error: Couldn't flush STDOUT!");
    io::stdin().read_line(&mut input).expect("Error: Couldn't read from STDIN!");
    let result = nim(input.trim().parse::<u8>().expect("Error: Couldn't parse string into valid u8"));
    if result {
        println!("You Won!");
    } else {
        println!("You Lose!");
    }
}

// Tests below
#[cfg(test)]

#[test]
fn test_nim() {
    assert_eq!(nim(4), false);
    assert_eq!(nim(1), true);
    assert_eq!(nim(2), true);
    assert_eq!(nim(3), true);
}
