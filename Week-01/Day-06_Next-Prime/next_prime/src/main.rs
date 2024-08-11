use std::io::{self, Write};

fn next_prime(n: u32) -> u32 {

    let mut prime: u32 = 2;
    
    // Base case 
    if n >= 0 && n <= 2 {
        return prime;

    } else {
        // We will use Sieve of Eratosthenes for finding primes upto n
        let mut is_prime: Vec<bool> = vec![true; (n + 1) as usize]; // Keep track of primes and non-primes

        let mut i: u32 = 2;
        
        // As long as the sqaures are below n
        while i * i <= n { 
            if is_prime[i as usize] {
                for j in (i * i..=n).step_by(i.try_into().unwrap()) { // mark all multiples of i´(prime) as false
                    is_prime[j as usize] = false;
                } 
            }
            i += 1;
        }

        // If n is prime, simply return it, else find the next number not divisible by any of the primes found so far
        if is_prime[n as usize] {
            return n; 

        } else {
            let mut k = n + 1;
            
            loop {
                let mut is_k_prime: bool = true; // we assume it to be true initially
                for l in 2..n { // we can ignore n here as it would not be a prime
                    if is_prime[l as usize] {
                        if k % l == 0 {
                            is_k_prime = false;
                            break;
                        } else {
                            continue;
                        }
                    }
                }

                // We need to check if the loop completed or we broke as it was not a prime
                if is_k_prime {
                    prime = k;
                    break;
                }

                k += 1;
            }
        }
    }

    prime
}

fn main() {
    let mut input: String = String::new();
    print!("Enter n: ");
    io::stdout().flush().expect("Error flushing STDOUT!");
    io::stdin().read_line(&mut input).expect("Error reading input from STDIN!");
    println!("The next prime: {}", next_prime(input.trim().parse::<u32>().expect("Cannot parse input into a valid integer!")));
}

// Below are the tests 
#[cfg(test)]

#[test]
fn test_next_prime() {
    assert_eq!(next_prime(12), 13);
    assert_eq!(next_prime(24), 29);
    assert_eq!(next_prime(11), 11);
    assert_eq!(next_prime(0), 2);
    assert_eq!(next_prime(1), 2);
    assert_eq!(next_prime(2), 2);
}
