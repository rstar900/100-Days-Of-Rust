use std::io::{self, Write};

fn landscape_type(list: &Vec<i32>) -> String {
    let mut output = String::new();

    // Base case: should have atleast 3 elements
    if list.len() < 3 {
        output.insert_str(0, "neither");

    } else {

        let mut peaks: u32 = 0;
        let mut troughs: u32 = 0;
        
        for i in 1..(list.len() - 1) {
            if (list[i - 1] < list [i]) && (list[i + 1] < list [i]) {
                // We found a local peak
                peaks += 1; 

            } else {

                if (list[i - 1] > list [i]) && (list[i + 1] > list [i]) {
                    // We found a local trough
                    troughs += 1;
                }

            }

        }

        // Evaluate based on number of peaks and troughs in the list 
        // Last 2 conditions for checking presence of unwanted edge peaks/troughs

        if (peaks == 1) && (troughs == 0) && (list[0] <= list[1]) && (list[list.len() - 1] <= list[list.len() - 2]) {
            // The list is a peak
            output.insert_str(0, "mountain");

        } else if (peaks == 0) && (troughs == 1) && (list[0] >= list[1]) && (list[list.len() - 1] >= list[list.len() - 2]) {
            // The list is a valley
            output.insert_str(0, "valley");

        } else {
            output.insert_str(0, "neither");
        }
    }

    output
}

fn main() {
    let mut input = String::new();
    let mut input_list: Vec<i32> = vec![];

    print!("Enter n: ");
    io::stdout().flush().expect("Error flushing STDOUT!");
    io::stdin().read_line(&mut input).expect("Error reading from STDIN!");

    let n = input.trim().parse::<usize>().expect("Error parsing n as usize!");
    input.clear();
    for i in 0..n {
        io::stdin().read_line(&mut input).expect("Error reading from STDIN!");
        input_list.push(input.trim().parse::<i32>().expect("Error parsing input as i32!"));
        input.clear();
    }

    println!("The landscape is: {}", landscape_type(&input_list));
}

// Tests below
#[cfg(test)]

#[test]
fn test_landscspe_type_mountains() {
    // Test for mountains
    assert_eq!(landscape_type(&vec![1, 3, 5, 4, 3, 2]), "mountain");
    assert_eq!(landscape_type(&vec![-1, 0, -1]), "mountain");
    assert_eq!(landscape_type(&vec![-1, -1, 0, -1, -1]), "mountain");
    assert_eq!(landscape_type(&vec![3, 4, 5, 4, 3]), "mountain");
}

#[test]
fn test_landscspe_type_valleys() {
    // Test for valleys
    assert_eq!(landscape_type(&vec![10, 9, 8, 7, 2, 3, 4, 5]), "valley");
    assert_eq!(landscape_type(&vec![350, 100, 200, 400, 700]), "valley");
    assert_eq!(landscape_type(&vec![9, 7, 3, 1, 2, 4]), "valley");
    assert_eq!(landscape_type(&vec![9, 8, 9]), "valley");
}

#[test]
fn test_landscspe_type_neither() {
    // Test for neither types
    assert_eq!(landscape_type(&vec![]), "neither");
    assert_eq!(landscape_type(&vec![1]), "neither");
    assert_eq!(landscape_type(&vec![1, 2]), "neither");
    assert_eq!(landscape_type(&vec![1, 2, 3, 2, 4, 1]), "neither");
    assert_eq!(landscape_type(&vec![5, 4, 3, 2, 1]), "neither");
    assert_eq!(landscape_type(&vec![0, -1, -1, 0, -1, -1]), "neither");
    assert_eq!(landscape_type(&vec![9, 8, 9, 8]), "neither");
}