use std::io::{self, Write};

fn can_fit(list: Vec<usize>, n: usize) -> bool {

    let mut sum: usize = 0;
    
    for i in list {
        // Check if any element is greater than 10, in this case simply return false as it will not fit any bag
        if i > 10 {
            return false;
        } else {
            // Otherwise the sum should be less than equal to n*10 as that is the total capacity of all bags combined
            sum += i;
        }
    } 

    sum <= n * 10
}


fn main() {
    let mut input = String::new();
    let mut list: Vec<usize> = vec![]; 
    print!("Enter the number of elements you want in the array: ");
    io::stdout().flush().expect("Error flushing STDOOUT!");
    io::stdin().read_line(&mut input).expect("Error reading from STDIN!");
    let num_elements = input.trim().parse::<usize>().expect("Error parsing as usize!");
    input.clear();
    for _i in 0..num_elements {
        io::stdin().read_line(&mut input).expect("Error reading from STDIN!");
        list.push(input.trim().parse::<usize>().expect("Error parsing as usize!"));
        input.clear();
    }
    print!("Enter the number of 10kg bags: ");
    io::stdout().flush().expect("Error flushing STDOOUT!");
    io::stdin().read_line(&mut input).expect("Error reading from STDIN!");
    println!("Can the elements fit the bags? The answer is {}", if can_fit(list, input.trim().parse::<usize>().expect("Error parsing as usize!")) {
        "Yes!"
    } else {
        "No!"
    });
}

// Tests below

#[cfg(test)]

#[test]
fn test_can_fit() {
    assert_eq!(can_fit(vec![2, 1, 2, 5, 4, 3, 6, 1, 1, 9, 3, 2], 4), true);
    assert_eq!(can_fit(vec![2, 7, 1, 3, 3, 4, 7, 4, 1, 8, 2], 4), false);
}