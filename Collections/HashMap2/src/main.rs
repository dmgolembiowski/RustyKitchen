// Creating a hashmap from a list 
// of teams and a list of scores
use std::collections::HashMap;

/* Note, HashMaps must have keys of one shared type
 * and values of one shared type.
 */

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> =
        teams.iter()
            .zip(initial_scores.iter())
            .collect();
    for (k,v) in &scores {
        println!("{}:{}", k, v);
    }
}
