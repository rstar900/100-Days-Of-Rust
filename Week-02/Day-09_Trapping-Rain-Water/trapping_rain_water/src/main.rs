use std::{cmp, io::{self, Write}};

fn trapped_water(heights: &Vec<u8>) -> u32 {

    // No water can be trapped when bars < 3
    if heights.len() < 3 {
        return 0;
    }

    let n =  heights.len();
    let mut sum: u32 = 0; // The output 
    let mut max_left: Vec<u8> = vec![0; n]; // The highest bar on left of each element  
    let mut max_right: Vec<u8> = vec![0; n]; // The highest bar on right of each element

    // Populate max_left
    for i in 1..n {
        max_left[i] = cmp::max(heights[i-1], max_left[i-1]);
    }

    // Populate max_right
    for i in (0..n-1).rev() {
       max_right[i] = cmp::max(heights[i+1], max_right[i+1]); 
    }


    // Calculate the sum of trapped water
    for i in 0..n {
        // Both the max should be greater than the ith bar's height
        sum += if (max_left[i] > heights[i]) && (max_right[i] > heights[i]) {
            cmp::min(max_left[i] as u32, max_right[i] as u32) - heights[i] as u32
        } else {
            0
        };
    }

    sum
}

fn main() {
    let mut input = String::new();
    let mut heights: Vec<u8> = vec![];
    print!("Enter n: ");
    io::stdout().flush().expect("Error flushing STDOUT!");
    io::stdin().read_line(&mut input).expect("Error reading from STDIN!");
    let n = input.trim().parse::<usize>().expect("Error parsing input to usize!");
    input.clear();
    println!("Start entering the heights below:");
    for i in 0..n {
        io::stdin().read_line(&mut input).expect("Error reading from STDIN!");
        heights.push(input.trim().parse::<u8>().expect("Error parsing input to u8!"));
        input.clear();
    }
    println!("Rain water trapped: {} units", trapped_water(&heights));
}

// Tests are below
#[cfg(test)]

#[test]
fn test_trapped_water() {
    assert_eq!(trapped_water(&vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    assert_eq!(trapped_water(&vec![4,2,0,3,2,5]), 9);
    assert_eq!(trapped_water(&vec![]), 0);
    assert_eq!(trapped_water(&vec![1]), 0);
    assert_eq!(trapped_water(&vec![1, 2]), 0);
}