use std::io::{self, Write};

// Helper function
fn shift_array_right(num: &mut Vec<i32>, pos: usize) {

    // Logic for shifting the array elements by 1 position to the right starting from pos
    for i in (pos..num.len()-1).rev() { // to reverse the loop i.e. from num.len-2 to pos with step -1 (we can do this as we have extra space)
        num[i+1] = num[i];
    }
}


fn merge_sorted_arrays(m: usize, n: usize, num1: &mut Vec<i32>, num2: &Vec<i32>) {
    let mut p1: usize = 0; // To keep track of num1 
    let mut p2: usize = 0; // To keep track of num2
    let mut num1_processed: usize = 0; // to keep track of elements processed in num1 (as the elements are moved in num1)

    // Continue till the last element of num2 is traversed
    while num1_processed <= m && p2 < n   {
        if num1[p1] <= num2[p2] {
            // increase p1 and num1_processed in this case
            p1 += 1; num1_processed += 1;
        } else {
            // Make space in num1 for the element from num2 by shifting right by 1
            shift_array_right(num1, p1);
            // Insert element from num2 into num1's current position
            num1[p1] = num2[p2];
            // Increase both p1 and p2
            p1 += 1; p2 += 1;
        }
    }

    // If num2 is processed, then no need to do anything as it is inserted into num1 which is already sorted
    // Otherwise if num2 remains, then num1 is already sorted and the remaining ones in num2 are greater than the ones in num1
    // So append from the remaining ones (n - p2)
    if p2 < n {
        let mut k: usize = (m + n) - (n - p2); // total - remaining to get the starting index for appending
        while k < (m + n) {
            num1[k] = num2[p2];
            k += 1; p2 += 1;
        }
    }

}

fn main() {

    let mut nums1: Vec<i32>  = vec![];
    let mut nums2: Vec<i32>  = vec![];
    let mut input = String::new();

    print!("Enter m: ");
    io::stdout().flush().expect("Error flushing STDOUT!");
    io::stdin().read_line(&mut input).expect("Error reading input from STDIN!");
    let m: usize = input.trim().parse().expect("Error parsing input to valid integer");
    input.clear(); // Need to clear before taking another input

    println!("Start entering elements of nums1 below:");
    for i in 0..m {
        io::stdin().read_line(&mut input).expect("Error reading input from STDIN!");
        let x: i32 = input.trim().parse().expect("Error parsing input to valid integer");
        nums1.push(x);
        input.clear(); // Need to clear before taking another input
    }

    print!("Enter n: ");
    io::stdout().flush().expect("Error flushing STDOUT!");
    io::stdin().read_line(&mut input).expect("Error reading input from STDIN!");
    let n: usize = input.trim().parse().expect("Error parsing input to valid integer");
    input.clear(); // Need to clear before taking another input

    println!("Start entering elements of nums2 below:");
    for i in 0..n {
        io::stdin().read_line(&mut input).expect("Error reading input from STDIN!");
        let x: i32 = input.trim().parse().expect("Error parsing input to valid integer");
        nums2.push(x);
        nums1.push(0); // making extra space for merge in nums 1
        input.clear(); // Need to clear before taking another input
    }

    println!("nums1: {:?}", nums1);
    println!("nums2: {:?}", nums2);
    merge_sorted_arrays(m, n, &mut nums1, &nums2);
    println!("nums1 after merge: {:?}", nums1);

}

// Tests are below
#[cfg(test)]

#[test]
fn test_merge_sorted_arrays_1() {
    let mut m = 3;
    let mut n = 3;
    let mut nums1 = vec![1,2,3,0,0,0];
    let mut nums2 = vec![2,5,6];
    merge_sorted_arrays(m, n, &mut nums1, &nums2);
    assert_eq!(nums1, vec![1,2,2,3,5,6]);
}

#[test]
fn test_merge_sorted_arrays_2() {
    let mut m = 1;
    let mut n = 3;
    let mut nums1 = vec![9,0,0,0];
    let mut nums2 = vec![2,5,6];
    merge_sorted_arrays(m, n, &mut nums1, &nums2);
    assert_eq!(nums1, vec![2,5,6,9]);
}

#[test]
fn test_merge_sorted_arrays_3() {
    let mut m = 3;
    let mut n = 1;
    let mut nums1 = vec![2,3,4,0];
    let mut nums2 = vec![1];
    merge_sorted_arrays(m, n, &mut nums1, &nums2);
    assert_eq!(nums1, vec![1,2,3,4]);
}