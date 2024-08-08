use std::io::{self, Write};

fn get_num_skewers(skewers: &Vec<String>) -> (u8, u8) {
    
    let mut veg_skewers: u8 = 0;
    let mut nveg_skewers: u8 = 0;

    for skewer in skewers {
        // veg and non veg ingredients in a single skewer
        let mut veg: u8 = 0;
        let mut nveg: u8 = 0;

        for ingredient in skewer.chars() {
            match ingredient {
                'x' => nveg += 1,
                'o' => veg += 1,
                _ => continue
            }
        }

        // If there is atleast 1 non-veg ingredient, then it is another non-veg skewer
        if nveg > 0 {
            nveg_skewers += 1;

        } else if veg > 0 && nveg == 0 { // If all ingredients are veg, then it is another veg skewer
            veg_skewers += 1;
        }
    }

    (veg_skewers, nveg_skewers)
}

fn main() {
    let mut n: String = String::new(); // number of lines
    print!("Enter the number of skewers: "); 
    io::stdout().flush().expect("Error flushing stdout!"); // Since we are using print!() which does not do flushing on its own
    io::stdin().read_line(&mut n).expect("Error reading the number of skewers!");

    // take input for n skewers
    if let Ok(n) = n.trim().parse::<u8>() {
        let mut skewers: Vec<String> = vec![]; // List to store Skewers
        for _i in 1..=n {
            let mut buf: String = String::new();
            io::stdin().read_line(&mut buf).expect("Error reading skewers!");
            skewers.push(buf.trim().to_string());
        }
        
        // Print the number of veg and non-veg skewers
        let (veg_skewers, nveg_skewers) = get_num_skewers(&skewers);
        println!("Result => [{} Veg Skewers, {} Non Veg Skewers]", veg_skewers, nveg_skewers);
    }
}
