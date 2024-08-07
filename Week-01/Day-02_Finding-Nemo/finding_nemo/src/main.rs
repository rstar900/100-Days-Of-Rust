use std::io;

// Function to extract the location of word "Nemo"
fn find_nemo(msg: &mut String) {
    // Split into separate words with indices
    for (pos, word) in msg.split(" ").enumerate() {

        // Trim to get rid of any additional new line or hidden characters along with the word
        match word.trim() { 
            "Nemo" => { println!("I found Nemo at {}!", pos + 1); // pos + 1, as indices start from 0 
                        return;
                    },   
            _ => continue
        }
    }

    // None of the words match "Nemo"    
    println!("I can't find Nemo :(");
}

fn main() {
    loop {
        let mut message = String::new();
        println!("\nEnter the sentence below:");

        // Input logic
        io::stdin().read_line(&mut message).expect("Error reading line from STDIN!");
        find_nemo(&mut message);
    }
}
