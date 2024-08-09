use std::io::{self, Write};

fn progress_days(list: &Vec<u8>) -> u8 {
    
    // No progress days for empty or list with single element
    if list.is_empty() || list.len() == 1 {
        return 0;
    }

    let mut progress_days_count: u8 = 0; 
    let mut prev_miles: u8 = list[0]; // keep track of previous Saturday's miles

    for (index, current_miles) in list.into_iter().enumerate() {
        // Ignore first element
        if index != 0 {
            if *current_miles > prev_miles {
                progress_days_count += 1;
            }
        }
        prev_miles = *current_miles; // update the previous miles to be the current one for the next iteration 
    }

    progress_days_count
}

fn main() {
    let mut list: Vec<u8> = vec![]; // list to capture inputs
    let mut n: String = String::new(); // number of saturdays

    print!("Enter the number of Saturdays: ");
    io::stdout().flush().expect("Error flushing STDOUT!");
    io::stdin().read_line(&mut n).expect("Error reading input from STDIN!");

    for i in 1..=n.trim().parse::<u8>().expect("Invalid value entered for number of Saturdays!") {
        let mut buf: String = String::new(); // for capturing line (declared here as read_line() appends to current string otherwise)
        io::stdin().read_line(&mut buf).expect("Error reading input from STDIN!");
        list.push(buf.trim().parse::<u8>().expect("Invalid value entered for miles!"));
    }

    println!("The progress days: {}", progress_days(&list));
}

// Tests begin below
#[cfg(test)]

#[test]
fn test_progress_days() {
    assert_eq!(progress_days(&vec![]), 0);
    assert_eq!(progress_days(&vec![1]), 0);
    assert_eq!(progress_days(&vec![3, 4, 1, 2]), 2);
    assert_eq!(progress_days(&vec![10, 11, 12, 9, 10]), 3); 
    assert_eq!(progress_days(&vec![6, 5, 4, 3, 2, 9]), 1);
    assert_eq!(progress_days(&vec![9, 9]), 0);
}
