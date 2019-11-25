/*
    What is required to loop over all key-value pairs in a HashMap, such that...
    the programmer needs to check whether condition(s) is/are present to call another
    function?
 */
use std::collections::HashMap;
pub struct Dictionary {
    // "Once you start storing references in objects, lifetimes start mattering" -kibwen
    data: HashMap<String, i32>
}

impl Dictionary {

    fn new() -> Dictionary {
        Dictionary {
            data: HashMap::new(),
        }
    }
    
    fn keep_highest_scores(&mut self) {
        let mut score_threshold = 30 as i32;
        
        &self.data.retain(|_user_tag, score| {
            // Some criterion for deleting K,V pairs
            match score >= &mut(&mut score_threshold){
                    true   => true,
                    false  => false,
            }
        });
    }
    fn print_updated(&mut self) {
        // Print only the highest values
        for (user_tag, score) in &mut self.data {
            println!("{:?} -------------------------------------------------------- {:?} points", &user_tag, &score);
        }
    }
}

fn main() {
    // Create the dictionary
    let mut namco_hall_of_fame: Dictionary = Dictionary::new();

    // Populate the dictionary with records
    namco_hall_of_fame.data.insert(String::from("WIZARD12"), 27);
    namco_hall_of_fame.data.insert(String::from("WIZAR12"), 99);
    namco_hall_of_fame.data.insert(String::from("WIARD12"), 12);
    namco_hall_of_fame.data.insert(String::from("WIRD12-2019"), 234);
    namco_hall_of_fame.data.insert(String::from("WIZARD12"), 24);
    namco_hall_of_fame.data.insert(String::from("WIZARD12-2019"), 102);
    namco_hall_of_fame.data.insert(String::from("IZARD12"), 11);
    namco_hall_of_fame.data.insert(String::from("WD12"), 81);
    namco_hall_of_fame.data.insert(String::from("WID12"), 33);
    namco_hall_of_fame.data.insert(String::from("WIZ2"), 27);
    namco_hall_of_fame.data.insert(String::from("W2"), 99);

    // Trim away the low-scoring records
    &namco_hall_of_fame.keep_highest_scores();

    // Print the highest of scores
    &namco_hall_of_fame.print_updated();
}
